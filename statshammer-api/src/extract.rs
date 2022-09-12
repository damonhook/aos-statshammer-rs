//! Custom extractors based on some of the extractors present in [`axum::extract`].
//! This will need to be rewritten once 0.6 comes out (and provides a much nicer way of doing this).

mod json;
pub use json::Json;

mod query;
pub use query::Query;
