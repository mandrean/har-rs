use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Log {
    pub creator: Creator,
    pub browser: Option<Creator>,
    pub pages: Option<Vec<Pages>>,
    pub entries: Vec<Entries>,
    pub comment: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Creator {
    pub name: String,
    pub version: String,
    pub comment: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Pages {
    #[serde(rename = "startedDateTime")]
    pub started_date_time: String,
    pub id: String,
    pub title: String,
    #[serde(rename = "pageTimings")]
    pub page_timings: PageTimings,
    pub comment: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct PageTimings {
    #[serde(rename = "onContentLoad", default = "default_fsize_maybe")]
    pub on_content_load: Option<f64>,
    #[serde(rename = "onLoad", default = "default_fsize_maybe")]
    pub on_load: Option<f64>,
    pub comment: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Entries {
    pub pageref: Option<String>,
    #[serde(rename = "startedDateTime")]
    pub started_date_time: String,
    pub time: f64,
    pub request: Request,
    pub response: Response,
    pub cache: Cache,
    pub timings: Timings,
    #[serde(rename = "serverIPAddress")]
    pub server_ip_address: Option<String>,
    pub connection: Option<String>,
    pub comment: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Request {
    pub method: String,
    pub url: String,
    #[serde(rename = "httpVersion")]
    pub http_version: String,
    pub cookies: Vec<Cookies>,
    pub headers: Vec<Headers>,
    #[serde(rename = "queryString")]
    pub query_string: Vec<QueryString>,
    #[serde(rename = "postData")]
    pub post_data: Option<PostData>,
    #[serde(rename = "headersSize", deserialize_with = "de_default_isize")]
    pub headers_size: i64,
    #[serde(rename = "bodySize", default = "default_isize")]
    pub body_size: i64,
    pub comment: Option<String>,
    #[serde(rename = "headersCompression")]
    pub headers_compression: Option<i64>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Headers {
    pub name: String,
    pub value: String,
    pub comment: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Cookies {
    pub name: String,
    pub value: String,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub expires: Option<String>,
    #[serde(rename = "httpOnly")]
    pub http_only: Option<bool>,
    pub secure: Option<bool>,
    pub comment: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct QueryString {
    pub name: String,
    pub value: String,
    pub comment: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct PostData {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    /// Either text or params but not both : TODO turn into an untagged enum
    pub text: Option<String>,
    pub params: Option<Vec<Params>>,
    pub comment: Option<String>,
    pub encoding: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Params {
    pub name: String,
    pub value: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    pub comment: Option<String>,
    pub encoding: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Response {
    pub status: i64,
    #[serde(rename = "statusText")]
    pub status_text: String,
    #[serde(rename = "httpVersion")]
    pub http_version: String,
    pub cookies: Vec<Cookies>,
    pub headers: Vec<Headers>,
    pub content: Content,
    #[serde(rename = "redirectURL")]
    pub redirect_url: Option<String>,
    #[serde(rename = "headersSize", default = "default_isize")]
    pub headers_size: i64,
    #[serde(rename = "bodySize", default = "default_isize")]
    pub body_size: i64,
    pub comment: Option<String>,
    #[serde(rename = "headersCompression")]
    pub headers_compression: Option<i64>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Content {
    #[serde(default = "default_isize")]
    pub size: i64,
    pub compression: Option<i64>,
    #[serde(rename = "mimeType")]
    pub mime_type: Option<String>,
    pub text: Option<String>,
    pub encoding: Option<String>,
    pub comment: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Cache {
    #[serde(rename = "beforeRequest")]
    pub before_request: Option<CacheEntity>,
    #[serde(rename = "afterRequest")]
    pub after_request: Option<CacheEntity>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct CacheEntity {
    pub expires: Option<String>,
    #[serde(rename = "lastAccess")]
    pub last_access: String,
    #[serde(rename = "eTag")]
    pub e_tag: String,
    #[serde(rename = "hitCount")]
    pub hit_count: i64,
    pub comment: Option<String>,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Timings {
    #[serde(default = "default_fsize_maybe")]
    pub blocked: Option<f64>,
    #[serde(default = "default_fsize_maybe")]
    pub dns: Option<f64>,
    #[serde(default = "default_fsize_maybe")]
    pub connect: Option<f64>,
    pub send: f64,
    pub wait: f64,
    pub receive: f64,
    #[serde(default = "default_fsize_maybe")]
    pub ssl: Option<f64>,
    pub comment: Option<String>,
}

fn de_default_isize<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Ok(Option::<i64>::deserialize(deserializer)?.unwrap_or(-1))
}

fn default_isize() -> i64 {
    -1
}

fn default_fsize_maybe() -> Option<f64> {
    Some(-1_f64)
}
