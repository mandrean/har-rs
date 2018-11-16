use errors;
use MINIMUM_HAR12_VERSION;

impl Spec {
    pub fn validate_version(&self) -> errors::Result<semver::Version> {
        let spec_version = &self.log.version;
        let sem_ver = semver::Version::parse(spec_version)?;
        let required_version = semver::VersionReq::parse(MINIMUM_HAR12_VERSION).unwrap();
        if required_version.matches(&sem_ver) {
            Ok(sem_ver)
        } else {
            Err(errors::ErrorKind::UnsupportedSpecFileVersion(sem_ver))?
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Spec {
    pub log: Log,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Log {
    pub version: String,
    pub creator: Creator,
    pub browser: Option<Creator>,
    pub pages: Option<Vec<Pages>>,
    pub entries: Vec<Entries>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Creator {
    pub name: String,
    pub version: String,
    pub comment: Option<String>,
}

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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct PageTimings {
    #[serde(rename = "onContentLoad")]
    pub on_content_load: Option<i64>,
    #[serde(rename = "onLoad")]
    pub on_load: Option<i64>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Entries {
    pub pageref: Option<String>,
    #[serde(rename = "startedDateTime")]
    pub started_date_time: String,
    pub time: i64,
    pub request: Request,
    pub response: Response,
    pub cache: Cache,
    pub timings: Timings,
    #[serde(rename = "serverIPAddress")]
    pub server_ip_address: Option<String>,
    pub connection: Option<String>,
    pub comment: Option<String>,
}

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
    #[serde(rename = "headersSize")]
    pub headers_size: i64,
    #[serde(rename = "bodySize")]
    pub body_size: i64,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Headers {
    pub name: String,
    pub value: String,
    pub comment: Option<String>,
}

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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct QueryString {
    pub name: String,
    pub value: String,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct PostData {
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub text: String,
    pub params: Option<Vec<Params>>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Params {
    pub name: String,
    pub value: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Response {
    #[serde(rename = "_charlesStatus")]
    pub charles_status: Option<String>,
    pub status: i64,
    #[serde(rename = "statusText")]
    pub status_text: String,
    #[serde(rename = "httpVersion")]
    pub http_version: String,
    pub cookies: Vec<Cookies>,
    pub headers: Vec<Headers>,
    pub content: Content,
    #[serde(rename = "redirectURL")]
    pub redirect_url: String,
    #[serde(rename = "headersSize")]
    pub headers_size: i64,
    #[serde(rename = "bodySize")]
    pub body_size: i64,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Content {
    pub size: i64,
    pub compression: Option<i64>,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub text: Option<String>,
    pub encoding: Option<String>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Cache {
    #[serde(rename = "beforeRequest")]
    pub before_request: Option<CacheEntity>,
    #[serde(rename = "afterRequest")]
    pub after_request: Option<CacheEntity>,
}

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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Timings {
    pub blocked: Option<i64>,
    pub dns: Option<i64>,
    pub connect: Option<i64>,
    pub send: i64,
    pub wait: i64,
    pub receive: i64,
    pub ssl: Option<i64>,
    pub comment: Option<String>,
}
