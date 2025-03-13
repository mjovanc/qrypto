# qrypto

![ci](https://img.shields.io/github/actions/workflow/status/mjovanc/qrypto/ci.yml?branch=master)
![crates.io](https://img.shields.io/crates/v/qrypto.svg)
[![documentation](https://img.shields.io/badge/docs-qrypto-blue?logo=rust)](https://docs.rs/qrypto/latest/)

A simple and fast post-quantum cryptography library in Rust. Currently experimental, use it with caution!

## Motivation

`qrypto` is a post-quantum cryptography library built to bring quantum-safe security to Rust developers with an easy-to-use API. As quantum computing advances, it threatens to break classical encryption (like RSA and ECC) via algorithms like Shor’s, making post-quantum solutions critical for future-proofing applications. Existing PQC libraries in Rust, such as `pqcrypto`, are powerful but low-level and complex, leaving a gap for a straightforward, practical tool. `qrypto` fills this by offering a high-level interface built on NIST-standard algorithms like Kyber and Dilithium, designed for real-world tasks—think secure messaging, file encryption, or API authentication—without the crypto PhD.

Rust’s rise in systems programming, with its focus on safety and speed, makes it a perfect fit for PQC, yet accessible options are scarce. `qrypto` aims to change that, starting with simple encryption and signing primitives that anyone can use. Why PQC now? Quantum threats might be years away, but systems (e.g., IoT, blockchain, HTTPS) need to transition early—`qrypto` provides a lightweight, hybrid-ready bridge to that future. Our goal is a library that scales from hobbyists to enterprise, keeping security simple and robust as quantum computers loom.

## Get Started

This guide reflects the current MVP and may evolve. Check unit tests and the `examples` directory for the latest usage. APIs might shift as `qrypto` grows.

Add `qrypto` to your project:

```toml
[dependencies]
qrypto = "0.1.0"
```

### Basic Example

Here’s a quick example of encrypting and decrypting a message with Kyber:

```rust
use qrypto::{algorithms::Kyber512, decapsulate, encapsulate, generate_keypair};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Alice generates a Kyber512 key pair
    let keypair = generate_keypair::<Kyber512>()?;

    // Bob encapsulates a shared secret using Alice's public key
    let (ciphertext, shared_secret_bob) = encapsulate::<Kyber512>(keypair.public_key())?;

    // Alice decapsulates the ciphertext to get the same shared secret
    let shared_secret_alice = decapsulate::<Kyber512>(&keypair.secret_key(), &ciphertext)?;

    // Verify they match (in a real impl, they will)
    assert_eq!(shared_secret_alice, shared_secret_bob);
    println!("Shared secret (Kyber512): {:?}", shared_secret_alice);

    Ok(())
}
```

## Supported Features

Below is a table of currently supported features and planned additions for `qrypto`, designed to make it a fully featured post-quantum cryptography library.

| Feature                    | Description                                              | Status  | Notes                                                                                   |
|----------------------------|----------------------------------------------------------|---------|-----------------------------------------------------------------------------------------|
| **Kyber (KEM)**            | Key Encapsulation Mechanism (NIST-standard)              | Planned | Supports Kyber512, Kyber768, Kyber1024 for varying security levels (FIPS 203).          |
| **Dilithium (Signatures)** | Digital signature scheme (NIST-standard)                 | Planned | Includes Dilithium2, Dilithium3, Dilithium5 variants (FIPS 204).                        |
| **SPHINCS+ (Signatures)**  | Stateless hash-based signature scheme (NIST-standard)    | Planned | Supports SPHINCS+-128s, 128f, 256s, 256f for stateless signing (FIPS 205).              |
| **HQC (KEM)**              | Code-based Key Encapsulation Mechanism (NIST-standard)   | Planned | Supports HQC-128, HQC-192, HQC-256; added to NIST standards March 11, 2025.             |
| **Hybrid Encryption**      | Combines PQC with classical algorithms                   | Planned | Will support AES-256-GCM or ChaCha20-Poly1305 for authenticated encryption.             |
| **Hybrid Public-Key Crypto** | Combines PQC with RSA/ECC for transitional use         | Planned | Enables dual KEMs or signatures (e.g., Kyber + RSA) for legacy compatibility.           |
| **Authenticated Encryption (AEAD)** | PQC-based symmetric encryption with authentication | Planned | Integrates KEM-shared secrets with AES-GCM/ChaCha20-Poly1305 for secure data protection.|
| **Key Derivation**         | Derives symmetric keys from KEM outputs                  | Planned | Uses HKDF with SHAKE256 for secure key stretching and derivation.                       |
| **Key Generation**         | Secure keypair generation                                | Planned | Uses cryptographically secure RNGs via `rand` crate for all algorithms.                 |
| **Key Serialization**      | Key and ciphertext serialization/deserialization         | Planned | Supports PEM, DER, and binary formats for keys and ciphertexts.                         |
| **Key Wrapping**           | Encrypts private keys with passphrase protection         | Planned | Uses PBKDF2 or Argon2 to secure stored keys with user-provided passphrases.             |
| **Message Authentication** | Provides MACs using PQC-derived keys                     | Planned | Implements HMAC-SHAKE256 or Poly1305 for lightweight authentication.                    |
| **Batch Signing/Verification** | Efficiently processes multiple signatures           | Planned | Allows signing/verifying multiple messages in one call for performance.                 |
| **File Encryption/Decryption** | High-level API for encrypting/decrypting files       | Planned | Simplifies file security (e.g., `encrypt_file(pubkey, "file.txt")`).                    |
| **Secure Messaging**       | End-to-end encrypted message exchange                    | Planned | Provides a simple API for messaging (e.g., `secure_message(pubkey, "hello")`).          |
| **Certificate Signing**    | Generates/verifies PQC signatures for certificates       | Planned | Supports X.509-like certificates with Dilithium or SPHINCS+ signatures.                 |
| **Key Agreement Protocol** | Full key exchange protocol using KEMs                    | Planned | High-level API for initiator/responder key exchange (e.g., Kyber or HQC).               |
| **Standard Format Support**| Compatibility with OpenPGP, CMS, JOSE (JWK/JWE)          | Planned | Enables interoperability with existing crypto ecosystems.                               |
| **FIPS Compliance**        | Adherence to FIPS 140-3 standards                        | Planned | Targets validation for enterprise use cases across all algorithms.                      |
| **Side-Channel Resistance** | Protection against timing/power attacks                | Planned | Implements constant-time operations for all cryptographic primitives.                   |
| **Benchmarking**           | Performance testing suite                                | Planned | Compares against classical crypto and other PQC libraries (e.g., via `criterion`).      |
| **WASM Support**           | WebAssembly compatibility                                | Planned | For browser-based applications and lightweight deployments; JavaScript-friendly APIs.   |
| **no_std Support**         | Compatibility with no_std environments                   | Planned | Enables use in embedded systems without Rust standard library dependencies.             |
| **Async Support**          | Integration with async runtimes (e.g., tokio)            | Planned | Supports asynchronous operations for networking and server applications.                |
| **CLI Interface**          | Command-line tool for quick PQC tasks                    | Planned | Allows users to encrypt, sign, or generate keys via CLI (e.g., `qrypto encrypt ...`).   |

## Contributing

Before contributing, please read the [contribution](https://github.com/mjovanc/qrypto/blob/master/CONTRIBUTING.md) guide for useful information how to get started with `qrypto` as well as what should be included when submitting a contribution to the project.

## License

The MIT License.
