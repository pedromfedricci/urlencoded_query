# An URL query string builder for urlencoded serializable values

![MSRV][rustc-image]
![MIT Licensed][license-image]
![Safety][safety-image]
[![CI][ci-image]][ci-link]

## About

A simple URL query string constructor that is able to append values to the query
string as long as they can be serialized to *application/x-www-form-urlencoded* format.
Serialization is provided by the [serde_urlencoded]([https://github.com/nox/serde_urlencoded]) crate.
You can set [rust-url]([https://github.com/servo/rust-url])'s query string with provided method.

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
urlencoded_query = { version = "0.1", git = "https://github.com/pedromfedricci/urlencoded_query" }
```

## Example

```rust
use serde::Serialize;
use url::Url;
use urlencoded_query::UrlEncodedQuery;

// Some struct definition that can be serialized to
// `application/x-www-form-urlencoded` format.
// Serialization will be perfomed by the `serde_urlencoded` crate.
// See https://docs.rs/serde_urlencoded/latest/serde_urlencoded/ for more details.
#[derive(Serialize)]
struct SearchParameter1 {
    pub q: &'static str,
}

#[derive(Serialize)]
struct SearchParameter2 {
    #[serde(rename(serialize = "type"))]
    pub t: &'static str,
    #[serde(rename(serialize = "ref"))]
    pub r: &'static str,
}

fn main() -> Result<(), serde_urlencoded::ser::Error> {
    let mut url = Url::parse("https://github.com/search").unwrap();

    // Create a new URL encoded query string from serializable value.
    // Fails if cannot serialize to `application/x-www-form-urlencoded` format.
    let mut query = UrlEncodedQuery::try_new(&SearchParameter1 {
        // Space will be encoded to `+` and `:` to `%3A`.
        q: "cargo user:rust-lang",
    })?;

    // Append a new serializable value.
    // Fails if cannot serialize to `application/x-www-form-urlencoded` format.
    query.try_append(&SearchParameter2 { t: "Repositories", r: "advsearch" })?;

    // Set URL.
    query.set_url(&mut url);

    assert_eq!(
        "https://github.com/search?q=cargo+user%3Arust-lang&type=Repositories&ref=advsearch",
        url.as_str()
    );

    Ok(())
}

```

## Documentation

Currently this project is not publish at [crates.io]([https://crates.io]),
so the documentation is not hosted at [docs.rs]([https://docs.rs]).
If you want to render the docs, please clone the project and run cargo doc --open.

## Code review

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)
to verify the trustworthiness of each of your dependencies, including this one.

## License

Licensed under [MIT license].

[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.59+-blue.svg
[safety-image]: https://img.shields.io/badge/unsafe-forbidden-success.svg
[license-image]: https://img.shields.io/badge/license-MIT-blue.svg
[ci-image]: https://github.com/pedromfedricci/urlencoded_query/actions/workflows/ci.yaml/badge.svg
[ci-link]: https://github.com/pedromfedricci/newsletter/urlencoded_query/workflows/ci.yaml

[//]: # (general links)

[MIT license]: https://github.com/pedromfedricci/urlencoded_query/LICENSE
