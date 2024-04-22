//! Request types for the Blobscan API.

use serde::{ser::SerializeStruct, Deserialize, Serialize};

/// Additional query parameters for the `blocks` endpoint.
///
/// By default all fields are requested.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct GetBlockQuery {
    /// What kind of block to fetch.
    // TODO flatten this into fields
    pub expand: BlockExpansion,
    /// What kind of block to fetch.
    pub kind: BlockKind,
}

impl GetBlockQuery {
    /// Create a new request with the given expansion options.
    pub const fn new(expand: BlockExpansion) -> Self {
        Self { expand, kind: BlockKind::Canonical }
    }

    /// Request transaction, blob and blob data in the response.
    pub const fn with_all(self) -> Self {
        Self { expand: BlockExpansion::all(), ..self }
    }

    /// Exclude transactions,blob and blob_data in the response.
    pub const fn with_none(self) -> Self {
        Self { expand: BlockExpansion::none(), ..self }
    }

    /// Request a reorged block.
    pub const fn reorged(self) -> Self {
        Self { kind: BlockKind::Reorged, ..self }
    }

    /// Include the transactions in the response.
    pub const fn with_transaction(self) -> Self {
        Self { expand: self.expand.with_transaction(), ..self }
    }

    /// Exclude the transactions from the response.
    pub const fn no_transactions(self) -> Self {
        Self { expand: self.expand.no_transactions(), ..self }
    }

    /// Include the blob in the response.
    pub const fn with_blob(self) -> Self {
        Self { expand: self.expand.with_blob(), ..self }
    }

    /// Exclude the blob from the response.
    pub const fn no_blob(self) -> Self {
        Self { expand: self.expand.no_blob(), ..self }
    }

    /// Include the blob in the response.
    pub const fn with_blob_data(self) -> Self {
        Self { expand: self.expand.with_blob_data(), ..self }
    }

    /// Exclude the blob data from the response.
    pub const fn no_blob_data(self) -> Self {
        Self { expand: self.expand.no_blob_data(), ..self }
    }
}

impl Serialize for GetBlockQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("GetBlockQuery", 2)?;
        let expand = self.expand.query_string();
        if !expand.is_empty() {
            s.serialize_field("expand", &expand)?;
        }
        s.serialize_field("type", &self.kind)?;
        s.end()
    }
}

/// What kind of block to fetch.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BlockKind {
    /// Fetch the canonical block.
    #[default]
    #[serde(rename = "canonical")]
    Canonical,
    /// Fetch a reorged block.
    #[serde(rename = "reorged")]
    Reorged,
}

/// What to include in the block response.
///
/// By default all fields are requested.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BlockExpansion {
    /// Include the transaction in the response
    pub transaction: bool,
    /// Include the blob in the response
    pub blob: bool,
    /// Include the blob_data in the response
    pub blob_data: bool,
}

impl BlockExpansion {
    /// Build a `BlockExpansion` that includes everything.
    pub const fn all() -> Self {
        Self { transaction: true, blob: true, blob_data: true }
    }

    /// Build a `BlockExpansion` that excludes everything.
    pub const fn none() -> Self {
        Self { transaction: false, blob: false, blob_data: false }
    }

    /// Returns the query string for the block expansion.
    pub fn query_string(self) -> String {
        let Self { transaction, blob, blob_data } = self;
        transaction
            .then_some("transaction")
            .into_iter()
            .chain(blob.then_some("blob"))
            .chain(blob_data.then_some("blob_data"))
            .collect::<Vec<_>>()
            .join(",")
    }

    /// Exclude the blob data from the response
    pub const fn no_blob_data(self) -> Self {
        Self { blob_data: false, ..self }
    }

    /// Exclude the transactions data from the response
    pub const fn no_transactions(self) -> Self {
        Self { transaction: false, ..self }
    }

    /// Exclude the blob  from the response
    pub const fn no_blob(self) -> Self {
        Self { blob: false, ..self }
    }

    /// Include the transactions in the response.
    pub const fn with_transaction(self) -> Self {
        Self { transaction: true, ..self }
    }

    /// Include the blob in the response.
    pub const fn with_blob(self) -> Self {
        Self { blob: true, ..self }
    }

    /// Include the blob in the response.
    pub const fn with_blob_data(self) -> Self {
        Self { blob_data: true, ..self }
    }
}

impl Default for BlockExpansion {
    fn default() -> Self {
        Self::all()
    }
}

/// What to include in the transaction response.
///
/// By default all fields are requested.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GetTransactionQuery {
    /// Include the block in the response
    pub block: bool,
    /// Include the blob in the response
    pub blob: bool,
    /// Include the blob_data in the response
    pub blob_data: bool,
}

impl GetTransactionQuery {
    /// Build a `TransactionExpansion` that includes everything.
    pub const fn all() -> Self {
        Self { block: true, blob: true, blob_data: true }
    }

    /// Build a `TransactionExpansion` that excludes everything.
    pub const fn none() -> Self {
        Self { block: false, blob: false, blob_data: false }
    }

    /// Returns the query string for the transaction expansion.
    pub fn query_string(self) -> String {
        let Self { block, blob, blob_data } = self;
        block
            .then_some("block")
            .into_iter()
            .chain(blob.then_some("blob"))
            .chain(blob_data.then_some("blob_data"))
            .collect::<Vec<_>>()
            .join(",")
    }

    /// Exclude the blob data from the response
    pub const fn no_blob_data(self) -> Self {
        Self { blob_data: false, ..self }
    }

    /// Exclude the block data from the response
    pub const fn no_block(self) -> Self {
        Self { block: false, ..self }
    }

    /// Exclude the blob  from the response
    pub const fn no_blob(self) -> Self {
        Self { blob: false, ..self }
    }
}

impl Default for GetTransactionQuery {
    fn default() -> Self {
        Self::all()
    }
}

impl Serialize for GetTransactionQuery {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("GetTransactionQuery", 1)?;
        let expand = self.query_string();
        if !expand.is_empty() {
            s.serialize_field("expand", &expand)?;
        }
        s.end()
    }
}
