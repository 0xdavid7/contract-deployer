#!/usr/bin/env bash
set -euo pipefail

# test-version-logic.sh
# Shared version calculation logic for CI, release, and local testing.
#
# Usage examples:
#   bash scripts/test-version-logic.sh --mode predict
#   bash scripts/test-version-logic.sh --mode release --bump-type minor
#
# Behavior:
# - Prefers latest semver Git tag (vX.Y.Z) as the source of truth.
# - Falls back to Cargo.toml version, else defaults to 0.0.0.
# - Computes bump type from commits since latest tag if not provided.
# - In release mode: updates Cargo.toml and ensures the new tag is unique.
# - Writes outputs to $GITHUB_OUTPUT if available; sets env via $GITHUB_ENV if provided.

MODE="predict"            # predict | release
BUMP_TYPE=""              # major | minor | patch | empty -> auto
SYNC_CARGO="auto"         # yes | no | auto (auto: yes for release, no for predict)
ENSURE_UNIQUE="auto"      # yes | no | auto (auto: yes for release, no for predict)
RANGE="since-tag"         # latest | since-tag | all

while [ $# -gt 0 ]; do
  case "$1" in
    --mode)
      MODE="${2:-predict}"; shift 2;;
    --bump-type)
      BUMP_TYPE="${2:-}"; shift 2;;
    --sync-cargo)
      SYNC_CARGO="${2:-auto}"; shift 2;;
    --ensure-unique)
      ENSURE_UNIQUE="${2:-auto}"; shift 2;;
    --range)
      RANGE="${2:-since-tag}"; shift 2;;
    *)
      echo "Unknown argument: $1" >&2; shift;;
  esac
done

# Resolve auto flags
if [ "$SYNC_CARGO" = "auto" ]; then
  SYNC_CARGO=$([ "$MODE" = "release" ] && echo yes || echo no)
fi
if [ "$ENSURE_UNIQUE" = "auto" ]; then
  ENSURE_UNIQUE=$([ "$MODE" = "release" ] && echo yes || echo no)
fi

# Ensure tags are available
git fetch --tags >/dev/null 2>&1 || true

# Detect current version
CARGO_VERSION=$(grep -E '^version = "[0-9]+\.[0-9]+\.[0-9]+"' Cargo.toml | sed 's/version = "\(.*\)"/\1/')
LATEST_TAG=$(git tag -l 'v[0-9]*.[0-9]*.[0-9]*' --sort=-v:refname | head -n 1)

if [ -n "${LATEST_TAG:-}" ]; then
  CURRENT_VERSION="${LATEST_TAG#v}"
  VERSION_SOURCE="tag"
  echo "Current version (from tag): $CURRENT_VERSION (tag: $LATEST_TAG)"
elif [ -n "${CARGO_VERSION:-}" ]; then
  CURRENT_VERSION="$CARGO_VERSION"
  VERSION_SOURCE="cargo"
  echo "Current version (from Cargo.toml): $CURRENT_VERSION"
else
  CURRENT_VERSION="0.0.0"
  VERSION_SOURCE="default"
  echo "No version found; defaulting to $CURRENT_VERSION"
fi

if [ -n "${LATEST_TAG:-}" ] && [ -n "${CARGO_VERSION:-}" ] && [ "${LATEST_TAG#v}" != "$CARGO_VERSION" ]; then
  echo "Warning: Version mismatch: Cargo.toml ($CARGO_VERSION) != latest tag (${LATEST_TAG#v}). Using tag." >&2
fi

# Parse components
IFS='.' read -r MAJOR MINOR PATCH <<< "$CURRENT_VERSION"
echo "Parsed version: MAJOR=$MAJOR, MINOR=$MINOR, PATCH=$PATCH"

# Determine bump type if not provided
if [ -z "$BUMP_TYPE" ] && [ "$MODE" != "read" ]; then
  echo "Determining version bump from commit messages ($RANGE)..."
  case "$RANGE" in
    latest)
      COMMIT_MESSAGES=$(git log -1 --pretty=format:"%s")
      ;;
    since-tag)
      if [ -n "${LATEST_TAG:-}" ]; then
        echo "Getting commits since latest tag: $LATEST_TAG"
        COMMIT_MESSAGES=$(git log --pretty=format:"%s" "$LATEST_TAG"..HEAD)
      else
        echo "No previous tags found, analyzing all commits"
        COMMIT_MESSAGES=$(git log --pretty=format:"%s")
      fi
      ;;
    all)
      COMMIT_MESSAGES=$(git log --pretty=format:"%s")
      ;;
    *)
      echo "Invalid range: $RANGE" >&2; exit 1;;
  esac
  echo "Recent commit messages:"
  echo "$COMMIT_MESSAGES"

  if [ -n "$COMMIT_MESSAGES" ]; then
    if echo "$COMMIT_MESSAGES" | grep -i "BREAKING CHANGE\|breaking:" > /dev/null; then
      BUMP_TYPE="major"
    elif echo "$COMMIT_MESSAGES" | grep -E "^feat(\(.+\))?:" > /dev/null; then
      BUMP_TYPE="minor"
    else
      BUMP_TYPE="patch"
    fi
  else
    echo "No commit messages found, defaulting to patch bump"
    BUMP_TYPE="patch"
  fi
fi

echo "Version bump type: $BUMP_TYPE"

# Modes:
if [ "$MODE" = "read" ]; then
  NEW_VERSION="$CURRENT_VERSION"
  BUMP_TYPE=""
else
  # Compute new version
  case "$BUMP_TYPE" in
    major)
      NEW_MAJOR=$((MAJOR + 1)); NEW_MINOR=0; NEW_PATCH=0;;
    minor)
      NEW_MAJOR=$MAJOR; NEW_MINOR=$((MINOR + 1)); NEW_PATCH=0;;
    patch)
      NEW_MAJOR=$MAJOR; NEW_MINOR=$MINOR; NEW_PATCH=$((PATCH + 1));;
    *)
      echo "Invalid bump type: $BUMP_TYPE" >&2; exit 1;;
  esac
  NEW_VERSION="$NEW_MAJOR.$NEW_MINOR.$NEW_PATCH"
fi

# Ensure uniqueness if requested
if [ "$ENSURE_UNIQUE" = "yes" ] && [ "$MODE" != "read" ]; then
  while git tag -l "v$NEW_VERSION" | grep -q .; do
    echo "Tag v$NEW_VERSION already exists; incrementing patch to avoid conflict"
    NEW_PATCH=$((NEW_PATCH + 1))
    NEW_VERSION="$NEW_MAJOR.$NEW_MINOR.$NEW_PATCH"
  done
fi

echo "Computed new version: $NEW_VERSION"

# Optionally update Cargo.toml
if [ "$SYNC_CARGO" = "yes" ] && [ "$MODE" != "read" ]; then
  echo "Updating Cargo.toml to version $NEW_VERSION"
  if [[ "$(uname)" == "Darwin" ]]; then
    sed -i '' "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
  else
    sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
  fi
  echo "Updated Cargo.toml version:"
  grep "^version = " Cargo.toml || true
fi

# Write outputs for GitHub Actions if available
if [ -n "${GITHUB_OUTPUT:-}" ]; then
  {
    echo "new_version=$NEW_VERSION"
    echo "bump_type=$BUMP_TYPE"
    echo "current_version=$CURRENT_VERSION"
    echo "version_source=$VERSION_SOURCE"
    echo "latest_tag=${LATEST_TAG:-}"
    echo "range=$RANGE"
  } >> "$GITHUB_OUTPUT"
fi

# Write env vars for subsequent steps
if [ -n "${GITHUB_ENV:-}" ]; then
  echo "NEW_VERSION=$NEW_VERSION" >> "$GITHUB_ENV"
  echo "BUMP_TYPE=$BUMP_TYPE" >> "$GITHUB_ENV"
fi

# Append step summary if available
if [ -n "${GITHUB_STEP_SUMMARY:-}" ]; then
  {
    echo "Predicted release version: v$NEW_VERSION"
    echo "- Current version ($VERSION_SOURCE): v$CURRENT_VERSION"
    [ -n "$BUMP_TYPE" ] && echo "- Bump type: $BUMP_TYPE" || true
    echo "- Latest tag: ${LATEST_TAG:-none}"
    echo "- Mode: $MODE"
    echo "- Cargo.toml updated: $([ "$SYNC_CARGO" = "yes" ] && echo yes || echo no)"
    echo "- Tag uniqueness enforced: $([ "$ENSURE_UNIQUE" = "yes" ] && echo yes || echo no)"
    echo "- Range: $RANGE"
  } >> "$GITHUB_STEP_SUMMARY"
fi

exit 0