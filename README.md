# serde_with_object_id

This crate provide `DisplayFromObjectId` which aims to provide de/serialize of `mongodb::bson::oid::ObjectId` using `serde_with` and `serde`.


## Usage

```rust
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with_object_id::DisplayFromObjectId;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SomeDocument {
    #[serde_as(as = "DisplayFromObjectId")]
    #[serde(rename = "_id")]
    pub id: String,
    #[serde_as(as = "Option<DisplayFromObjectId>")]
    pub id_optional: Option<String>,
}
```


## Install

```bash
cargo add serde -F derive
cargo add mongodb -F bson-serde_with
cargo add serde_with_object_id
```


## Other links

- https://github.com/serde-rs/serde
- https://github.com/jonasbb/serde_with
- https://github.com/mongodb/mongo-rust-driver
- https://github.com/mongodb/bson-rust
