har-rs
======
[HTTP Archive format (HAR)][har] serializing & deserializing library, written purely in Rust.

Install
-------
Add the following to your `Cargo.toml` file:

```toml
[dependencies]
har = "0.1"
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
This project follows [semver], [conventional commits] and semantic releasing using [semantic-rs].

Note
----
Inspired by [softprops/openapi](https://github.com/softprops/openapi).

[har]: https://en.wikipedia.org/wiki/.har
[semver]: https://semver.org/
[conventional commits]: https://www.conventionalcommits.org
[semantic-rs]: https://github.com/semantic-rs/semantic-rs
