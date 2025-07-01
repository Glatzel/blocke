# Blocke

[![codecov](https://codecov.io/gh/Glatzel/blocke/graph/badge.svg?token=NLTQSALvc9)](https://codecov.io/gh/Glatzel/blocke)

Blocke is a Rust library for parsing, dispatching, and handling structured messages from embedded systems—including but not limited to NMEA. It is designed for modularity, extensibility, and robust error handling, making it suitable for a wide range of embedded protocols and data streams.

## Features

- **Generic Message Parsing**: Supports NMEA and can be extended to other embedded protocols.
- **Multi-line Handling**: Buffers and assembles multi-line messages.
- **Strong Typing**: Uses Rust enums and structs for safe, clear data representation.
- **Extensible**: Easily add support for new message types and protocols.
- **Error Handling**: Uses `miette` for rich error reporting.
- **Test Coverage**: Includes unit tests and code coverage reporting.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
blocke = "0.1"
```

### Example

```rust
use blocke::dispatcher::Dispatcher;
use blocke::data::{Talker, Identifier};
use rax_parser::io::SomeRaxReader; // Replace with your actual reader

let mut reader = SomeRaxReader::new("embedded.log");
let mut dispatcher = Dispatcher::new(&mut reader);

for (talker, identifier, message) in dispatcher {
    println!("Talker: {:?}, Identifier: {:?}, Message: {}", talker, identifier, message);
}
```

## Crate Structure

- `crates/rax-nmea`: Core parsing logic and data types (NMEA and more).
- `src/data`: Definitions for each supported message type.
- `src/dispatcher.rs`: Message dispatching and multi-line handling.

## Development

- Clone the repo:
  `git clone https://github.com/Glatzel/blocke.git`
- Run tests:
  `cargo test`
- Check code coverage:
  `cargo tarpaulin --out Html`

## License

This project is licensed under the BSD-3-Clause License.

---

**Blocke** aims to make embedded message parsing in Rust easy, safe, and reliable. Contributions and feedback are welcome!
