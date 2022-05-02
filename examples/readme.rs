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
