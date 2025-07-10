# Foundry Deploy


```null
contract-deployer/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs           # Entry point, orchestrates the deployment
│   ├── lib.rs            # Library root, exports public modules
│   ├── cli.rs            # Command line interface definitions
│   ├── config.rs         # Configuration parsing and management
│   ├── environment.rs    # Environment variable handling
│   └── deployer.rs       # Core deployment logic
├── examples/
│   ├── basic-deploy.toml
│   ├── multi-chain.toml
│   └── development.toml
└── tests/
    ├── integration_tests.rs
    └── fixtures/
        └── test_config.toml
```


