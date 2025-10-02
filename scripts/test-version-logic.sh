#!/bin/bash

# Test script to validate the version bump logic
set -e

echo "🧪 Testing version bump logic..."
echo ""

# Get current version from Cargo.toml
CURRENT_VERSION=$(grep "^version = " Cargo.toml | sed 's/version = "\(.*\)"/\1/')
echo "📦 Current version: $CURRENT_VERSION"

# Parse version components
MAJOR=$(echo $CURRENT_VERSION | cut -d. -f1)
MINOR=$(echo $CURRENT_VERSION | cut -d. -f2)
PATCH=$(echo $CURRENT_VERSION | cut -d. -f3)
echo "🔍 Parsed version: MAJOR=$MAJOR, MINOR=$MINOR, PATCH=$PATCH"
echo ""

# Get commit messages since last tag
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "")
if [ -n "$LAST_TAG" ]; then
  echo "🏷️  Getting commits since last tag: $LAST_TAG"
  COMMIT_MESSAGES=$(git log --pretty=format:"%s" "$LAST_TAG"..HEAD)
else
  echo "🏷️  No previous tags found, analyzing all commits"
  COMMIT_MESSAGES=$(git log --pretty=format:"%s")
fi

echo "📝 Recent commit messages:"
echo "$COMMIT_MESSAGES"
echo ""

# Determine bump type
if [ -n "$COMMIT_MESSAGES" ]; then
  if echo "$COMMIT_MESSAGES" | grep -i "BREAKING CHANGE\|breaking:" > /dev/null; then
    BUMP_TYPE="major"
    echo "💥 Found breaking changes - MAJOR bump"
  elif echo "$COMMIT_MESSAGES" | grep -E "^feat(\(.+\))?:" > /dev/null; then
    BUMP_TYPE="minor"
    echo "✨ Found feature commits - MINOR bump"
  else
    BUMP_TYPE="patch"
    echo "🔧 Default - PATCH bump"
  fi
else
  echo "❓ No commit messages found, defaulting to PATCH bump"
  BUMP_TYPE="patch"
fi

echo ""
echo "📈 Version bump type: $BUMP_TYPE"

# Calculate new version
case $BUMP_TYPE in
  major)
    NEW_MAJOR=$((MAJOR + 1))
    NEW_MINOR=0
    NEW_PATCH=0
    ;;
  minor)
    NEW_MAJOR=$MAJOR
    NEW_MINOR=$((MINOR + 1))
    NEW_PATCH=0
    ;;
  patch)
    NEW_MAJOR=$MAJOR
    NEW_MINOR=$MINOR
    NEW_PATCH=$((PATCH + 1))
    ;;
esac

NEW_VERSION="$NEW_MAJOR.$NEW_MINOR.$NEW_PATCH"
echo "🎯 New version would be: $NEW_VERSION"
echo ""
echo "✅ Version logic test completed successfully!"