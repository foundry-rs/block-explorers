#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(std::borrow::Cow<'static, str>);
        impl std::error::Error for ConversionError {}
        impl std::fmt::Display for ConversionError {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> Result<(), std::fmt::Error> {
                std::fmt::Display::fmt(&self.0, f)
            }
        }
        impl std::fmt::Debug for ConversionError {
            fn fmt(
                &self,
                f: &mut std::fmt::Formatter<'_>,
            ) -> Result<(), std::fmt::Error> {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }
        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }
        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }
    ///BlobGetAllResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlobGetAllResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<BlobGetAllResponseIssuesItem>,
        pub message: String,
    }
    impl From<&BlobGetAllResponse> for BlobGetAllResponse {
        fn from(value: &BlobGetAllResponse) -> Self {
            value.clone()
        }
    }
    ///BlobGetAllResponseBlobsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blockNumber",
    ///    "commitment",
    ///    "dataStorageReferences",
    ///    "index",
    ///    "proof",
    ///    "size",
    ///    "txHash",
    ///    "versionedHash"
    ///  ],
    ///  "properties": {
    ///    "block": {
    ///      "type": "object",
    ///      "properties": {
    ///        "blobAsCalldataGasUsed": {
    ///          "type": "string"
    ///        },
    ///        "blobGasPrice": {
    ///          "type": "string"
    ///        },
    ///        "blobGasUsed": {
    ///          "type": "string"
    ///        },
    ///        "excessBlobGas": {
    ///          "type": "string"
    ///        },
    ///        "hash": {
    ///          "type": "string"
    ///        },
    ///        "slot": {
    ///          "type": "number",
    ///          "minimum": 0.0
    ///        },
    ///        "timestamp": {
    ///          "type": "string"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    "blockNumber": {
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    },
    ///    "commitment": {
    ///      "type": "string"
    ///    },
    ///    "data": {
    ///      "type": "string"
    ///    },
    ///    "dataStorageReferences": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "blobStorage",
    ///          "dataReference"
    ///        ],
    ///        "properties": {
    ///          "blobStorage": {
    ///            "type": "string",
    ///            "enum": [
    ///              "google",
    ///              "swarm",
    ///              "postgres",
    ///              "file_system"
    ///            ]
    ///          },
    ///          "dataReference": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "index": {
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    },
    ///    "proof": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "size": {
    ///      "type": "number"
    ///    },
    ///    "transaction": {
    ///      "type": "object",
    ///      "properties": {
    ///        "blobAsCalldataGasFee": {
    ///          "type": "string"
    ///        },
    ///        "blobAsCalldataGasUsed": {
    ///          "type": "string"
    ///        },
    ///        "blobGasBaseFee": {
    ///          "type": "string"
    ///        },
    ///        "blobGasMaxFee": {
    ///          "type": "string"
    ///        },
    ///        "blobGasUsed": {
    ///          "type": "string"
    ///        },
    ///        "from": {
    ///          "type": "string"
    ///        },
    ///        "maxFeePerBlobGas": {
    ///          "type": "string"
    ///        },
    ///        "rollup": {
    ///          "type": [
    ///            "string",
    ///            "null"
    ///          ]
    ///        },
    ///        "to": {
    ///          "type": "string"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    "txHash": {
    ///      "type": "string"
    ///    },
    ///    "versionedHash": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlobGetAllResponseBlobsItem {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub block: Option<BlobGetAllResponseBlobsItemBlock>,
        #[serde(rename = "blockNumber")]
        pub block_number: f64,
        pub commitment: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data: Option<String>,
        #[serde(rename = "dataStorageReferences")]
        pub data_storage_references: Vec<
            BlobGetAllResponseBlobsItemDataStorageReferencesItem,
        >,
        pub index: f64,
        pub proof: Option<String>,
        pub size: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub transaction: Option<BlobGetAllResponseBlobsItemTransaction>,
        #[serde(rename = "txHash")]
        pub tx_hash: String,
        #[serde(rename = "versionedHash")]
        pub versioned_hash: String,
    }
    impl From<&BlobGetAllResponseBlobsItem> for BlobGetAllResponseBlobsItem {
        fn from(value: &BlobGetAllResponseBlobsItem) -> Self {
            value.clone()
        }
    }
    ///BlobGetAllResponseBlobsItemBlock
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "blobAsCalldataGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "blobGasPrice": {
    ///      "type": "string"
    ///    },
    ///    "blobGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "excessBlobGas": {
    ///      "type": "string"
    ///    },
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "slot": {
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    },
    ///    "timestamp": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlobGetAllResponseBlobsItemBlock {
        #[serde(
            rename = "blobAsCalldataGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_as_calldata_gas_used: Option<String>,
        #[serde(
            rename = "blobGasPrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_price: Option<String>,
        #[serde(
            rename = "blobGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_used: Option<String>,
        #[serde(
            rename = "excessBlobGas",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub excess_blob_gas: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hash: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub slot: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<String>,
    }
    impl From<&BlobGetAllResponseBlobsItemBlock> for BlobGetAllResponseBlobsItemBlock {
        fn from(value: &BlobGetAllResponseBlobsItemBlock) -> Self {
            value.clone()
        }
    }
    ///BlobGetAllResponseBlobsItemDataStorageReferencesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blobStorage",
    ///    "dataReference"
    ///  ],
    ///  "properties": {
    ///    "blobStorage": {
    ///      "type": "string",
    ///      "enum": [
    ///        "google",
    ///        "swarm",
    ///        "postgres",
    ///        "file_system"
    ///      ]
    ///    },
    ///    "dataReference": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlobGetAllResponseBlobsItemDataStorageReferencesItem {
        #[serde(rename = "blobStorage")]
        pub blob_storage: BlobGetAllResponseBlobsItemDataStorageReferencesItemBlobStorage,
        #[serde(rename = "dataReference")]
        pub data_reference: String,
    }
    impl From<&BlobGetAllResponseBlobsItemDataStorageReferencesItem>
    for BlobGetAllResponseBlobsItemDataStorageReferencesItem {
        fn from(value: &BlobGetAllResponseBlobsItemDataStorageReferencesItem) -> Self {
            value.clone()
        }
    }
    ///BlobGetAllResponseBlobsItemDataStorageReferencesItemBlobStorage
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "google",
    ///    "swarm",
    ///    "postgres",
    ///    "file_system"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum BlobGetAllResponseBlobsItemDataStorageReferencesItemBlobStorage {
        #[serde(rename = "google")]
        Google,
        #[serde(rename = "swarm")]
        Swarm,
        #[serde(rename = "postgres")]
        Postgres,
        #[serde(rename = "file_system")]
        FileSystem,
    }
    impl From<&BlobGetAllResponseBlobsItemDataStorageReferencesItemBlobStorage>
    for BlobGetAllResponseBlobsItemDataStorageReferencesItemBlobStorage {
        fn from(
            value: &BlobGetAllResponseBlobsItemDataStorageReferencesItemBlobStorage,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for BlobGetAllResponseBlobsItemDataStorageReferencesItemBlobStorage {
        fn to_string(&self) -> String {
            match *self {
                Self::Google => "google".to_string(),
                Self::Swarm => "swarm".to_string(),
                Self::Postgres => "postgres".to_string(),
                Self::FileSystem => "file_system".to_string(),
            }
        }
    }
    impl std::str::FromStr
    for BlobGetAllResponseBlobsItemDataStorageReferencesItemBlobStorage {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "google" => Ok(Self::Google),
                "swarm" => Ok(Self::Swarm),
                "postgres" => Ok(Self::Postgres),
                "file_system" => Ok(Self::FileSystem),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str>
    for BlobGetAllResponseBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
    for BlobGetAllResponseBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
    for BlobGetAllResponseBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///BlobGetAllResponseBlobsItemTransaction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "blobAsCalldataGasFee": {
    ///      "type": "string"
    ///    },
    ///    "blobAsCalldataGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "blobGasBaseFee": {
    ///      "type": "string"
    ///    },
    ///    "blobGasMaxFee": {
    ///      "type": "string"
    ///    },
    ///    "blobGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "from": {
    ///      "type": "string"
    ///    },
    ///    "maxFeePerBlobGas": {
    ///      "type": "string"
    ///    },
    ///    "rollup": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "to": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlobGetAllResponseBlobsItemTransaction {
        #[serde(
            rename = "blobAsCalldataGasFee",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_as_calldata_gas_fee: Option<String>,
        #[serde(
            rename = "blobAsCalldataGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_as_calldata_gas_used: Option<String>,
        #[serde(
            rename = "blobGasBaseFee",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_base_fee: Option<String>,
        #[serde(
            rename = "blobGasMaxFee",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_max_fee: Option<String>,
        #[serde(
            rename = "blobGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_used: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub from: Option<String>,
        #[serde(
            rename = "maxFeePerBlobGas",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_fee_per_blob_gas: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rollup: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub to: Option<String>,
    }
    impl From<&BlobGetAllResponseBlobsItemTransaction>
    for BlobGetAllResponseBlobsItemTransaction {
        fn from(value: &BlobGetAllResponseBlobsItemTransaction) -> Self {
            value.clone()
        }
    }
    ///BlobGetAllResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlobGetAllResponseIssuesItem {
        pub message: String,
    }
    impl From<&BlobGetAllResponseIssuesItem> for BlobGetAllResponseIssuesItem {
        fn from(value: &BlobGetAllResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///BlobGetAllSort
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "desc",
    ///  "type": "string",
    ///  "enum": [
    ///    "asc",
    ///    "desc"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum BlobGetAllSort {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
    }
    impl From<&BlobGetAllSort> for BlobGetAllSort {
        fn from(value: &BlobGetAllSort) -> Self {
            value.clone()
        }
    }
    impl ToString for BlobGetAllSort {
        fn to_string(&self) -> String {
            match *self {
                Self::Asc => "asc".to_string(),
                Self::Desc => "desc".to_string(),
            }
        }
    }
    impl std::str::FromStr for BlobGetAllSort {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "asc" => Ok(Self::Asc),
                "desc" => Ok(Self::Desc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str> for BlobGetAllSort {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for BlobGetAllSort {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for BlobGetAllSort {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl Default for BlobGetAllSort {
        fn default() -> Self {
            BlobGetAllSort::Desc
        }
    }
    ///BlobGetAllType
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "canonical",
    ///  "type": "string",
    ///  "enum": [
    ///    "reorged",
    ///    "canonical"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum BlobGetAllType {
        #[serde(rename = "reorged")]
        Reorged,
        #[serde(rename = "canonical")]
        Canonical,
    }
    impl From<&BlobGetAllType> for BlobGetAllType {
        fn from(value: &BlobGetAllType) -> Self {
            value.clone()
        }
    }
    impl ToString for BlobGetAllType {
        fn to_string(&self) -> String {
            match *self {
                Self::Reorged => "reorged".to_string(),
                Self::Canonical => "canonical".to_string(),
            }
        }
    }
    impl std::str::FromStr for BlobGetAllType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "reorged" => Ok(Self::Reorged),
                "canonical" => Ok(Self::Canonical),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str> for BlobGetAllType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for BlobGetAllType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for BlobGetAllType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl Default for BlobGetAllType {
        fn default() -> Self {
            BlobGetAllType::Canonical
        }
    }
    ///BlobGetByBlobIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlobGetByBlobIdResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<BlobGetByBlobIdResponseIssuesItem>,
        pub message: String,
    }
    impl From<&BlobGetByBlobIdResponse> for BlobGetByBlobIdResponse {
        fn from(value: &BlobGetByBlobIdResponse) -> Self {
            value.clone()
        }
    }
    ///BlobGetByBlobIdResponseDataStorageReferencesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blobStorage",
    ///    "dataReference"
    ///  ],
    ///  "properties": {
    ///    "blobStorage": {
    ///      "type": "string",
    ///      "enum": [
    ///        "google",
    ///        "swarm",
    ///        "postgres",
    ///        "file_system"
    ///      ]
    ///    },
    ///    "dataReference": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlobGetByBlobIdResponseDataStorageReferencesItem {
        #[serde(rename = "blobStorage")]
        pub blob_storage: BlobGetByBlobIdResponseDataStorageReferencesItemBlobStorage,
        #[serde(rename = "dataReference")]
        pub data_reference: String,
    }
    impl From<&BlobGetByBlobIdResponseDataStorageReferencesItem>
    for BlobGetByBlobIdResponseDataStorageReferencesItem {
        fn from(value: &BlobGetByBlobIdResponseDataStorageReferencesItem) -> Self {
            value.clone()
        }
    }
    ///BlobGetByBlobIdResponseDataStorageReferencesItemBlobStorage
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "google",
    ///    "swarm",
    ///    "postgres",
    ///    "file_system"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum BlobGetByBlobIdResponseDataStorageReferencesItemBlobStorage {
        #[serde(rename = "google")]
        Google,
        #[serde(rename = "swarm")]
        Swarm,
        #[serde(rename = "postgres")]
        Postgres,
        #[serde(rename = "file_system")]
        FileSystem,
    }
    impl From<&BlobGetByBlobIdResponseDataStorageReferencesItemBlobStorage>
    for BlobGetByBlobIdResponseDataStorageReferencesItemBlobStorage {
        fn from(
            value: &BlobGetByBlobIdResponseDataStorageReferencesItemBlobStorage,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for BlobGetByBlobIdResponseDataStorageReferencesItemBlobStorage {
        fn to_string(&self) -> String {
            match *self {
                Self::Google => "google".to_string(),
                Self::Swarm => "swarm".to_string(),
                Self::Postgres => "postgres".to_string(),
                Self::FileSystem => "file_system".to_string(),
            }
        }
    }
    impl std::str::FromStr
    for BlobGetByBlobIdResponseDataStorageReferencesItemBlobStorage {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "google" => Ok(Self::Google),
                "swarm" => Ok(Self::Swarm),
                "postgres" => Ok(Self::Postgres),
                "file_system" => Ok(Self::FileSystem),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str>
    for BlobGetByBlobIdResponseDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
    for BlobGetByBlobIdResponseDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
    for BlobGetByBlobIdResponseDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///BlobGetByBlobIdResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlobGetByBlobIdResponseIssuesItem {
        pub message: String,
    }
    impl From<&BlobGetByBlobIdResponseIssuesItem> for BlobGetByBlobIdResponseIssuesItem {
        fn from(value: &BlobGetByBlobIdResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///BlobGetByBlobIdResponseTransactionsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "block",
    ///    "hash",
    ///    "index"
    ///  ],
    ///  "properties": {
    ///    "blobAsCalldataGasFee": {
    ///      "type": "string"
    ///    },
    ///    "blobAsCalldataGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "blobGasBaseFee": {
    ///      "type": "string"
    ///    },
    ///    "blobGasMaxFee": {
    ///      "type": "string"
    ///    },
    ///    "blobGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "block": {
    ///      "type": "object",
    ///      "required": [
    ///        "number"
    ///      ],
    ///      "properties": {
    ///        "blobAsCalldataGasUsed": {
    ///          "type": "string"
    ///        },
    ///        "blobGasPrice": {
    ///          "type": "string"
    ///        },
    ///        "blobGasUsed": {
    ///          "type": "string"
    ///        },
    ///        "excessBlobGas": {
    ///          "type": "string"
    ///        },
    ///        "hash": {
    ///          "type": "string"
    ///        },
    ///        "number": {
    ///          "type": "number",
    ///          "minimum": 0.0
    ///        },
    ///        "slot": {
    ///          "type": "number",
    ///          "minimum": 0.0
    ///        },
    ///        "timestamp": {
    ///          "type": "string"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    "from": {
    ///      "type": "string"
    ///    },
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "index": {
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    },
    ///    "maxFeePerBlobGas": {
    ///      "type": "string"
    ///    },
    ///    "rollup": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "to": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlobGetByBlobIdResponseTransactionsItem {
        #[serde(
            rename = "blobAsCalldataGasFee",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_as_calldata_gas_fee: Option<String>,
        #[serde(
            rename = "blobAsCalldataGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_as_calldata_gas_used: Option<String>,
        #[serde(
            rename = "blobGasBaseFee",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_base_fee: Option<String>,
        #[serde(
            rename = "blobGasMaxFee",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_max_fee: Option<String>,
        #[serde(
            rename = "blobGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_used: Option<String>,
        pub block: BlobGetByBlobIdResponseTransactionsItemBlock,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub from: Option<String>,
        pub hash: String,
        pub index: f64,
        #[serde(
            rename = "maxFeePerBlobGas",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_fee_per_blob_gas: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rollup: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub to: Option<String>,
    }
    impl From<&BlobGetByBlobIdResponseTransactionsItem>
    for BlobGetByBlobIdResponseTransactionsItem {
        fn from(value: &BlobGetByBlobIdResponseTransactionsItem) -> Self {
            value.clone()
        }
    }
    ///BlobGetByBlobIdResponseTransactionsItemBlock
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "number"
    ///  ],
    ///  "properties": {
    ///    "blobAsCalldataGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "blobGasPrice": {
    ///      "type": "string"
    ///    },
    ///    "blobGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "excessBlobGas": {
    ///      "type": "string"
    ///    },
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "number": {
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    },
    ///    "slot": {
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    },
    ///    "timestamp": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlobGetByBlobIdResponseTransactionsItemBlock {
        #[serde(
            rename = "blobAsCalldataGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_as_calldata_gas_used: Option<String>,
        #[serde(
            rename = "blobGasPrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_price: Option<String>,
        #[serde(
            rename = "blobGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_used: Option<String>,
        #[serde(
            rename = "excessBlobGas",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub excess_blob_gas: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hash: Option<String>,
        pub number: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub slot: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<String>,
    }
    impl From<&BlobGetByBlobIdResponseTransactionsItemBlock>
    for BlobGetByBlobIdResponseTransactionsItemBlock {
        fn from(value: &BlobGetByBlobIdResponseTransactionsItemBlock) -> Self {
            value.clone()
        }
    }
    ///BlockGetAllResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockGetAllResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<BlockGetAllResponseIssuesItem>,
        pub message: String,
    }
    impl From<&BlockGetAllResponse> for BlockGetAllResponse {
        fn from(value: &BlockGetAllResponse) -> Self {
            value.clone()
        }
    }
    ///BlockGetAllResponseBlocksItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blobAsCalldataGasUsed",
    ///    "blobGasPrice",
    ///    "blobGasUsed",
    ///    "excessBlobGas",
    ///    "hash",
    ///    "number",
    ///    "slot",
    ///    "timestamp",
    ///    "transactions"
    ///  ],
    ///  "properties": {
    ///    "blobAsCalldataGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "blobGasPrice": {
    ///      "type": "string"
    ///    },
    ///    "blobGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "excessBlobGas": {
    ///      "type": "string"
    ///    },
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "number": {
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    },
    ///    "slot": {
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    },
    ///    "timestamp": {
    ///      "type": "string"
    ///    },
    ///    "transactions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "blobs",
    ///          "hash"
    ///        ],
    ///        "properties": {
    ///          "blobAsCalldataGasFee": {
    ///            "type": "string"
    ///          },
    ///          "blobAsCalldataGasUsed": {
    ///            "type": "string"
    ///          },
    ///          "blobGasBaseFee": {
    ///            "type": "string"
    ///          },
    ///          "blobGasMaxFee": {
    ///            "type": "string"
    ///          },
    ///          "blobGasUsed": {
    ///            "type": "string"
    ///          },
    ///          "blobs": {
    ///            "type": "array",
    ///            "items": {
    ///              "type": "object",
    ///              "required": [
    ///                "index",
    ///                "versionedHash"
    ///              ],
    ///              "properties": {
    ///                "commitment": {
    ///                  "type": "string"
    ///                },
    ///                "data": {
    ///                  "type": "string"
    ///                },
    ///                "dataStorageReferences": {
    ///                  "type": "array",
    ///                  "items": {
    ///                    "type": "object",
    ///                    "required": [
    ///                      "blobStorage",
    ///                      "dataReference"
    ///                    ],
    ///                    "properties": {
    ///                      "blobStorage": {
    ///                        "type": "string",
    ///                        "enum": [
    ///                          "google",
    ///                          "swarm",
    ///                          "postgres",
    ///                          "file_system"
    ///                        ]
    ///                      },
    ///                      "dataReference": {
    ///                        "type": "string"
    ///                      }
    ///                    },
    ///                    "additionalProperties": false
    ///                  }
    ///                },
    ///                "index": {
    ///                  "type": "number"
    ///                },
    ///                "proof": {
    ///                  "type": [
    ///                    "string",
    ///                    "null"
    ///                  ]
    ///                },
    ///                "size": {
    ///                  "type": "number"
    ///                },
    ///                "versionedHash": {
    ///                  "type": "string"
    ///                }
    ///              },
    ///              "additionalProperties": false
    ///            }
    ///          },
    ///          "from": {
    ///            "type": "string"
    ///          },
    ///          "hash": {
    ///            "type": "string"
    ///          },
    ///          "maxFeePerBlobGas": {
    ///            "type": "string"
    ///          },
    ///          "rollup": {
    ///            "type": [
    ///              "string",
    ///              "null"
    ///            ]
    ///          },
    ///          "to": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockGetAllResponseBlocksItem {
        #[serde(rename = "blobAsCalldataGasUsed")]
        pub blob_as_calldata_gas_used: String,
        #[serde(rename = "blobGasPrice")]
        pub blob_gas_price: String,
        #[serde(rename = "blobGasUsed")]
        pub blob_gas_used: String,
        #[serde(rename = "excessBlobGas")]
        pub excess_blob_gas: String,
        pub hash: String,
        pub number: f64,
        pub slot: f64,
        pub timestamp: String,
        pub transactions: Vec<BlockGetAllResponseBlocksItemTransactionsItem>,
    }
    impl From<&BlockGetAllResponseBlocksItem> for BlockGetAllResponseBlocksItem {
        fn from(value: &BlockGetAllResponseBlocksItem) -> Self {
            value.clone()
        }
    }
    ///BlockGetAllResponseBlocksItemTransactionsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blobs",
    ///    "hash"
    ///  ],
    ///  "properties": {
    ///    "blobAsCalldataGasFee": {
    ///      "type": "string"
    ///    },
    ///    "blobAsCalldataGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "blobGasBaseFee": {
    ///      "type": "string"
    ///    },
    ///    "blobGasMaxFee": {
    ///      "type": "string"
    ///    },
    ///    "blobGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "blobs": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "index",
    ///          "versionedHash"
    ///        ],
    ///        "properties": {
    ///          "commitment": {
    ///            "type": "string"
    ///          },
    ///          "data": {
    ///            "type": "string"
    ///          },
    ///          "dataStorageReferences": {
    ///            "type": "array",
    ///            "items": {
    ///              "type": "object",
    ///              "required": [
    ///                "blobStorage",
    ///                "dataReference"
    ///              ],
    ///              "properties": {
    ///                "blobStorage": {
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "google",
    ///                    "swarm",
    ///                    "postgres",
    ///                    "file_system"
    ///                  ]
    ///                },
    ///                "dataReference": {
    ///                  "type": "string"
    ///                }
    ///              },
    ///              "additionalProperties": false
    ///            }
    ///          },
    ///          "index": {
    ///            "type": "number"
    ///          },
    ///          "proof": {
    ///            "type": [
    ///              "string",
    ///              "null"
    ///            ]
    ///          },
    ///          "size": {
    ///            "type": "number"
    ///          },
    ///          "versionedHash": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "from": {
    ///      "type": "string"
    ///    },
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "maxFeePerBlobGas": {
    ///      "type": "string"
    ///    },
    ///    "rollup": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "to": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockGetAllResponseBlocksItemTransactionsItem {
        #[serde(
            rename = "blobAsCalldataGasFee",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_as_calldata_gas_fee: Option<String>,
        #[serde(
            rename = "blobAsCalldataGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_as_calldata_gas_used: Option<String>,
        #[serde(
            rename = "blobGasBaseFee",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_base_fee: Option<String>,
        #[serde(
            rename = "blobGasMaxFee",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_max_fee: Option<String>,
        #[serde(
            rename = "blobGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_used: Option<String>,
        pub blobs: Vec<BlockGetAllResponseBlocksItemTransactionsItemBlobsItem>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub from: Option<String>,
        pub hash: String,
        #[serde(
            rename = "maxFeePerBlobGas",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_fee_per_blob_gas: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rollup: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub to: Option<String>,
    }
    impl From<&BlockGetAllResponseBlocksItemTransactionsItem>
    for BlockGetAllResponseBlocksItemTransactionsItem {
        fn from(value: &BlockGetAllResponseBlocksItemTransactionsItem) -> Self {
            value.clone()
        }
    }
    ///BlockGetAllResponseBlocksItemTransactionsItemBlobsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "index",
    ///    "versionedHash"
    ///  ],
    ///  "properties": {
    ///    "commitment": {
    ///      "type": "string"
    ///    },
    ///    "data": {
    ///      "type": "string"
    ///    },
    ///    "dataStorageReferences": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "blobStorage",
    ///          "dataReference"
    ///        ],
    ///        "properties": {
    ///          "blobStorage": {
    ///            "type": "string",
    ///            "enum": [
    ///              "google",
    ///              "swarm",
    ///              "postgres",
    ///              "file_system"
    ///            ]
    ///          },
    ///          "dataReference": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "index": {
    ///      "type": "number"
    ///    },
    ///    "proof": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "size": {
    ///      "type": "number"
    ///    },
    ///    "versionedHash": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockGetAllResponseBlocksItemTransactionsItemBlobsItem {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub commitment: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data: Option<String>,
        #[serde(
            rename = "dataStorageReferences",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub data_storage_references: Vec<
            BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItem,
        >,
        pub index: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub proof: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        #[serde(rename = "versionedHash")]
        pub versioned_hash: String,
    }
    impl From<&BlockGetAllResponseBlocksItemTransactionsItemBlobsItem>
    for BlockGetAllResponseBlocksItemTransactionsItemBlobsItem {
        fn from(value: &BlockGetAllResponseBlocksItemTransactionsItemBlobsItem) -> Self {
            value.clone()
        }
    }
    ///BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blobStorage",
    ///    "dataReference"
    ///  ],
    ///  "properties": {
    ///    "blobStorage": {
    ///      "type": "string",
    ///      "enum": [
    ///        "google",
    ///        "swarm",
    ///        "postgres",
    ///        "file_system"
    ///      ]
    ///    },
    ///    "dataReference": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItem {
        #[serde(rename = "blobStorage")]
        pub blob_storage: BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage,
        #[serde(rename = "dataReference")]
        pub data_reference: String,
    }
    impl From<
        &BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItem,
    >
    for BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItem {
        fn from(
            value: &BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItem,
        ) -> Self {
            value.clone()
        }
    }
    ///BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "google",
    ///    "swarm",
    ///    "postgres",
    ///    "file_system"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        #[serde(rename = "google")]
        Google,
        #[serde(rename = "swarm")]
        Swarm,
        #[serde(rename = "postgres")]
        Postgres,
        #[serde(rename = "file_system")]
        FileSystem,
    }
    impl From<
        &BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage,
    >
    for BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        fn from(
            value: &BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString
    for BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        fn to_string(&self) -> String {
            match *self {
                Self::Google => "google".to_string(),
                Self::Swarm => "swarm".to_string(),
                Self::Postgres => "postgres".to_string(),
                Self::FileSystem => "file_system".to_string(),
            }
        }
    }
    impl std::str::FromStr
    for BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "google" => Ok(Self::Google),
                "swarm" => Ok(Self::Swarm),
                "postgres" => Ok(Self::Postgres),
                "file_system" => Ok(Self::FileSystem),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str>
    for BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
    for BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
    for BlockGetAllResponseBlocksItemTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///BlockGetAllResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockGetAllResponseIssuesItem {
        pub message: String,
    }
    impl From<&BlockGetAllResponseIssuesItem> for BlockGetAllResponseIssuesItem {
        fn from(value: &BlockGetAllResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///BlockGetAllSort
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "desc",
    ///  "type": "string",
    ///  "enum": [
    ///    "asc",
    ///    "desc"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum BlockGetAllSort {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
    }
    impl From<&BlockGetAllSort> for BlockGetAllSort {
        fn from(value: &BlockGetAllSort) -> Self {
            value.clone()
        }
    }
    impl ToString for BlockGetAllSort {
        fn to_string(&self) -> String {
            match *self {
                Self::Asc => "asc".to_string(),
                Self::Desc => "desc".to_string(),
            }
        }
    }
    impl std::str::FromStr for BlockGetAllSort {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "asc" => Ok(Self::Asc),
                "desc" => Ok(Self::Desc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str> for BlockGetAllSort {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for BlockGetAllSort {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for BlockGetAllSort {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl Default for BlockGetAllSort {
        fn default() -> Self {
            BlockGetAllSort::Desc
        }
    }
    ///BlockGetAllType
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "canonical",
    ///  "type": "string",
    ///  "enum": [
    ///    "reorged",
    ///    "canonical"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum BlockGetAllType {
        #[serde(rename = "reorged")]
        Reorged,
        #[serde(rename = "canonical")]
        Canonical,
    }
    impl From<&BlockGetAllType> for BlockGetAllType {
        fn from(value: &BlockGetAllType) -> Self {
            value.clone()
        }
    }
    impl ToString for BlockGetAllType {
        fn to_string(&self) -> String {
            match *self {
                Self::Reorged => "reorged".to_string(),
                Self::Canonical => "canonical".to_string(),
            }
        }
    }
    impl std::str::FromStr for BlockGetAllType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "reorged" => Ok(Self::Reorged),
                "canonical" => Ok(Self::Canonical),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str> for BlockGetAllType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for BlockGetAllType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for BlockGetAllType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl Default for BlockGetAllType {
        fn default() -> Self {
            BlockGetAllType::Canonical
        }
    }
    ///BlockGetByBlockIdResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockGetByBlockIdResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<BlockGetByBlockIdResponseIssuesItem>,
        pub message: String,
    }
    impl From<&BlockGetByBlockIdResponse> for BlockGetByBlockIdResponse {
        fn from(value: &BlockGetByBlockIdResponse) -> Self {
            value.clone()
        }
    }
    ///BlockGetByBlockIdResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockGetByBlockIdResponseIssuesItem {
        pub message: String,
    }
    impl From<&BlockGetByBlockIdResponseIssuesItem>
    for BlockGetByBlockIdResponseIssuesItem {
        fn from(value: &BlockGetByBlockIdResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///BlockGetByBlockIdResponseTransactionsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blobs",
    ///    "hash"
    ///  ],
    ///  "properties": {
    ///    "blobAsCalldataGasFee": {
    ///      "type": "string"
    ///    },
    ///    "blobAsCalldataGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "blobGasBaseFee": {
    ///      "type": "string"
    ///    },
    ///    "blobGasMaxFee": {
    ///      "type": "string"
    ///    },
    ///    "blobGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "blobs": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "index",
    ///          "versionedHash"
    ///        ],
    ///        "properties": {
    ///          "commitment": {
    ///            "type": "string"
    ///          },
    ///          "data": {
    ///            "type": "string"
    ///          },
    ///          "dataStorageReferences": {
    ///            "type": "array",
    ///            "items": {
    ///              "type": "object",
    ///              "required": [
    ///                "blobStorage",
    ///                "dataReference"
    ///              ],
    ///              "properties": {
    ///                "blobStorage": {
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "google",
    ///                    "swarm",
    ///                    "postgres",
    ///                    "file_system"
    ///                  ]
    ///                },
    ///                "dataReference": {
    ///                  "type": "string"
    ///                }
    ///              },
    ///              "additionalProperties": false
    ///            }
    ///          },
    ///          "index": {
    ///            "type": "number"
    ///          },
    ///          "proof": {
    ///            "type": [
    ///              "string",
    ///              "null"
    ///            ]
    ///          },
    ///          "size": {
    ///            "type": "number"
    ///          },
    ///          "versionedHash": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "from": {
    ///      "type": "string"
    ///    },
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "maxFeePerBlobGas": {
    ///      "type": "string"
    ///    },
    ///    "rollup": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "to": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockGetByBlockIdResponseTransactionsItem {
        #[serde(
            rename = "blobAsCalldataGasFee",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_as_calldata_gas_fee: Option<String>,
        #[serde(
            rename = "blobAsCalldataGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_as_calldata_gas_used: Option<String>,
        #[serde(
            rename = "blobGasBaseFee",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_base_fee: Option<String>,
        #[serde(
            rename = "blobGasMaxFee",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_max_fee: Option<String>,
        #[serde(
            rename = "blobGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_used: Option<String>,
        pub blobs: Vec<BlockGetByBlockIdResponseTransactionsItemBlobsItem>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub from: Option<String>,
        pub hash: String,
        #[serde(
            rename = "maxFeePerBlobGas",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_fee_per_blob_gas: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rollup: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub to: Option<String>,
    }
    impl From<&BlockGetByBlockIdResponseTransactionsItem>
    for BlockGetByBlockIdResponseTransactionsItem {
        fn from(value: &BlockGetByBlockIdResponseTransactionsItem) -> Self {
            value.clone()
        }
    }
    ///BlockGetByBlockIdResponseTransactionsItemBlobsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "index",
    ///    "versionedHash"
    ///  ],
    ///  "properties": {
    ///    "commitment": {
    ///      "type": "string"
    ///    },
    ///    "data": {
    ///      "type": "string"
    ///    },
    ///    "dataStorageReferences": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "blobStorage",
    ///          "dataReference"
    ///        ],
    ///        "properties": {
    ///          "blobStorage": {
    ///            "type": "string",
    ///            "enum": [
    ///              "google",
    ///              "swarm",
    ///              "postgres",
    ///              "file_system"
    ///            ]
    ///          },
    ///          "dataReference": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "index": {
    ///      "type": "number"
    ///    },
    ///    "proof": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "size": {
    ///      "type": "number"
    ///    },
    ///    "versionedHash": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockGetByBlockIdResponseTransactionsItemBlobsItem {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub commitment: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data: Option<String>,
        #[serde(
            rename = "dataStorageReferences",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub data_storage_references: Vec<
            BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItem,
        >,
        pub index: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub proof: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        #[serde(rename = "versionedHash")]
        pub versioned_hash: String,
    }
    impl From<&BlockGetByBlockIdResponseTransactionsItemBlobsItem>
    for BlockGetByBlockIdResponseTransactionsItemBlobsItem {
        fn from(value: &BlockGetByBlockIdResponseTransactionsItemBlobsItem) -> Self {
            value.clone()
        }
    }
    ///BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blobStorage",
    ///    "dataReference"
    ///  ],
    ///  "properties": {
    ///    "blobStorage": {
    ///      "type": "string",
    ///      "enum": [
    ///        "google",
    ///        "swarm",
    ///        "postgres",
    ///        "file_system"
    ///      ]
    ///    },
    ///    "dataReference": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItem {
        #[serde(rename = "blobStorage")]
        pub blob_storage: BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage,
        #[serde(rename = "dataReference")]
        pub data_reference: String,
    }
    impl From<
        &BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItem,
    > for BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItem {
        fn from(
            value: &BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItem,
        ) -> Self {
            value.clone()
        }
    }
    ///BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "google",
    ///    "swarm",
    ///    "postgres",
    ///    "file_system"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        #[serde(rename = "google")]
        Google,
        #[serde(rename = "swarm")]
        Swarm,
        #[serde(rename = "postgres")]
        Postgres,
        #[serde(rename = "file_system")]
        FileSystem,
    }
    impl From<
        &BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage,
    >
    for BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        fn from(
            value: &BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString
    for BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        fn to_string(&self) -> String {
            match *self {
                Self::Google => "google".to_string(),
                Self::Swarm => "swarm".to_string(),
                Self::Postgres => "postgres".to_string(),
                Self::FileSystem => "file_system".to_string(),
            }
        }
    }
    impl std::str::FromStr
    for BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "google" => Ok(Self::Google),
                "swarm" => Ok(Self::Swarm),
                "postgres" => Ok(Self::Postgres),
                "file_system" => Ok(Self::FileSystem),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str>
    for BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
    for BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
    for BlockGetByBlockIdResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///BlockGetByBlockIdType
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "canonical",
    ///  "type": "string",
    ///  "enum": [
    ///    "reorged",
    ///    "canonical"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum BlockGetByBlockIdType {
        #[serde(rename = "reorged")]
        Reorged,
        #[serde(rename = "canonical")]
        Canonical,
    }
    impl From<&BlockGetByBlockIdType> for BlockGetByBlockIdType {
        fn from(value: &BlockGetByBlockIdType) -> Self {
            value.clone()
        }
    }
    impl ToString for BlockGetByBlockIdType {
        fn to_string(&self) -> String {
            match *self {
                Self::Reorged => "reorged".to_string(),
                Self::Canonical => "canonical".to_string(),
            }
        }
    }
    impl std::str::FromStr for BlockGetByBlockIdType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "reorged" => Ok(Self::Reorged),
                "canonical" => Ok(Self::Canonical),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str> for BlockGetByBlockIdType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for BlockGetByBlockIdType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for BlockGetByBlockIdType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl Default for BlockGetByBlockIdType {
        fn default() -> Self {
            BlockGetByBlockIdType::Canonical
        }
    }
    ///HealthcheckResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct HealthcheckResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<HealthcheckResponseIssuesItem>,
        pub message: String,
    }
    impl From<&HealthcheckResponse> for HealthcheckResponse {
        fn from(value: &HealthcheckResponse) -> Self {
            value.clone()
        }
    }
    ///HealthcheckResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct HealthcheckResponseIssuesItem {
        pub message: String,
    }
    impl From<&HealthcheckResponseIssuesItem> for HealthcheckResponseIssuesItem {
        fn from(value: &HealthcheckResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///IndexerHandleReorgedSlotsBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "reorgedSlots"
    ///  ],
    ///  "properties": {
    ///    "reorgedSlots": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "number",
    ///        "exclusiveMinimum": 0.0
    ///      },
    ///      "minItems": 1
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct IndexerHandleReorgedSlotsBody {
        #[serde(rename = "reorgedSlots")]
        pub reorged_slots: Vec<f64>,
    }
    impl From<&IndexerHandleReorgedSlotsBody> for IndexerHandleReorgedSlotsBody {
        fn from(value: &IndexerHandleReorgedSlotsBody) -> Self {
            value.clone()
        }
    }
    ///IndexerHandleReorgedSlotsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct IndexerHandleReorgedSlotsResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<IndexerHandleReorgedSlotsResponseIssuesItem>,
        pub message: String,
    }
    impl From<&IndexerHandleReorgedSlotsResponse> for IndexerHandleReorgedSlotsResponse {
        fn from(value: &IndexerHandleReorgedSlotsResponse) -> Self {
            value.clone()
        }
    }
    ///IndexerHandleReorgedSlotsResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct IndexerHandleReorgedSlotsResponseIssuesItem {
        pub message: String,
    }
    impl From<&IndexerHandleReorgedSlotsResponseIssuesItem>
    for IndexerHandleReorgedSlotsResponseIssuesItem {
        fn from(value: &IndexerHandleReorgedSlotsResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///IndexerIndexDataBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blobs",
    ///    "block",
    ///    "transactions"
    ///  ],
    ///  "properties": {
    ///    "blobs": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "commitment",
    ///          "data",
    ///          "index",
    ///          "proof",
    ///          "txHash",
    ///          "versionedHash"
    ///        ],
    ///        "properties": {
    ///          "commitment": {
    ///            "type": "string"
    ///          },
    ///          "data": {
    ///            "type": "string"
    ///          },
    ///          "index": {
    ///            "type": "number"
    ///          },
    ///          "proof": {
    ///            "type": "string"
    ///          },
    ///          "txHash": {
    ///            "type": "string"
    ///          },
    ///          "versionedHash": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "block": {
    ///      "type": "object",
    ///      "required": [
    ///        "blobGasUsed",
    ///        "excessBlobGas",
    ///        "hash",
    ///        "number",
    ///        "slot",
    ///        "timestamp"
    ///      ],
    ///      "properties": {
    ///        "blobGasUsed": {
    ///          "type": "string"
    ///        },
    ///        "excessBlobGas": {
    ///          "type": "string"
    ///        },
    ///        "hash": {
    ///          "type": "string"
    ///        },
    ///        "number": {
    ///          "type": "number"
    ///        },
    ///        "slot": {
    ///          "type": "number"
    ///        },
    ///        "timestamp": {
    ///          "type": "number"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    "transactions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "blockNumber",
    ///          "from",
    ///          "gasPrice",
    ///          "hash",
    ///          "maxFeePerBlobGas",
    ///          "to"
    ///        ],
    ///        "properties": {
    ///          "blockNumber": {
    ///            "type": "number"
    ///          },
    ///          "from": {
    ///            "type": "string"
    ///          },
    ///          "gasPrice": {
    ///            "type": "string"
    ///          },
    ///          "hash": {
    ///            "type": "string"
    ///          },
    ///          "maxFeePerBlobGas": {
    ///            "type": "string"
    ///          },
    ///          "to": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct IndexerIndexDataBody {
        pub blobs: Vec<IndexerIndexDataBodyBlobsItem>,
        pub block: IndexerIndexDataBodyBlock,
        pub transactions: Vec<IndexerIndexDataBodyTransactionsItem>,
    }
    impl From<&IndexerIndexDataBody> for IndexerIndexDataBody {
        fn from(value: &IndexerIndexDataBody) -> Self {
            value.clone()
        }
    }
    ///IndexerIndexDataBodyBlobsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "commitment",
    ///    "data",
    ///    "index",
    ///    "proof",
    ///    "txHash",
    ///    "versionedHash"
    ///  ],
    ///  "properties": {
    ///    "commitment": {
    ///      "type": "string"
    ///    },
    ///    "data": {
    ///      "type": "string"
    ///    },
    ///    "index": {
    ///      "type": "number"
    ///    },
    ///    "proof": {
    ///      "type": "string"
    ///    },
    ///    "txHash": {
    ///      "type": "string"
    ///    },
    ///    "versionedHash": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct IndexerIndexDataBodyBlobsItem {
        pub commitment: String,
        pub data: String,
        pub index: f64,
        pub proof: String,
        #[serde(rename = "txHash")]
        pub tx_hash: String,
        #[serde(rename = "versionedHash")]
        pub versioned_hash: String,
    }
    impl From<&IndexerIndexDataBodyBlobsItem> for IndexerIndexDataBodyBlobsItem {
        fn from(value: &IndexerIndexDataBodyBlobsItem) -> Self {
            value.clone()
        }
    }
    ///IndexerIndexDataBodyBlock
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blobGasUsed",
    ///    "excessBlobGas",
    ///    "hash",
    ///    "number",
    ///    "slot",
    ///    "timestamp"
    ///  ],
    ///  "properties": {
    ///    "blobGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "excessBlobGas": {
    ///      "type": "string"
    ///    },
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "number": {
    ///      "type": "number"
    ///    },
    ///    "slot": {
    ///      "type": "number"
    ///    },
    ///    "timestamp": {
    ///      "type": "number"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct IndexerIndexDataBodyBlock {
        #[serde(rename = "blobGasUsed")]
        pub blob_gas_used: String,
        #[serde(rename = "excessBlobGas")]
        pub excess_blob_gas: String,
        pub hash: String,
        pub number: f64,
        pub slot: f64,
        pub timestamp: f64,
    }
    impl From<&IndexerIndexDataBodyBlock> for IndexerIndexDataBodyBlock {
        fn from(value: &IndexerIndexDataBodyBlock) -> Self {
            value.clone()
        }
    }
    ///IndexerIndexDataBodyTransactionsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blockNumber",
    ///    "from",
    ///    "gasPrice",
    ///    "hash",
    ///    "maxFeePerBlobGas",
    ///    "to"
    ///  ],
    ///  "properties": {
    ///    "blockNumber": {
    ///      "type": "number"
    ///    },
    ///    "from": {
    ///      "type": "string"
    ///    },
    ///    "gasPrice": {
    ///      "type": "string"
    ///    },
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "maxFeePerBlobGas": {
    ///      "type": "string"
    ///    },
    ///    "to": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct IndexerIndexDataBodyTransactionsItem {
        #[serde(rename = "blockNumber")]
        pub block_number: f64,
        pub from: String,
        #[serde(rename = "gasPrice")]
        pub gas_price: String,
        pub hash: String,
        #[serde(rename = "maxFeePerBlobGas")]
        pub max_fee_per_blob_gas: String,
        pub to: String,
    }
    impl From<&IndexerIndexDataBodyTransactionsItem>
    for IndexerIndexDataBodyTransactionsItem {
        fn from(value: &IndexerIndexDataBodyTransactionsItem) -> Self {
            value.clone()
        }
    }
    ///IndexerIndexDataResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct IndexerIndexDataResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<IndexerIndexDataResponseIssuesItem>,
        pub message: String,
    }
    impl From<&IndexerIndexDataResponse> for IndexerIndexDataResponse {
        fn from(value: &IndexerIndexDataResponse) -> Self {
            value.clone()
        }
    }
    ///IndexerIndexDataResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct IndexerIndexDataResponseIssuesItem {
        pub message: String,
    }
    impl From<&IndexerIndexDataResponseIssuesItem>
    for IndexerIndexDataResponseIssuesItem {
        fn from(value: &IndexerIndexDataResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///StatsGetAllOverallStatsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetAllOverallStatsResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<StatsGetAllOverallStatsResponseIssuesItem>,
        pub message: String,
    }
    impl From<&StatsGetAllOverallStatsResponse> for StatsGetAllOverallStatsResponse {
        fn from(value: &StatsGetAllOverallStatsResponse) -> Self {
            value.clone()
        }
    }
    ///StatsGetAllOverallStatsResponseBlob
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "avgBlobSize",
    ///    "totalBlobSize",
    ///    "totalBlobs",
    ///    "totalUniqueBlobs",
    ///    "updatedAt"
    ///  ],
    ///  "properties": {
    ///    "avgBlobSize": {
    ///      "type": "number"
    ///    },
    ///    "totalBlobSize": {
    ///      "type": "string"
    ///    },
    ///    "totalBlobs": {
    ///      "type": "number"
    ///    },
    ///    "totalUniqueBlobs": {
    ///      "type": "number"
    ///    },
    ///    "updatedAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetAllOverallStatsResponseBlob {
        #[serde(rename = "avgBlobSize")]
        pub avg_blob_size: f64,
        #[serde(rename = "totalBlobSize")]
        pub total_blob_size: String,
        #[serde(rename = "totalBlobs")]
        pub total_blobs: f64,
        #[serde(rename = "totalUniqueBlobs")]
        pub total_unique_blobs: f64,
        #[serde(rename = "updatedAt")]
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }
    impl From<&StatsGetAllOverallStatsResponseBlob>
    for StatsGetAllOverallStatsResponseBlob {
        fn from(value: &StatsGetAllOverallStatsResponseBlob) -> Self {
            value.clone()
        }
    }
    ///StatsGetAllOverallStatsResponseBlock
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "avgBlobAsCalldataFee",
    ///    "avgBlobFee",
    ///    "avgBlobGasPrice",
    ///    "totalBlobAsCalldataFee",
    ///    "totalBlobAsCalldataGasUsed",
    ///    "totalBlobFee",
    ///    "totalBlobGasUsed",
    ///    "totalBlocks",
    ///    "updatedAt"
    ///  ],
    ///  "properties": {
    ///    "avgBlobAsCalldataFee": {
    ///      "type": "number"
    ///    },
    ///    "avgBlobFee": {
    ///      "type": "number"
    ///    },
    ///    "avgBlobGasPrice": {
    ///      "type": "number"
    ///    },
    ///    "totalBlobAsCalldataFee": {
    ///      "type": "string"
    ///    },
    ///    "totalBlobAsCalldataGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "totalBlobFee": {
    ///      "type": "string"
    ///    },
    ///    "totalBlobGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "totalBlocks": {
    ///      "type": "number"
    ///    },
    ///    "updatedAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetAllOverallStatsResponseBlock {
        #[serde(rename = "avgBlobAsCalldataFee")]
        pub avg_blob_as_calldata_fee: f64,
        #[serde(rename = "avgBlobFee")]
        pub avg_blob_fee: f64,
        #[serde(rename = "avgBlobGasPrice")]
        pub avg_blob_gas_price: f64,
        #[serde(rename = "totalBlobAsCalldataFee")]
        pub total_blob_as_calldata_fee: String,
        #[serde(rename = "totalBlobAsCalldataGasUsed")]
        pub total_blob_as_calldata_gas_used: String,
        #[serde(rename = "totalBlobFee")]
        pub total_blob_fee: String,
        #[serde(rename = "totalBlobGasUsed")]
        pub total_blob_gas_used: String,
        #[serde(rename = "totalBlocks")]
        pub total_blocks: f64,
        #[serde(rename = "updatedAt")]
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }
    impl From<&StatsGetAllOverallStatsResponseBlock>
    for StatsGetAllOverallStatsResponseBlock {
        fn from(value: &StatsGetAllOverallStatsResponseBlock) -> Self {
            value.clone()
        }
    }
    ///StatsGetAllOverallStatsResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetAllOverallStatsResponseIssuesItem {
        pub message: String,
    }
    impl From<&StatsGetAllOverallStatsResponseIssuesItem>
    for StatsGetAllOverallStatsResponseIssuesItem {
        fn from(value: &StatsGetAllOverallStatsResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///StatsGetAllOverallStatsResponseTransaction
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "avgMaxBlobGasFee",
    ///    "totalTransactions",
    ///    "totalUniqueReceivers",
    ///    "totalUniqueSenders",
    ///    "updatedAt"
    ///  ],
    ///  "properties": {
    ///    "avgMaxBlobGasFee": {
    ///      "type": "number"
    ///    },
    ///    "totalTransactions": {
    ///      "type": "number"
    ///    },
    ///    "totalUniqueReceivers": {
    ///      "type": "number"
    ///    },
    ///    "totalUniqueSenders": {
    ///      "type": "number"
    ///    },
    ///    "updatedAt": {
    ///      "type": "string",
    ///      "format": "date-time"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetAllOverallStatsResponseTransaction {
        #[serde(rename = "avgMaxBlobGasFee")]
        pub avg_max_blob_gas_fee: f64,
        #[serde(rename = "totalTransactions")]
        pub total_transactions: f64,
        #[serde(rename = "totalUniqueReceivers")]
        pub total_unique_receivers: f64,
        #[serde(rename = "totalUniqueSenders")]
        pub total_unique_senders: f64,
        #[serde(rename = "updatedAt")]
        pub updated_at: chrono::DateTime<chrono::offset::Utc>,
    }
    impl From<&StatsGetAllOverallStatsResponseTransaction>
    for StatsGetAllOverallStatsResponseTransaction {
        fn from(value: &StatsGetAllOverallStatsResponseTransaction) -> Self {
            value.clone()
        }
    }
    ///StatsGetBlobDailyStatsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetBlobDailyStatsResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<StatsGetBlobDailyStatsResponseIssuesItem>,
        pub message: String,
    }
    impl From<&StatsGetBlobDailyStatsResponse> for StatsGetBlobDailyStatsResponse {
        fn from(value: &StatsGetBlobDailyStatsResponse) -> Self {
            value.clone()
        }
    }
    ///StatsGetBlobDailyStatsResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetBlobDailyStatsResponseIssuesItem {
        pub message: String,
    }
    impl From<&StatsGetBlobDailyStatsResponseIssuesItem>
    for StatsGetBlobDailyStatsResponseIssuesItem {
        fn from(value: &StatsGetBlobDailyStatsResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///StatsGetBlobDailyStatsTimeFrame
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1d",
    ///    "7d",
    ///    "15d",
    ///    "30d",
    ///    "180d",
    ///    "360d",
    ///    "All"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum StatsGetBlobDailyStatsTimeFrame {
        #[serde(rename = "1d")]
        _1d,
        #[serde(rename = "7d")]
        _7d,
        #[serde(rename = "15d")]
        _15d,
        #[serde(rename = "30d")]
        _30d,
        #[serde(rename = "180d")]
        _180d,
        #[serde(rename = "360d")]
        _360d,
        All,
    }
    impl From<&StatsGetBlobDailyStatsTimeFrame> for StatsGetBlobDailyStatsTimeFrame {
        fn from(value: &StatsGetBlobDailyStatsTimeFrame) -> Self {
            value.clone()
        }
    }
    impl ToString for StatsGetBlobDailyStatsTimeFrame {
        fn to_string(&self) -> String {
            match *self {
                Self::_1d => "1d".to_string(),
                Self::_7d => "7d".to_string(),
                Self::_15d => "15d".to_string(),
                Self::_30d => "30d".to_string(),
                Self::_180d => "180d".to_string(),
                Self::_360d => "360d".to_string(),
                Self::All => "All".to_string(),
            }
        }
    }
    impl std::str::FromStr for StatsGetBlobDailyStatsTimeFrame {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "1d" => Ok(Self::_1d),
                "7d" => Ok(Self::_7d),
                "15d" => Ok(Self::_15d),
                "30d" => Ok(Self::_30d),
                "180d" => Ok(Self::_180d),
                "360d" => Ok(Self::_360d),
                "All" => Ok(Self::All),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str> for StatsGetBlobDailyStatsTimeFrame {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for StatsGetBlobDailyStatsTimeFrame {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for StatsGetBlobDailyStatsTimeFrame {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///StatsGetBlobOverallStatsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetBlobOverallStatsResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<StatsGetBlobOverallStatsResponseIssuesItem>,
        pub message: String,
    }
    impl From<&StatsGetBlobOverallStatsResponse> for StatsGetBlobOverallStatsResponse {
        fn from(value: &StatsGetBlobOverallStatsResponse) -> Self {
            value.clone()
        }
    }
    ///StatsGetBlobOverallStatsResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetBlobOverallStatsResponseIssuesItem {
        pub message: String,
    }
    impl From<&StatsGetBlobOverallStatsResponseIssuesItem>
    for StatsGetBlobOverallStatsResponseIssuesItem {
        fn from(value: &StatsGetBlobOverallStatsResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///StatsGetBlockDailyStatsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetBlockDailyStatsResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<StatsGetBlockDailyStatsResponseIssuesItem>,
        pub message: String,
    }
    impl From<&StatsGetBlockDailyStatsResponse> for StatsGetBlockDailyStatsResponse {
        fn from(value: &StatsGetBlockDailyStatsResponse) -> Self {
            value.clone()
        }
    }
    ///StatsGetBlockDailyStatsResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetBlockDailyStatsResponseIssuesItem {
        pub message: String,
    }
    impl From<&StatsGetBlockDailyStatsResponseIssuesItem>
    for StatsGetBlockDailyStatsResponseIssuesItem {
        fn from(value: &StatsGetBlockDailyStatsResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///StatsGetBlockDailyStatsTimeFrame
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1d",
    ///    "7d",
    ///    "15d",
    ///    "30d",
    ///    "180d",
    ///    "360d",
    ///    "All"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum StatsGetBlockDailyStatsTimeFrame {
        #[serde(rename = "1d")]
        _1d,
        #[serde(rename = "7d")]
        _7d,
        #[serde(rename = "15d")]
        _15d,
        #[serde(rename = "30d")]
        _30d,
        #[serde(rename = "180d")]
        _180d,
        #[serde(rename = "360d")]
        _360d,
        All,
    }
    impl From<&StatsGetBlockDailyStatsTimeFrame> for StatsGetBlockDailyStatsTimeFrame {
        fn from(value: &StatsGetBlockDailyStatsTimeFrame) -> Self {
            value.clone()
        }
    }
    impl ToString for StatsGetBlockDailyStatsTimeFrame {
        fn to_string(&self) -> String {
            match *self {
                Self::_1d => "1d".to_string(),
                Self::_7d => "7d".to_string(),
                Self::_15d => "15d".to_string(),
                Self::_30d => "30d".to_string(),
                Self::_180d => "180d".to_string(),
                Self::_360d => "360d".to_string(),
                Self::All => "All".to_string(),
            }
        }
    }
    impl std::str::FromStr for StatsGetBlockDailyStatsTimeFrame {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "1d" => Ok(Self::_1d),
                "7d" => Ok(Self::_7d),
                "15d" => Ok(Self::_15d),
                "30d" => Ok(Self::_30d),
                "180d" => Ok(Self::_180d),
                "360d" => Ok(Self::_360d),
                "All" => Ok(Self::All),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str> for StatsGetBlockDailyStatsTimeFrame {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for StatsGetBlockDailyStatsTimeFrame {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for StatsGetBlockDailyStatsTimeFrame {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///StatsGetBlockOverallStatsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetBlockOverallStatsResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<StatsGetBlockOverallStatsResponseIssuesItem>,
        pub message: String,
    }
    impl From<&StatsGetBlockOverallStatsResponse> for StatsGetBlockOverallStatsResponse {
        fn from(value: &StatsGetBlockOverallStatsResponse) -> Self {
            value.clone()
        }
    }
    ///StatsGetBlockOverallStatsResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetBlockOverallStatsResponseIssuesItem {
        pub message: String,
    }
    impl From<&StatsGetBlockOverallStatsResponseIssuesItem>
    for StatsGetBlockOverallStatsResponseIssuesItem {
        fn from(value: &StatsGetBlockOverallStatsResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///StatsGetTransactionDailyStatsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetTransactionDailyStatsResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<StatsGetTransactionDailyStatsResponseIssuesItem>,
        pub message: String,
    }
    impl From<&StatsGetTransactionDailyStatsResponse>
    for StatsGetTransactionDailyStatsResponse {
        fn from(value: &StatsGetTransactionDailyStatsResponse) -> Self {
            value.clone()
        }
    }
    ///StatsGetTransactionDailyStatsResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetTransactionDailyStatsResponseIssuesItem {
        pub message: String,
    }
    impl From<&StatsGetTransactionDailyStatsResponseIssuesItem>
    for StatsGetTransactionDailyStatsResponseIssuesItem {
        fn from(value: &StatsGetTransactionDailyStatsResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///StatsGetTransactionDailyStatsTimeFrame
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "1d",
    ///    "7d",
    ///    "15d",
    ///    "30d",
    ///    "180d",
    ///    "360d",
    ///    "All"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum StatsGetTransactionDailyStatsTimeFrame {
        #[serde(rename = "1d")]
        _1d,
        #[serde(rename = "7d")]
        _7d,
        #[serde(rename = "15d")]
        _15d,
        #[serde(rename = "30d")]
        _30d,
        #[serde(rename = "180d")]
        _180d,
        #[serde(rename = "360d")]
        _360d,
        All,
    }
    impl From<&StatsGetTransactionDailyStatsTimeFrame>
    for StatsGetTransactionDailyStatsTimeFrame {
        fn from(value: &StatsGetTransactionDailyStatsTimeFrame) -> Self {
            value.clone()
        }
    }
    impl ToString for StatsGetTransactionDailyStatsTimeFrame {
        fn to_string(&self) -> String {
            match *self {
                Self::_1d => "1d".to_string(),
                Self::_7d => "7d".to_string(),
                Self::_15d => "15d".to_string(),
                Self::_30d => "30d".to_string(),
                Self::_180d => "180d".to_string(),
                Self::_360d => "360d".to_string(),
                Self::All => "All".to_string(),
            }
        }
    }
    impl std::str::FromStr for StatsGetTransactionDailyStatsTimeFrame {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "1d" => Ok(Self::_1d),
                "7d" => Ok(Self::_7d),
                "15d" => Ok(Self::_15d),
                "30d" => Ok(Self::_30d),
                "180d" => Ok(Self::_180d),
                "360d" => Ok(Self::_360d),
                "All" => Ok(Self::All),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str> for StatsGetTransactionDailyStatsTimeFrame {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for StatsGetTransactionDailyStatsTimeFrame {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for StatsGetTransactionDailyStatsTimeFrame {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///StatsGetTransactionOverallStatsResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetTransactionOverallStatsResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<StatsGetTransactionOverallStatsResponseIssuesItem>,
        pub message: String,
    }
    impl From<&StatsGetTransactionOverallStatsResponse>
    for StatsGetTransactionOverallStatsResponse {
        fn from(value: &StatsGetTransactionOverallStatsResponse) -> Self {
            value.clone()
        }
    }
    ///StatsGetTransactionOverallStatsResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatsGetTransactionOverallStatsResponseIssuesItem {
        pub message: String,
    }
    impl From<&StatsGetTransactionOverallStatsResponseIssuesItem>
    for StatsGetTransactionOverallStatsResponseIssuesItem {
        fn from(value: &StatsGetTransactionOverallStatsResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///SyncStateGetStateResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct SyncStateGetStateResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<SyncStateGetStateResponseIssuesItem>,
        pub message: String,
    }
    impl From<&SyncStateGetStateResponse> for SyncStateGetStateResponse {
        fn from(value: &SyncStateGetStateResponse) -> Self {
            value.clone()
        }
    }
    ///SyncStateGetStateResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct SyncStateGetStateResponseIssuesItem {
        pub message: String,
    }
    impl From<&SyncStateGetStateResponseIssuesItem>
    for SyncStateGetStateResponseIssuesItem {
        fn from(value: &SyncStateGetStateResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///SyncStateUpdateStateBody
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "lastFinalizedBlock": {
    ///      "type": "number"
    ///    },
    ///    "lastLowerSyncedSlot": {
    ///      "type": "number"
    ///    },
    ///    "lastUpperSyncedSlot": {
    ///      "type": "number"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct SyncStateUpdateStateBody {
        #[serde(
            rename = "lastFinalizedBlock",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_finalized_block: Option<f64>,
        #[serde(
            rename = "lastLowerSyncedSlot",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_lower_synced_slot: Option<f64>,
        #[serde(
            rename = "lastUpperSyncedSlot",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_upper_synced_slot: Option<f64>,
    }
    impl From<&SyncStateUpdateStateBody> for SyncStateUpdateStateBody {
        fn from(value: &SyncStateUpdateStateBody) -> Self {
            value.clone()
        }
    }
    ///SyncStateUpdateStateResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct SyncStateUpdateStateResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<SyncStateUpdateStateResponseIssuesItem>,
        pub message: String,
    }
    impl From<&SyncStateUpdateStateResponse> for SyncStateUpdateStateResponse {
        fn from(value: &SyncStateUpdateStateResponse) -> Self {
            value.clone()
        }
    }
    ///SyncStateUpdateStateResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct SyncStateUpdateStateResponseIssuesItem {
        pub message: String,
    }
    impl From<&SyncStateUpdateStateResponseIssuesItem>
    for SyncStateUpdateStateResponseIssuesItem {
        fn from(value: &SyncStateUpdateStateResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///TxGetAllResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct TxGetAllResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<TxGetAllResponseIssuesItem>,
        pub message: String,
    }
    impl From<&TxGetAllResponse> for TxGetAllResponse {
        fn from(value: &TxGetAllResponse) -> Self {
            value.clone()
        }
    }
    ///TxGetAllResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct TxGetAllResponseIssuesItem {
        pub message: String,
    }
    impl From<&TxGetAllResponseIssuesItem> for TxGetAllResponseIssuesItem {
        fn from(value: &TxGetAllResponseIssuesItem) -> Self {
            value.clone()
        }
    }
    ///TxGetAllResponseTransactionsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blobAsCalldataGasFee",
    ///    "blobAsCalldataGasUsed",
    ///    "blobGasMaxFee",
    ///    "blobGasUsed",
    ///    "blobs",
    ///    "blockHash",
    ///    "blockNumber",
    ///    "from",
    ///    "hash",
    ///    "maxFeePerBlobGas",
    ///    "rollup",
    ///    "to"
    ///  ],
    ///  "properties": {
    ///    "blobAsCalldataGasFee": {
    ///      "type": "string"
    ///    },
    ///    "blobAsCalldataGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "blobGasBaseFee": {
    ///      "type": "string"
    ///    },
    ///    "blobGasMaxFee": {
    ///      "type": "string"
    ///    },
    ///    "blobGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "blobs": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "index",
    ///          "versionedHash"
    ///        ],
    ///        "properties": {
    ///          "commitment": {
    ///            "type": "string"
    ///          },
    ///          "data": {
    ///            "type": "string"
    ///          },
    ///          "dataStorageReferences": {
    ///            "type": "array",
    ///            "items": {
    ///              "type": "object",
    ///              "required": [
    ///                "blobStorage",
    ///                "dataReference"
    ///              ],
    ///              "properties": {
    ///                "blobStorage": {
    ///                  "type": "string",
    ///                  "enum": [
    ///                    "google",
    ///                    "swarm",
    ///                    "postgres",
    ///                    "file_system"
    ///                  ]
    ///                },
    ///                "dataReference": {
    ///                  "type": "string"
    ///                }
    ///              },
    ///              "additionalProperties": false
    ///            }
    ///          },
    ///          "index": {
    ///            "type": "number"
    ///          },
    ///          "proof": {
    ///            "type": [
    ///              "string",
    ///              "null"
    ///            ]
    ///          },
    ///          "size": {
    ///            "type": "number"
    ///          },
    ///          "versionedHash": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "block": {
    ///      "type": "object",
    ///      "properties": {
    ///        "blobAsCalldataGasUsed": {
    ///          "type": "string"
    ///        },
    ///        "blobGasPrice": {
    ///          "type": "string"
    ///        },
    ///        "blobGasUsed": {
    ///          "type": "string"
    ///        },
    ///        "excessBlobGas": {
    ///          "type": "string"
    ///        },
    ///        "hash": {
    ///          "type": "string"
    ///        },
    ///        "slot": {
    ///          "type": "number",
    ///          "minimum": 0.0
    ///        },
    ///        "timestamp": {
    ///          "type": "string"
    ///        }
    ///      },
    ///      "additionalProperties": false
    ///    },
    ///    "blockHash": {
    ///      "type": "string"
    ///    },
    ///    "blockNumber": {
    ///      "type": "number"
    ///    },
    ///    "from": {
    ///      "type": "string"
    ///    },
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "maxFeePerBlobGas": {
    ///      "type": "string"
    ///    },
    ///    "rollup": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "to": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct TxGetAllResponseTransactionsItem {
        #[serde(rename = "blobAsCalldataGasFee")]
        pub blob_as_calldata_gas_fee: String,
        #[serde(rename = "blobAsCalldataGasUsed")]
        pub blob_as_calldata_gas_used: String,
        #[serde(
            rename = "blobGasBaseFee",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_base_fee: Option<String>,
        #[serde(rename = "blobGasMaxFee")]
        pub blob_gas_max_fee: String,
        #[serde(rename = "blobGasUsed")]
        pub blob_gas_used: String,
        pub blobs: Vec<TxGetAllResponseTransactionsItemBlobsItem>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub block: Option<TxGetAllResponseTransactionsItemBlock>,
        #[serde(rename = "blockHash")]
        pub block_hash: String,
        #[serde(rename = "blockNumber")]
        pub block_number: f64,
        pub from: String,
        pub hash: String,
        #[serde(rename = "maxFeePerBlobGas")]
        pub max_fee_per_blob_gas: String,
        pub rollup: Option<String>,
        pub to: String,
    }
    impl From<&TxGetAllResponseTransactionsItem> for TxGetAllResponseTransactionsItem {
        fn from(value: &TxGetAllResponseTransactionsItem) -> Self {
            value.clone()
        }
    }
    ///TxGetAllResponseTransactionsItemBlobsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "index",
    ///    "versionedHash"
    ///  ],
    ///  "properties": {
    ///    "commitment": {
    ///      "type": "string"
    ///    },
    ///    "data": {
    ///      "type": "string"
    ///    },
    ///    "dataStorageReferences": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "blobStorage",
    ///          "dataReference"
    ///        ],
    ///        "properties": {
    ///          "blobStorage": {
    ///            "type": "string",
    ///            "enum": [
    ///              "google",
    ///              "swarm",
    ///              "postgres",
    ///              "file_system"
    ///            ]
    ///          },
    ///          "dataReference": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "index": {
    ///      "type": "number"
    ///    },
    ///    "proof": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "size": {
    ///      "type": "number"
    ///    },
    ///    "versionedHash": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct TxGetAllResponseTransactionsItemBlobsItem {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub commitment: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data: Option<String>,
        #[serde(
            rename = "dataStorageReferences",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub data_storage_references: Vec<
            TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItem,
        >,
        pub index: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub proof: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        #[serde(rename = "versionedHash")]
        pub versioned_hash: String,
    }
    impl From<&TxGetAllResponseTransactionsItemBlobsItem>
    for TxGetAllResponseTransactionsItemBlobsItem {
        fn from(value: &TxGetAllResponseTransactionsItemBlobsItem) -> Self {
            value.clone()
        }
    }
    ///TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blobStorage",
    ///    "dataReference"
    ///  ],
    ///  "properties": {
    ///    "blobStorage": {
    ///      "type": "string",
    ///      "enum": [
    ///        "google",
    ///        "swarm",
    ///        "postgres",
    ///        "file_system"
    ///      ]
    ///    },
    ///    "dataReference": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItem {
        #[serde(rename = "blobStorage")]
        pub blob_storage: TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage,
        #[serde(rename = "dataReference")]
        pub data_reference: String,
    }
    impl From<&TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItem>
    for TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItem {
        fn from(
            value: &TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItem,
        ) -> Self {
            value.clone()
        }
    }
    ///TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "google",
    ///    "swarm",
    ///    "postgres",
    ///    "file_system"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        #[serde(rename = "google")]
        Google,
        #[serde(rename = "swarm")]
        Swarm,
        #[serde(rename = "postgres")]
        Postgres,
        #[serde(rename = "file_system")]
        FileSystem,
    }
    impl From<
        &TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage,
    > for TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        fn from(
            value: &TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString
    for TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        fn to_string(&self) -> String {
            match *self {
                Self::Google => "google".to_string(),
                Self::Swarm => "swarm".to_string(),
                Self::Postgres => "postgres".to_string(),
                Self::FileSystem => "file_system".to_string(),
            }
        }
    }
    impl std::str::FromStr
    for TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "google" => Ok(Self::Google),
                "swarm" => Ok(Self::Swarm),
                "postgres" => Ok(Self::Postgres),
                "file_system" => Ok(Self::FileSystem),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str>
    for TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
    for TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
    for TxGetAllResponseTransactionsItemBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///TxGetAllResponseTransactionsItemBlock
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "blobAsCalldataGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "blobGasPrice": {
    ///      "type": "string"
    ///    },
    ///    "blobGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "excessBlobGas": {
    ///      "type": "string"
    ///    },
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "slot": {
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    },
    ///    "timestamp": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct TxGetAllResponseTransactionsItemBlock {
        #[serde(
            rename = "blobAsCalldataGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_as_calldata_gas_used: Option<String>,
        #[serde(
            rename = "blobGasPrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_price: Option<String>,
        #[serde(
            rename = "blobGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_used: Option<String>,
        #[serde(
            rename = "excessBlobGas",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub excess_blob_gas: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hash: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub slot: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<String>,
    }
    impl From<&TxGetAllResponseTransactionsItemBlock>
    for TxGetAllResponseTransactionsItemBlock {
        fn from(value: &TxGetAllResponseTransactionsItemBlock) -> Self {
            value.clone()
        }
    }
    ///TxGetAllSort
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "desc",
    ///  "type": "string",
    ///  "enum": [
    ///    "asc",
    ///    "desc"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum TxGetAllSort {
        #[serde(rename = "asc")]
        Asc,
        #[serde(rename = "desc")]
        Desc,
    }
    impl From<&TxGetAllSort> for TxGetAllSort {
        fn from(value: &TxGetAllSort) -> Self {
            value.clone()
        }
    }
    impl ToString for TxGetAllSort {
        fn to_string(&self) -> String {
            match *self {
                Self::Asc => "asc".to_string(),
                Self::Desc => "desc".to_string(),
            }
        }
    }
    impl std::str::FromStr for TxGetAllSort {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "asc" => Ok(Self::Asc),
                "desc" => Ok(Self::Desc),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str> for TxGetAllSort {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for TxGetAllSort {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for TxGetAllSort {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl Default for TxGetAllSort {
        fn default() -> Self {
            TxGetAllSort::Desc
        }
    }
    ///TxGetAllType
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "default": "canonical",
    ///  "type": "string",
    ///  "enum": [
    ///    "reorged",
    ///    "canonical"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum TxGetAllType {
        #[serde(rename = "reorged")]
        Reorged,
        #[serde(rename = "canonical")]
        Canonical,
    }
    impl From<&TxGetAllType> for TxGetAllType {
        fn from(value: &TxGetAllType) -> Self {
            value.clone()
        }
    }
    impl ToString for TxGetAllType {
        fn to_string(&self) -> String {
            match *self {
                Self::Reorged => "reorged".to_string(),
                Self::Canonical => "canonical".to_string(),
            }
        }
    }
    impl std::str::FromStr for TxGetAllType {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "reorged" => Ok(Self::Reorged),
                "canonical" => Ok(Self::Canonical),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str> for TxGetAllType {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String> for TxGetAllType {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String> for TxGetAllType {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl Default for TxGetAllType {
        fn default() -> Self {
            TxGetAllType::Canonical
        }
    }
    ///TxGetByHashResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "code",
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "code": {
    ///      "type": "string"
    ///    },
    ///    "issues": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "message"
    ///        ],
    ///        "properties": {
    ///          "message": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct TxGetByHashResponse {
        pub code: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub issues: Vec<TxGetByHashResponseIssuesItem>,
        pub message: String,
    }
    impl From<&TxGetByHashResponse> for TxGetByHashResponse {
        fn from(value: &TxGetByHashResponse) -> Self {
            value.clone()
        }
    }
    ///TxGetByHashResponseBlobsItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "index",
    ///    "versionedHash"
    ///  ],
    ///  "properties": {
    ///    "commitment": {
    ///      "type": "string"
    ///    },
    ///    "data": {
    ///      "type": "string"
    ///    },
    ///    "dataStorageReferences": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "object",
    ///        "required": [
    ///          "blobStorage",
    ///          "dataReference"
    ///        ],
    ///        "properties": {
    ///          "blobStorage": {
    ///            "type": "string",
    ///            "enum": [
    ///              "google",
    ///              "swarm",
    ///              "postgres",
    ///              "file_system"
    ///            ]
    ///          },
    ///          "dataReference": {
    ///            "type": "string"
    ///          }
    ///        },
    ///        "additionalProperties": false
    ///      }
    ///    },
    ///    "index": {
    ///      "type": "number"
    ///    },
    ///    "proof": {
    ///      "type": [
    ///        "string",
    ///        "null"
    ///      ]
    ///    },
    ///    "size": {
    ///      "type": "number"
    ///    },
    ///    "versionedHash": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct TxGetByHashResponseBlobsItem {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub commitment: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub data: Option<String>,
        #[serde(
            rename = "dataStorageReferences",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub data_storage_references: Vec<
            TxGetByHashResponseBlobsItemDataStorageReferencesItem,
        >,
        pub index: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub proof: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub size: Option<f64>,
        #[serde(rename = "versionedHash")]
        pub versioned_hash: String,
    }
    impl From<&TxGetByHashResponseBlobsItem> for TxGetByHashResponseBlobsItem {
        fn from(value: &TxGetByHashResponseBlobsItem) -> Self {
            value.clone()
        }
    }
    ///TxGetByHashResponseBlobsItemDataStorageReferencesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "blobStorage",
    ///    "dataReference"
    ///  ],
    ///  "properties": {
    ///    "blobStorage": {
    ///      "type": "string",
    ///      "enum": [
    ///        "google",
    ///        "swarm",
    ///        "postgres",
    ///        "file_system"
    ///      ]
    ///    },
    ///    "dataReference": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct TxGetByHashResponseBlobsItemDataStorageReferencesItem {
        #[serde(rename = "blobStorage")]
        pub blob_storage: TxGetByHashResponseBlobsItemDataStorageReferencesItemBlobStorage,
        #[serde(rename = "dataReference")]
        pub data_reference: String,
    }
    impl From<&TxGetByHashResponseBlobsItemDataStorageReferencesItem>
    for TxGetByHashResponseBlobsItemDataStorageReferencesItem {
        fn from(value: &TxGetByHashResponseBlobsItemDataStorageReferencesItem) -> Self {
            value.clone()
        }
    }
    ///TxGetByHashResponseBlobsItemDataStorageReferencesItemBlobStorage
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "google",
    ///    "swarm",
    ///    "postgres",
    ///    "file_system"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(
        Clone,
        Copy,
        Debug,
        Deserialize,
        Eq,
        Hash,
        Ord,
        PartialEq,
        PartialOrd,
        Serialize
    )]
    pub enum TxGetByHashResponseBlobsItemDataStorageReferencesItemBlobStorage {
        #[serde(rename = "google")]
        Google,
        #[serde(rename = "swarm")]
        Swarm,
        #[serde(rename = "postgres")]
        Postgres,
        #[serde(rename = "file_system")]
        FileSystem,
    }
    impl From<&TxGetByHashResponseBlobsItemDataStorageReferencesItemBlobStorage>
    for TxGetByHashResponseBlobsItemDataStorageReferencesItemBlobStorage {
        fn from(
            value: &TxGetByHashResponseBlobsItemDataStorageReferencesItemBlobStorage,
        ) -> Self {
            value.clone()
        }
    }
    impl ToString for TxGetByHashResponseBlobsItemDataStorageReferencesItemBlobStorage {
        fn to_string(&self) -> String {
            match *self {
                Self::Google => "google".to_string(),
                Self::Swarm => "swarm".to_string(),
                Self::Postgres => "postgres".to_string(),
                Self::FileSystem => "file_system".to_string(),
            }
        }
    }
    impl std::str::FromStr
    for TxGetByHashResponseBlobsItemDataStorageReferencesItemBlobStorage {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "google" => Ok(Self::Google),
                "swarm" => Ok(Self::Swarm),
                "postgres" => Ok(Self::Postgres),
                "file_system" => Ok(Self::FileSystem),
                _ => Err("invalid value".into()),
            }
        }
    }
    impl std::convert::TryFrom<&str>
    for TxGetByHashResponseBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<&String>
    for TxGetByHashResponseBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    impl std::convert::TryFrom<String>
    for TxGetByHashResponseBlobsItemDataStorageReferencesItemBlobStorage {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }
    ///TxGetByHashResponseBlock
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "blobAsCalldataGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "blobGasPrice": {
    ///      "type": "string"
    ///    },
    ///    "blobGasUsed": {
    ///      "type": "string"
    ///    },
    ///    "excessBlobGas": {
    ///      "type": "string"
    ///    },
    ///    "hash": {
    ///      "type": "string"
    ///    },
    ///    "slot": {
    ///      "type": "number",
    ///      "minimum": 0.0
    ///    },
    ///    "timestamp": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct TxGetByHashResponseBlock {
        #[serde(
            rename = "blobAsCalldataGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_as_calldata_gas_used: Option<String>,
        #[serde(
            rename = "blobGasPrice",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_price: Option<String>,
        #[serde(
            rename = "blobGasUsed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub blob_gas_used: Option<String>,
        #[serde(
            rename = "excessBlobGas",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub excess_blob_gas: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub hash: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub slot: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<String>,
    }
    impl From<&TxGetByHashResponseBlock> for TxGetByHashResponseBlock {
        fn from(value: &TxGetByHashResponseBlock) -> Self {
            value.clone()
        }
    }
    ///TxGetByHashResponseIssuesItem
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "required": [
    ///    "message"
    ///  ],
    ///  "properties": {
    ///    "message": {
    ///      "type": "string"
    ///    }
    ///  },
    ///  "additionalProperties": false
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[serde(deny_unknown_fields)]
    pub struct TxGetByHashResponseIssuesItem {
        pub message: String,
    }
    impl From<&TxGetByHashResponseIssuesItem> for TxGetByHashResponseIssuesItem {
        fn from(value: &TxGetByHashResponseIssuesItem) -> Self {
            value.clone()
        }
    }
}
#[derive(Clone, Debug)]
/**Client for Blobscan API

OpenAPI compliant REST API built using tRPC with Express

Version: 1.0.0*/
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}
impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new().connect_timeout(dur).timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }
    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }
    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }
    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }
    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.0.0"
    }
}
#[allow(clippy::all)]
impl Client {
    /**retrieves all blocks

Sends a `GET` request to `/blocks`

*/
    pub async fn block_get_all<'a>(
        &'a self,
        end_block: Option<f64>,
        end_date: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
        end_slot: Option<f64>,
        expand: Option<&'a str>,
        from: Option<&'a str>,
        p: Option<f64>,
        ps: Option<f64>,
        rollup: Option<&'a str>,
        sort: Option<types::BlockGetAllSort>,
        start_block: Option<f64>,
        start_date: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
        start_slot: Option<f64>,
        to: Option<&'a str>,
        type_: Option<types::BlockGetAllType>,
    ) -> Result<
        ResponseValue<types::BlockGetAllResponse>,
        Error<types::BlockGetAllResponse>,
    > {
        let url = format!("{}/blocks", self.baseurl,);
        let mut query = Vec::with_capacity(14usize);
        if let Some(v) = &end_block {
            query.push(("endBlock", v.to_string()));
        }
        if let Some(v) = &end_date {
            query.push(("endDate", v.to_string()));
        }
        if let Some(v) = &end_slot {
            query.push(("endSlot", v.to_string()));
        }
        if let Some(v) = &expand {
            query.push(("expand", v.to_string()));
        }
        if let Some(v) = &from {
            query.push(("from", v.to_string()));
        }
        if let Some(v) = &p {
            query.push(("p", v.to_string()));
        }
        if let Some(v) = &ps {
            query.push(("ps", v.to_string()));
        }
        if let Some(v) = &rollup {
            query.push(("rollup", v.to_string()));
        }
        if let Some(v) = &sort {
            query.push(("sort", v.to_string()));
        }
        if let Some(v) = &start_block {
            query.push(("startBlock", v.to_string()));
        }
        if let Some(v) = &start_date {
            query.push(("startDate", v.to_string()));
        }
        if let Some(v) = &start_slot {
            query.push(("startSlot", v.to_string()));
        }
        if let Some(v) = &to {
            query.push(("to", v.to_string()));
        }
        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**retrieves block details for given block number or hash

Sends a `GET` request to `/blocks/{id}`

*/
    pub async fn block_get_by_block_id<'a>(
        &'a self,
        id: &'a str,
        expand: Option<&'a str>,
        type_: Option<types::BlockGetByBlockIdType>,
    ) -> Result<
        ResponseValue<types::BlockGetByBlockIdResponse>,
        Error<types::BlockGetByBlockIdResponse>,
    > {
        let url = format!("{}/blocks/{}", self.baseurl, encode_path(& id.to_string()),);
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &expand {
            query.push(("expand", v.to_string()));
        }
        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**retrieves all blob transactions

Sends a `GET` request to `/transactions`

*/
    pub async fn tx_get_all<'a>(
        &'a self,
        end_block: Option<f64>,
        end_date: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
        end_slot: Option<f64>,
        expand: Option<&'a str>,
        from: Option<&'a str>,
        p: Option<f64>,
        ps: Option<f64>,
        rollup: Option<&'a str>,
        sort: Option<types::TxGetAllSort>,
        start_block: Option<f64>,
        start_date: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
        start_slot: Option<f64>,
        to: Option<&'a str>,
        type_: Option<types::TxGetAllType>,
    ) -> Result<ResponseValue<types::TxGetAllResponse>, Error<types::TxGetAllResponse>> {
        let url = format!("{}/transactions", self.baseurl,);
        let mut query = Vec::with_capacity(14usize);
        if let Some(v) = &end_block {
            query.push(("endBlock", v.to_string()));
        }
        if let Some(v) = &end_date {
            query.push(("endDate", v.to_string()));
        }
        if let Some(v) = &end_slot {
            query.push(("endSlot", v.to_string()));
        }
        if let Some(v) = &expand {
            query.push(("expand", v.to_string()));
        }
        if let Some(v) = &from {
            query.push(("from", v.to_string()));
        }
        if let Some(v) = &p {
            query.push(("p", v.to_string()));
        }
        if let Some(v) = &ps {
            query.push(("ps", v.to_string()));
        }
        if let Some(v) = &rollup {
            query.push(("rollup", v.to_string()));
        }
        if let Some(v) = &sort {
            query.push(("sort", v.to_string()));
        }
        if let Some(v) = &start_block {
            query.push(("startBlock", v.to_string()));
        }
        if let Some(v) = &start_date {
            query.push(("startDate", v.to_string()));
        }
        if let Some(v) = &start_slot {
            query.push(("startSlot", v.to_string()));
        }
        if let Some(v) = &to {
            query.push(("to", v.to_string()));
        }
        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**retrieves transaction details for given transaction hash

Sends a `GET` request to `/transactions/{hash}`

*/
    pub async fn tx_get_by_hash<'a>(
        &'a self,
        hash: &'a str,
        expand: Option<&'a str>,
    ) -> Result<
        ResponseValue<types::TxGetByHashResponse>,
        Error<types::TxGetByHashResponse>,
    > {
        let url = format!(
            "{}/transactions/{}", self.baseurl, encode_path(& hash.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &expand {
            query.push(("expand", v.to_string()));
        }
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**retrieves all blobs

Sends a `GET` request to `/blobs`

*/
    pub async fn blob_get_all<'a>(
        &'a self,
        end_block: Option<f64>,
        end_date: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
        end_slot: Option<f64>,
        expand: Option<&'a str>,
        from: Option<&'a str>,
        p: Option<f64>,
        ps: Option<f64>,
        rollup: Option<&'a str>,
        sort: Option<types::BlobGetAllSort>,
        start_block: Option<f64>,
        start_date: Option<&'a chrono::DateTime<chrono::offset::Utc>>,
        start_slot: Option<f64>,
        to: Option<&'a str>,
        type_: Option<types::BlobGetAllType>,
    ) -> Result<
        ResponseValue<types::BlobGetAllResponse>,
        Error<types::BlobGetAllResponse>,
    > {
        let url = format!("{}/blobs", self.baseurl,);
        let mut query = Vec::with_capacity(14usize);
        if let Some(v) = &end_block {
            query.push(("endBlock", v.to_string()));
        }
        if let Some(v) = &end_date {
            query.push(("endDate", v.to_string()));
        }
        if let Some(v) = &end_slot {
            query.push(("endSlot", v.to_string()));
        }
        if let Some(v) = &expand {
            query.push(("expand", v.to_string()));
        }
        if let Some(v) = &from {
            query.push(("from", v.to_string()));
        }
        if let Some(v) = &p {
            query.push(("p", v.to_string()));
        }
        if let Some(v) = &ps {
            query.push(("ps", v.to_string()));
        }
        if let Some(v) = &rollup {
            query.push(("rollup", v.to_string()));
        }
        if let Some(v) = &sort {
            query.push(("sort", v.to_string()));
        }
        if let Some(v) = &start_block {
            query.push(("startBlock", v.to_string()));
        }
        if let Some(v) = &start_date {
            query.push(("startDate", v.to_string()));
        }
        if let Some(v) = &start_slot {
            query.push(("startSlot", v.to_string()));
        }
        if let Some(v) = &to {
            query.push(("to", v.to_string()));
        }
        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**retrieves blob details for given versioned hash or kzg commitment

Sends a `GET` request to `/blobs/{id}`

*/
    pub async fn blob_get_by_blob_id<'a>(
        &'a self,
        id: &'a str,
        expand: Option<&'a str>,
    ) -> Result<
        ResponseValue<types::BlobGetByBlobIdResponse>,
        Error<types::BlobGetByBlobIdResponse>,
    > {
        let url = format!("{}/blobs/{}", self.baseurl, encode_path(& id.to_string()),);
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &expand {
            query.push(("expand", v.to_string()));
        }
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**retrieves all overall stats

Sends a `GET` request to `/stats/overall`

*/
    pub async fn stats_get_all_overall_stats<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::StatsGetAllOverallStatsResponse>,
        Error<types::StatsGetAllOverallStatsResponse>,
    > {
        let url = format!("{}/stats/overall", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**retrieves blob time series stats

Sends a `GET` request to `/stats/blobs`

*/
    pub async fn stats_get_blob_daily_stats<'a>(
        &'a self,
        time_frame: types::StatsGetBlobDailyStatsTimeFrame,
    ) -> Result<
        ResponseValue<types::StatsGetBlobDailyStatsResponse>,
        Error<types::StatsGetBlobDailyStatsResponse>,
    > {
        let url = format!("{}/stats/blobs", self.baseurl,);
        let mut query = Vec::with_capacity(1usize);
        query.push(("timeFrame", time_frame.to_string()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**retrieves blobs overall stats

Sends a `GET` request to `/stats/blobs/overall`

*/
    pub async fn stats_get_blob_overall_stats<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::StatsGetBlobOverallStatsResponse>,
        Error<types::StatsGetBlobOverallStatsResponse>,
    > {
        let url = format!("{}/stats/blobs/overall", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**retrieves blocks time series stats

Sends a `GET` request to `/stats/blocks`

*/
    pub async fn stats_get_block_daily_stats<'a>(
        &'a self,
        time_frame: types::StatsGetBlockDailyStatsTimeFrame,
    ) -> Result<
        ResponseValue<types::StatsGetBlockDailyStatsResponse>,
        Error<types::StatsGetBlockDailyStatsResponse>,
    > {
        let url = format!("{}/stats/blocks", self.baseurl,);
        let mut query = Vec::with_capacity(1usize);
        query.push(("timeFrame", time_frame.to_string()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**retrieves blocks overall stats

Sends a `GET` request to `/stats/blocks/overall`

*/
    pub async fn stats_get_block_overall_stats<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::StatsGetBlockOverallStatsResponse>,
        Error<types::StatsGetBlockOverallStatsResponse>,
    > {
        let url = format!("{}/stats/blocks/overall", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**retrieves transactions time series stats

Sends a `GET` request to `/stats/transactions`

*/
    pub async fn stats_get_transaction_daily_stats<'a>(
        &'a self,
        time_frame: types::StatsGetTransactionDailyStatsTimeFrame,
    ) -> Result<
        ResponseValue<types::StatsGetTransactionDailyStatsResponse>,
        Error<types::StatsGetTransactionDailyStatsResponse>,
    > {
        let url = format!("{}/stats/transactions", self.baseurl,);
        let mut query = Vec::with_capacity(1usize);
        query.push(("timeFrame", time_frame.to_string()));
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**retrieves transactions overall stats

Sends a `GET` request to `/stats/transactions/overall`

*/
    pub async fn stats_get_transaction_overall_stats<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::StatsGetTransactionOverallStatsResponse>,
        Error<types::StatsGetTransactionOverallStatsResponse>,
    > {
        let url = format!("{}/stats/transactions/overall", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**marks all blocks with slots matching the given reorged slots as reorged

Sends a `PUT` request to `/indexer/reorged-slots`

*/
    pub async fn indexer_handle_reorged_slots<'a>(
        &'a self,
        body: &'a types::IndexerHandleReorgedSlotsBody,
    ) -> Result<
        ResponseValue<types::IndexerHandleReorgedSlotsResponse>,
        Error<types::IndexerHandleReorgedSlotsResponse>,
    > {
        let url = format!("{}/indexer/reorged-slots", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**indexes data in the database

Sends a `PUT` request to `/indexer/block-txs-blobs`

*/
    pub async fn indexer_index_data<'a>(
        &'a self,
        body: &'a types::IndexerIndexDataBody,
    ) -> Result<
        ResponseValue<types::IndexerIndexDataResponse>,
        Error<types::IndexerIndexDataResponse>,
    > {
        let url = format!("{}/indexer/block-txs-blobs", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**retrieves the blockchain sync state

Sends a `GET` request to `/blockchain-sync-state`

*/
    pub async fn sync_state_get_state<'a>(
        &'a self,
    ) -> Result<
        ResponseValue<types::SyncStateGetStateResponse>,
        Error<types::SyncStateGetStateResponse>,
    > {
        let url = format!("{}/blockchain-sync-state", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**updates the blockchain sync state

Sends a `PUT` request to `/blockchain-sync-state`

*/
    pub async fn sync_state_update_state<'a>(
        &'a self,
        body: &'a types::SyncStateUpdateStateBody,
    ) -> Result<
        ResponseValue<serde_json::Value>,
        Error<types::SyncStateUpdateStateResponse>,
    > {
        let url = format!("{}/blockchain-sync-state", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
    /**connection healthcheck

Sends a `GET` request to `/healthcheck`

*/
    pub async fn healthcheck<'a>(
        &'a self,
    ) -> Result<ResponseValue<String>, Error<types::HealthcheckResponse>> {
        let url = format!("{}/healthcheck", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            200..=299 => ResponseValue::from_response(response).await,
            _ => Err(Error::ErrorResponse(ResponseValue::from_response(response).await?)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}
/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
