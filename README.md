<div align="center">
  <img src="images/header.png" alt="Skald" width="250" height="250">
  <p align="center">
    <br>
    Journaling in Rust
    <br/>
  </p>
</div>


**Skald** (will be) an encrypted digital journal written in Rust. Each entry will be timestamped, encrypted, and immutable after save.

## Project Overview

Skald is a learning project where the primary focus is to build a secure, file-based journal application using Rust. Initially a CLI application, but perhaps eventually a GUI app.

## Goals

1. **Learn Rust**: The primary goal of this project is to deepen understanding of Rust, particularly with regard to file handling, error management, and secure systems programming.
2. **Security**: Each entry should be encrypted before it's saved.
3. **Integrity**: Implement anti-tampering and immutability.
4. **Cross-Platform GUI**: Build a GUI using Tauri, with some nice UI framework.
5. **Learn Systems Programming**: Might over-engineer some areas to make this project a gateway into low-level programming.

## Tools

- **Rust**: For building the backend (CLI, encryption, file handling).
- **Tauri**: For the GUI desktop app (future goal).
- **sha2**: Cryptographic hashing, immutability.
- etc

## Installation

_Installation instructions will be added._

For now, one can clone the repository and build the project with Cargo:

```bash
git clone https://github.com/bjornthiberg/skald.git

cd skald

cargo build
```
