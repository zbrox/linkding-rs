#[cfg(feature = "ffi")]
uniffi::setup_scaffolding!();

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
#[cfg_attr(feature = "ffi", derive(uniffi::Error))]
#[cfg_attr(feature = "ffi", uniffi(flat_error))]
pub enum LinkDingError {
    #[error("Invalid URL")]
    InvalidUrl(#[from] http::uri::InvalidUri),
    #[error("Error building URL")]
    ParseUrlError(#[from] http::Error),
    #[error("Error sending HTTP request")]
    SendHttpError(#[from] ureq::Error),
    #[error("Could not parse JSON response from API")]
    JsonParse(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
pub enum Endpoint {
    ListBookmarks(ListArgs),
    ListArchivedBookmarks(ListArgs),
}

impl TryInto<http::Uri> for Endpoint {
    type Error = LinkDingError;

    fn try_into(self) -> Result<http::Uri, Self::Error> {
        match &self {
            Self::ListBookmarks(args) => {
                let query_string = args.query_string();
                let path: &str = self.into();
                http::Uri::try_from(format!("{}{}", path, query_string))
                    .map_err(LinkDingError::InvalidUrl)
            }
            Self::ListArchivedBookmarks(args) => {
                let query_string = args.query_string();
                let path: &str = self.into();
                http::Uri::try_from(format!("{}{}", path, query_string))
                    .map_err(LinkDingError::InvalidUrl)
            }
        }
    }
}

impl QueryString for Endpoint {
    fn query_string(&self) -> String {
        match self {
            Self::ListBookmarks(args) => args.query_string(),
            Self::ListArchivedBookmarks(args) => args.query_string(),
        }
    }
}

impl<'a> Into<&'a str> for Endpoint {
    fn into(self) -> &'a str {
        match self {
            Self::ListBookmarks(_) => "/api/bookmarks/",
            Self::ListArchivedBookmarks(_) => "/api/bookmarks/archived/",
        }
    }
}

impl Into<http::Method> for Endpoint {
    fn into(self) -> http::Method {
        match self {
            Self::ListBookmarks(_) => http::Method::GET,
            Self::ListArchivedBookmarks(_) => http::Method::GET,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct Bookmark {
    pub id: i32,
    pub url: String,
    pub title: String,
    pub description: String,
    pub notes: String,
    pub web_archive_snapshot_url: String,
    pub favicon_url: Option<String>,
    pub preview_image_url: Option<String>,
    pub is_archived: bool,
    pub unread: bool,
    pub shared: bool,
    pub tag_names: Vec<String>,
    pub date_added: String,
    pub date_modified: String,
    pub website_title: Option<String>,
    pub website_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct ListBookmarksResponse {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Bookmark>,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct ListArgs {
    query: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
}

trait QueryString {
    fn query_string(&self) -> String;
}

impl QueryString for ListArgs {
    fn query_string(&self) -> String {
        vec![
            ("q", self.query.as_ref().map(|v| v.to_string())),
            ("limit", self.limit.as_ref().map(|v| v.to_string())),
            ("offset", self.offset.as_ref().map(|v| v.to_string())),
        ]
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|v| format!("{}={}", k, v)))
        .collect::<Vec<_>>()
        .join("&")
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(uniffi::Object))]
pub struct LinkDingClient {
    token: String,
    url: String,
}

impl LinkDingClient {
    fn prepare_request(&self, endpoint: Endpoint) -> Result<http::request::Builder, LinkDingError> {
        let uri: http::Uri = self.url.parse()?;
        let path_and_query_uri: http::Uri = endpoint.clone().try_into()?;
        let uri = http::uri::Builder::from(uri)
            .path_and_query(path_and_query_uri.to_string())
            .build()
            .map_err(LinkDingError::ParseUrlError)?;

        Ok(http::request::Builder::new()
            .method(endpoint)
            .uri(uri)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", &format!("Token {}", &self.token)))
    }
}

#[cfg_attr(feature = "ffi", uniffi::export)]
impl LinkDingClient {
    #[cfg_attr(feature = "ffi", uniffi::constructor)]
    pub fn new(url: String, token: String) -> Self {
        LinkDingClient { token, url }
    }

    pub fn list_bookmarks(&self, args: ListArgs) -> Result<ListBookmarksResponse, LinkDingError> {
        let endpoint = Endpoint::ListBookmarks(args);
        let request = self.prepare_request(endpoint)?.body(())?;
        let body: ListBookmarksResponse = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    pub fn list_archived_bookmarks(
        &self,
        args: ListArgs,
    ) -> Result<ListBookmarksResponse, LinkDingError> {
        let endpoint = Endpoint::ListArchivedBookmarks(args);
        let request = self.prepare_request(endpoint)?.body(())?;
        let body: ListBookmarksResponse = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }
}
