# qrypto

<!-- ![ci](https://img.shields.io/github/actions/workflow/status/[your-username]/qrypto/ci.yml?branch=main)
![crates.io](https://img.shields.io/crates/v/qrypto.svg)
[![documentation](https://img.shields.io/badge/docs-qrypto-blue?logo=rust)](https://docs.rs/qrypto/latest/) -->

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

## License

The MIT License.
