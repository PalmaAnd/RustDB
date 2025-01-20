# RustDB - A SQLite Clone in Rust

## Foreword

This project is a hobby implementation of a SQLite-like database engine written in Rust.#
It is **not** intended for production use and serves primarily as a learning exercise to understand how databases work under the hood.
The goal is to gain deep insights into database internals, SQL parsing, query optimization, and storage engine design.
I also want to use this project to learn Rust and understand the hype behind it.

I want to learn more about:
- Database file formats and storage management
- B-tree indexing and data structures
- SQL parsing and query execution
- Transaction management and ACID properties
- Buffer management and caching
- Concurrent access patterns

## Project Overview

RustDB is a simple SQL database engine that (hopefully at some time) implements a subset of SQLite's functionality. It supports basic SQL operations, persistent storage, and simple indexing.

### Planned Features
- Basic SQL parsing and execution
- CRUD operations (CREATE, READ, UPDATE, DELETE)
- Simple query optimizer
- B-tree based indexing
- Basic transaction support
- Simple buffer manager
- Command-line interface

### Thinks I don't aim to do
- Full SQL compliance
- Production-level performance
- Network protocol support
- Advanced features like triggers or views
- Full SQLite compatibility

## Development Guidelines I try to follow

1. Follow Rust best practices and idioms
2. Write comprehensive tests for each component
3. Keep performance in mind but prioritize correctness
4. Use error handling appropriately
5. Add logging for debugging

## Getting Started

1. Ensure you have Rust installed (cargo 1.84+ recommended)
2. Clone the repository
3. Run `cargo build` to build the project
4. Run `cargo test` to execute tests

## License

MIT License - feel free to use this for your own learning purposes.