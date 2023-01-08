har-rs
======
[HTTP Archive format (HAR)][har] serialization & deserialization library, written in Rust.

<!---![Build Status](https://github.com/mandrean/har-rs/workflows/CI/badge.svg?branch=master)-->
[![Latest version](https://img.shields.io/crates/v/har.svg)](https://crates.io/crates/har)
[![Documentation](https://docs.rs/har/badge.svg)](https://docs.rs/har)
![License](https://img.shields.io/crates/l/har.svg)

Install
-------
Add the following to your `Cargo.toml` file:

```toml
[dependencies]
har = "0.8"
```

Use
---
Simplest possible example:
```rust
use har::from_path;

fn main() {
  match har::from_path("path/to/file.har") {
    Ok(spec) => println!("spec: {:?}", spec),
    Err(err) => println!("error: {}", err)
  }
}
```

See [docs.rs/har] for the full library API documentation.

Contribute
----------
This project follows [semver], [conventional commits] and semantic releasing using [mandrean/semantic-rs].

Note
----
Inspired by [softprops/openapi](https://github.com/softprops/openapi).

[conventional commits]: https://www.conventionalcommits.org
[docs.rs/har]: https://docs.rs/har
[har]: https://en.wikipedia.org/wiki/.har
[mandrean/semantic-rs]: https://github.com/mandrean/semantic-rs
[semver]: https://semver.org/
