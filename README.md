# Dilok DID - Decentralized Identity System

A Rust-based Decentralized Identity (DID) system with face recognition capabilities.

## Features

- Face recognition-based identity verification
- On-chain and off-chain storage using RocksDB
- RESTful API using Axum framework
- Secure storage of face data with encryption
- DID standard compliance

## Prerequisites

- Rust 1.70 or higher
- RocksDB
- LZ4 library (for RocksDB)

### macOS

```bash
brew install rocksdb lz4
```

### Linux (Ubuntu/Debian)

```bash
sudo apt-get install librocksdb-dev liblz4-dev
```

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/dilok-did.git
cd dilok-did
```

2. Build the project:
```bash
cargo build
```

## Usage

1. Configure the storage paths in your configuration:

```rust
let config = StorageConfig {
    onchain_storage_path: "path/to/onchain/storage",
    offchain_storage_path: "path/to/offchain/storage",
    encryption_key: "your-encryption-key",
};
```

2. Initialize the DID system:

```rust
let did_system = DilokDid::new(config)?;
```

3. Register a user:

```rust
let user_info = UserInfo {
    // ... user details ...
};
did_system.register_user(user_info, camera_index).await?;
```

4. Verify a user:

```rust
let is_verified = did_system.verify_user(user_id, camera_index).await?;
```

## API Endpoints

- `POST /register` - Register a new user
- `POST /verify/:user_id` - Verify a user's identity
- `GET /user/:user_id` - Get user information

## Testing

Run the test suite:

```bash
cargo test
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
