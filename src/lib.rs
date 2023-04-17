#![forbid(unsafe_code)]

//! # serde_with_object_id
//!
//! This crate provide `DisplayFromObjectId` which aims to provide de/serialize of `mongodb::bson::oid::ObjectId` using `serde_with` and `serde`.
//!
//! ## Usage
//!
//! ```rust
//! use serde::{Deserialize, Serialize};
//! use serde_with::serde_as;
//! use serde_with_object_id::DisplayFromObjectId;
//!
//! #[serde_as]
//! #[derive(Deserialize, Serialize)]
//! pub struct SomeDocument {
//!     #[serde_as(as = "DisplayFromObjectId")]
//!     pub id: String,
//!     #[serde_as(as = "Option<DisplayFromObjectId>")]
//!     pub id_optional: Option<String>,
//! }
//! ```

use core::{fmt::Display, str::FromStr};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Deserializer, Serializer};
use serde_with::{DeserializeAs, SerializeAs};

pub struct DisplayFromObjectId;

impl<'de, T> DeserializeAs<'de, T> for DisplayFromObjectId
where
    T: FromStr,
    T::Err: Display,
{
    fn deserialize_as<D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
    {
        let object_id = ObjectId::deserialize(deserializer).map_err(serde::de::Error::custom)?;
        object_id.to_hex().parse().map_err(serde::de::Error::custom)
    }
}

impl<T> SerializeAs<T> for DisplayFromObjectId
where
    T: Display,
{
    fn serialize_as<S>(source: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&source)
    }
}
