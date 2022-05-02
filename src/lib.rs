#![forbid(unsafe_code)]

use std::borrow::Borrow;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

use form_urlencoded::Serializer as UrlEncoder;
use serde::Serialize;
use url::Url;

/// A struct that represents a URL encoded query string.
pub struct UrlEncodedQuery {
    urlencoder: UrlEncoder<'static, String>,
}

impl UrlEncodedQuery {
    /// Creates a new, empty URL encoded query string.
    pub fn new() -> UrlEncodedQuery {
        UrlEncodedQuery::default()
    }

    /// Creates a new URL encoded query string from a initial, serializable value.
    ///
    /// Fails if cannot serialize to `application/x-www-form-urlencoded` format.
    pub fn try_new(
        value: &impl Serialize,
    ) -> Result<UrlEncodedQuery, serde_urlencoded::ser::Error> {
        let mut query = Self::default();
        value.serialize(query.serializer())?;
        Ok(query)
    }

    /// Sets an [`Url`] query string with the produced URL encoded [`String`].
    pub fn set_url(mut self, url: &mut Url) {
        // Can't call any mutable function from inner `form_urlenconded:Serializer`
        // after the `finish` call, or else will panic.
        // `set_url` consumes `self` in order to prevent any further calls.
        url.set_query(Some(&self.urlencoder.finish()))
    }

    /// Serilize and append a serializable value.
    /// Fails if cannot serialize to `application/x-www-form-urlencoded` format.
    pub fn try_append(
        &mut self,
        value: &impl Serialize,
    ) -> Result<&mut UrlEncodedQuery, serde_urlencoded::ser::Error> {
        value.serialize(self.serializer())?;
        Ok(self)
    }

    /// Serialize and append a name/value pair.
    pub fn append_pair(&mut self, name: &str, value: &str) -> &mut UrlEncodedQuery {
        self.urlencoder.append_pair(name, value);
        self
    }

    /// Serialize and append the name of a parameter without any value.
    pub fn append_key_only(&mut self, name: &str) -> &mut UrlEncodedQuery {
        self.urlencoder.append_key_only(name);
        self
    }

    /// Serialize and append a number of values to `application/x-www-form-urlencoded` format.
    ///
    /// Fails if one or more values could not be serialized, giving
    /// back a container with all values that were not appended after completing the iteration.
    pub fn try_extend<I>(
        &mut self,
        iter: I,
    ) -> Result<&mut UrlEncodedQuery, TryExtendError<I::Item>>
    where
        I: IntoIterator,
        I::Item: Serialize,
    {
        let mut unserialized = vec![];
        for value in iter {
            if let Err(error) = self.try_append(&value) {
                unserialized.push(Unserialized { value, error });
            }
        }

        if unserialized.is_empty() {
            Ok(self)
        } else {
            Err(TryExtendError { unserialized })
        }
    }

    /// Serialize and append a number of name/value pairs.
    /// This simply calls `append_pair` repeatedly.
    pub fn extend_pairs<I, K, V>(&mut self, iter: I) -> &mut UrlEncodedQuery
    where
        I: IntoIterator,
        I::Item: Borrow<(K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        self.urlencoder.extend_pairs(iter);
        self
    }

    /// Serialize and append a number of names without values.
    /// This simply calls `append_key_only` repeatedly.
    pub fn extend_keys_only<I, K>(&mut self, iter: I) -> &mut UrlEncodedQuery
    where
        I: IntoIterator,
        I::Item: Borrow<K>,
        K: AsRef<str>,
    {
        self.urlencoder.extend_keys_only(iter);
        self
    }

    /// Removes any existing name/value pair.
    pub fn clear(&mut self) -> &mut UrlEncodedQuery {
        self.urlencoder.clear();
        self
    }

    /// Creates a [`serde_urlencoded::Serializer`] from the inner [`UrlEncoder`].
    #[inline]
    fn serializer<'a>(&'a mut self) -> serde_urlencoded::Serializer<'static, 'a, String> {
        serde_urlencoded::Serializer::new(&mut self.urlencoder)
    }
}

impl Default for UrlEncodedQuery {
    /// Create a new, empty URL encoded query string.
    fn default() -> Self {
        Self { urlencoder: UrlEncoder::new(String::new()) }
    }
}

impl From<UrlEncodedQuery> for String {
    fn from(mut query: UrlEncodedQuery) -> String {
        query.urlencoder.finish()
    }
}

/// A struct that holds a value that could not be serialized to
/// `application/x-www-form-urlencoded` format
/// and their corresponding [`Error`].
///
/// [`Error`]: serde_urlencoded::ser::Error
#[derive(Debug)]
pub struct Unserialized<T> {
    pub value: T,
    pub error: serde_urlencoded::ser::Error,
}

/// An [`Error`] that holds values that could not be serialized during the `try_extend` operation.
///
/// [`Error`]: std::error::Error
#[derive(Debug)]
pub struct TryExtendError<T> {
    unserialized: Vec<Unserialized<T>>,
}

impl<T> TryExtendError<T> {
    /// Returns a slice of values that could not be serialized during the `try_extend` operation.
    pub fn unserialized(&self) -> &[Unserialized<T>] {
        &self.unserialized
    }
}

impl<T> From<TryExtendError<T>> for Vec<Unserialized<T>> {
    fn from(error: TryExtendError<T>) -> Self {
        error.unserialized
    }
}

impl<T> Display for TryExtendError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "could not serialize one or more values while extending the query string")
    }
}

impl<T: Debug> StdError for TryExtendError<T> {}
