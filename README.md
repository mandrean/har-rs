har-rs
======
[HTTP Archive format (HAR)][har] serialization & deserialization library, written in Rust.

[![Build Status](https://travis-ci.org/mandrean/har-rs.svg?branch=master)](https://travis-ci.org/mandrean/har-rs)
[![Latest version](https://img.shields.io/crates/v/har.svg)](https://crates.io/crates/har)
[![Documentation](https://docs.rs/har/badge.svg)](https://docs.rs/har)
![License](https://img.shields.io/crates/l/har.svg)

Install
-------
Add the following to your `Cargo.toml` file:

```toml
[dependencies]
har = "0.5"
```

Use
---
```rust
extern crate har;

fn main() {
  match har::from_path("path/to/file.har") {
    Ok(spec) => println!("spec: {:?}", spec),
    Err(err) => println!("error: {}", err)
  }
}
```

Contribute
----------
This project follows [semver], [conventional commits] and semantic releasing using [mandrean/semantic-rs].

Note
----
Inspired by [softprops/openapi](https://github.com/softprops/openapi).

[har]: https://en.wikipedia.org/wiki/.har
[semver]: https://semver.org/
[conventional commits]: https://www.conventionalcommits.org
[mandrean/semantic-rs]: https://github.com/mandrean/semantic-rs
