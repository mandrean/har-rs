//! HTTP Archive (HAR) serialization and deserialization helpers.
//!
//! The crate accepts HAR input as JSON and can serialize parsed documents back to
//! JSON or YAML.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub mod v1_2;
pub mod v1_3;

/// Errors that HAR functions may return.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to read HAR input")]
    Read {
        #[source]
        source: std::io::Error,
    },
    #[error("failed to decode HAR JSON")]
    DecodeJson {
        #[source]
        source: serde_json::Error,
    },
    #[error("failed to encode HAR as YAML")]
    EncodeYaml {
        #[source]
        source: serde_yaml::Error,
    },
    #[error("failed to encode HAR as JSON")]
    EncodeJson {
        #[source]
        source: serde_json::Error,
    },
    #[error("HAR document must contain a top-level `log` object")]
    MissingLog,
    #[error("HAR document is missing `log.version`")]
    MissingVersion,
    #[error("unsupported HAR version `{0}`")]
    UnsupportedVersion(String),
}

impl Error {
    fn read(source: std::io::Error) -> Self {
        Self::Read { source }
    }

    fn decode_json(source: serde_json::Error) -> Self {
        Self::DecodeJson { source }
    }

    fn encode_yaml(source: serde_yaml::Error) -> Self {
        Self::EncodeYaml { source }
    }

    fn encode_json(source: serde_json::Error) -> Self {
        Self::EncodeJson { source }
    }
}

/// Supported HAR versions.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HarVersion {
    V1_2,
    V1_3,
}

impl HarVersion {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::V1_2 => "1.2",
            Self::V1_3 => "1.3",
        }
    }
}

impl fmt::Display for HarVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Supported versions of HAR.
///
/// Note that point releases require adding here (as they must otherwise they
/// wouldn't need a new version). Using `untagged` can avoid that but the errors
/// on incompatible documents become super hard to debug.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(tag = "version")]
pub enum Spec {
    /// Version 1.2 of the HAR specification.
    ///
    /// Refer to the official
    /// [specification](https://w3c.github.io/web-performance/specs/HAR/Overview.html)
    /// for more information.
    #[allow(non_camel_case_types)]
    #[serde(rename = "1.2")]
    V1_2(v1_2::Log),

    /// Version 1.3 of the HAR specification.
    ///
    /// Refer to the draft
    /// [specification](https://github.com/ahmadnassri/har-spec/blob/master/versions/1.3.md)
    /// for more information.
    #[allow(non_camel_case_types)]
    #[serde(rename = "1.3")]
    V1_3(v1_3::Log),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Har {
    pub log: Spec,
}

impl Har {
    pub fn version(&self) -> HarVersion {
        match &self.log {
            Spec::V1_2(_) => HarVersion::V1_2,
            Spec::V1_3(_) => HarVersion::V1_3,
        }
    }
}

/// Deserialize a HAR from a path.
pub fn from_path<P>(path: P) -> Result<Har, Error>
where
    P: AsRef<Path>,
{
    from_reader(File::open(path).map_err(Error::read)?)
}

/// Deserialize a HAR from a byte slice.
pub fn from_slice(input: &[u8]) -> Result<Har, Error> {
    let value = serde_json::from_slice::<serde_json::Value>(input).map_err(Error::decode_json)?;
    parse_har_value(value)
}

/// Deserialize a HAR from a string slice.
///
/// ```
/// use har::{from_str, to_json, HarVersion};
///
/// let input = r#"{
///   "log": {
///     "version": "1.2",
///     "creator": { "name": "example", "version": "1.0" },
///     "entries": []
///   }
/// }"#;
///
/// let har = from_str(input)?;
/// assert_eq!(har.version(), HarVersion::V1_2);
/// assert!(to_json(&har)?.contains("\"version\": \"1.2\""));
/// # Ok::<(), har::Error>(())
/// ```
pub fn from_str(input: &str) -> Result<Har, Error> {
    from_slice(input.as_bytes())
}

/// Deserialize a HAR from a type which implements `Read`.
pub fn from_reader<R>(mut reader: R) -> Result<Har, Error>
where
    R: Read,
{
    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).map_err(Error::read)?;
    from_slice(&bytes)
}

/// Serialize a HAR to a YAML string.
pub fn to_yaml(spec: &Har) -> Result<String, Error> {
    serde_yaml::to_string(spec).map_err(Error::encode_yaml)
}

/// Serialize a HAR to a JSON string.
pub fn to_json(spec: &Har) -> Result<String, Error> {
    serde_json::to_string_pretty(spec).map_err(Error::encode_json)
}

fn parse_har_value(value: serde_json::Value) -> Result<Har, Error> {
    let root = value.as_object().ok_or(Error::MissingLog)?;
    let log_value = root.get("log").cloned().ok_or(Error::MissingLog)?;

    let Some(log_object) = log_value.as_object() else {
        return Err(Error::MissingLog);
    };

    let version = match log_object
        .get("version")
        .and_then(serde_json::Value::as_str)
    {
        Some("1.2") => HarVersion::V1_2,
        Some("1.3") => HarVersion::V1_3,
        Some(other) => return Err(Error::UnsupportedVersion(other.to_owned())),
        None => return Err(Error::MissingVersion),
    };

    let log = match version {
        HarVersion::V1_2 => {
            let log = serde_json::from_value(log_value).map_err(Error::decode_json)?;
            Spec::V1_2(log)
        }
        HarVersion::V1_3 => {
            let log = serde_json::from_value(log_value).map_err(Error::decode_json)?;
            Spec::V1_3(log)
        }
    };

    Ok(Har { log })
}
