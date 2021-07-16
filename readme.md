# rs-ipfix

[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Crates.io Version](https://img.shields.io/crates/v/rsipfix.svg)](https://crates.io/crates/rsipfix)

This is a library to parse IPFIX/Netflow v10 (RFC7011) (fork from rs-ipfix).

Features :

- Support custom fields definitions
- Can parse variable size fields
- Minimal memory usage
- JSON output
- Concurrent parsing (with thread-safe state handling)

See `./tests` for usage.
