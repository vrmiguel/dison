//! `dison` is a tiny crate for zero-copy JSON Display implementation for any type that implements `Serialize`.
//!
//! ```rust
//! use dison::Json;
//!
//! let arr = [[1, 2], [20, 30], [40, 50]];
//! let json = Json(arr);
//! println!("{json}");
//! ```
//!
//! This crate uses `serde_json` internally and therefore shall always
//! match whatever `serde_json::to_string` produces, with the added benefit
//! of not having to allocate a temporary `String` for common use cases such as printing
//! a JSON-formatted string to `stdout` or within `format!`.
//!
//! ```rust
//! # use serde::Serialize;
//! # #[derive(Serialize)]
//! # struct Message;
//! # type Result<T = ()> = std::result::Result<T, ()>;
//! # macro_rules! query {
//! #     ($sql:expr, $param:expr) => { Ok(()) };
//! # }
//! use dison::Json;
//!
//! fn send_message(message: &Message) -> Result {
//!    query!("SELECT from send_message($1::jsonb)", Json(message))
//! }
//! ```

use serde::Serialize;

/// The `serde_json`-based display implementation for [Json]
/// and [JsonPretty].
mod display;

/// A wrapper over a type that serializes to JSON in its Display implementation.
///
/// ```rust
/// use dison::Json;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("hey", "there");
///
/// let json = Json(&map);
///
/// // Prints `{"hey":"there"}`
/// println!("{json}");
/// ```
pub struct Json<T: Serialize>(pub T);

/// A wrapper over a type that serializes to "pretty" JSON in its Display implementation.
///
/// ```rust
/// use dison::Json;
/// use std::collections::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("hey", "there");
///
/// let json = Json(&map);
///
/// // Prints:
/// //  {
/// //    "hey": "there"
/// //  }
/// println!("{json}");
/// ```
pub struct JsonPretty<T: Serialize>(pub T);

impl<T: Serialize> Json<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Serialize> JsonPretty<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}
