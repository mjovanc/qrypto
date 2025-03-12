# qrypto

![ci](https://img.shields.io/github/actions/workflow/status/mjovanc/qrypto/ci.yml?branch=master)
![crates.io](https://img.shields.io/crates/v/qrypto.svg)
[![documentation](https://img.shields.io/badge/docs-qrypto-blue?logo=rust)](https://docs.rs/qrypto/latest/)

A high-performance post-quantum cryptography library in Rust.

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
fn main() -> Result<(), QryptoError> {
    let (public_key, secret_key) = Kyber::keypair()?;
    let plaintext = b"Hello, quantum-safe world!";
    let (ciphertext, shared_secret_enc) = Kyber::encrypt(&public_key, plaintext)?;
    let shared_secret_dec = Kyber::decrypt(&secret_key, &ciphertext)?;
    assert_eq!(shared_secret_enc, shared_secret_dec);
    Ok(())
}
```

## Supported Features

Below is a table of currently supported features and planned additions.

| Feature                  | Description                                      | Status  | Notes                                                                 |
|--------------------------|--------------------------------------------------|---------|----------------------------------------------------------------------|
| **Kyber (KEM)**          | Key Encapsulation Mechanism (NIST-standard)      | Planned | Supports Kyber512, Kyber768, Kyber1024 for varying security levels.  |
| **Dilithium (Signatures)** | Digital signature scheme (NIST-standard)        | Planned | Includes Dilithium2, Dilithium3, Dilithium5 variants.                |
| **Hybrid Encryption**    | Combines PQC with classical algorithms           | Planned | Will support AES-256 integration for backward compatibility.         |
| **Key Generation**       | Secure keypair generation                        | Planned | Uses cryptographically secure RNGs via `rand` crate.                 |
| **Serialization**        | Key and ciphertext serialization/deserialization | Planned | Planned support for PEM, DER, and binary formats.                    |
| **FIPS Compliance**      | Adherence to FIPS 140-3 standards                | Planned | Targeting validation for enterprise use cases.                       |
| **Side-Channel Resistance** | Protection against timing/power attacks       | Planned | Implementing constant-time operations where applicable.              |
| **Benchmarking**         | Performance testing suite                        | Planned | To compare against classical crypto and other PQC libraries.         |
| **WASM Support**         | WebAssembly compatibility                        | Planned | For browser-based applications and lightweight deployments.          |

## Contributing

Before contributing, please read the [contribution](https://github.com/mjovanc/qrypto/blob/master/CONTRIBUTING.md) guide for useful information how to get started with `qrypto` as well as what should be included when submitting a contribution to the project.

## License

The MIT License.
