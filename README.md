har-rs
======

[HTTP Archive format (HAR)][har] serialization & deserialization library, written in Rust.

[![CI](https://github.com/mandrean/har-rs/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/mandrean/har-rs/actions/workflows/ci.yml)
[![Latest version](https://img.shields.io/crates/v/har.svg)](https://crates.io/crates/har)
[![Documentation](https://docs.rs/har/badge.svg)](https://docs.rs/har)
[![License](https://img.shields.io/crates/l/har.svg)](https://github.com/mandrean/har-rs/blob/master/LICENSE)

Install
-------

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
har = "0.8"
```

Use
---

HAR input is parsed from JSON. The crate can serialize parsed documents back to JSON or YAML.

```rust
use har::{from_str, to_json, HarVersion};

fn main() -> Result<(), har::Error> {
    let input = r#"{
        "log": {
            "version": "1.2",
            "creator": { "name": "example", "version": "1.0" },
            "entries": []
        }
    }"#;

    let har = from_str(input)?;
    assert_eq!(har.version(), HarVersion::V1_2);

    println!("{}", to_json(&har)?);
    Ok(())
}
```

See [docs.rs/har] for the full library API documentation.

Contribute
----------

This project follows [semver] and [conventional commits]. CI runs on GitHub Actions, and releases are prepared and published with [release-plz] on `master`.

See [CHANGELOG.md](CHANGELOG.md) for release history.

Note
----

Inspired by [softprops/openapi](https://github.com/softprops/openapi).

[conventional commits]: https://www.conventionalcommits.org
[docs.rs/har]: https://docs.rs/har
[har]: https://en.wikipedia.org/wiki/HTTP_Archive
[release-plz]: https://release-plz.dev
[semver]: https://semver.org/
