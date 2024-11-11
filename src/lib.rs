#[cfg(feature = "ffi")]
uniffi::setup_scaffolding!();

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
#[cfg_attr(feature = "ffi", derive(uniffi::Error))]
#[cfg_attr(feature = "ffi", uniffi(flat_error))]
pub enum LinkDingError {
    #[error("Error parsing URL")]
    ParseUrlError(#[from] http::uri::InvalidUri),
    #[error("Error sending HTTP request")]
    SendHttpError(#[from] ureq::Error),
    #[error("Could not parse JSON response from API")]
    JsonParse(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
pub enum Endpoint {
    ListBookmarks,
    ListArchivedBookmarks,
}

impl<'a> Into<&'a str> for Endpoint {
    fn into(self) -> &'a str {
        match self {
            Self::ListBookmarks => "/api/bookmarks/",
            Self::ListArchivedBookmarks => "/api/bookmarks/archived/",
        }
    }
}

impl Into<http::Method> for Endpoint {
    fn into(self) -> http::Method {
        match self {
            Self::ListBookmarks => http::Method::GET,
            Self::ListArchivedBookmarks => http::Method::GET,
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
pub struct ListBookmarksRespone {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<Bookmark>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(uniffi::Object))]
pub struct LinkDingClient {
    token: String,
    url: String,
}

impl LinkDingClient {
    fn prepare_request(&self, endpoint: Endpoint) -> ureq::Request {
        let method: http::Method = endpoint.clone().into();
        let url = self.url.to_string() + endpoint.into();
        ureq::request(method.as_str(), &url)
            .set("Authorization", &format!("Token {}", &self.token))
            .set("Accept", "application/json")
    }
}

#[cfg_attr(feature = "ffi", uniffi::export)]
impl LinkDingClient {
    #[cfg_attr(feature = "ffi", uniffi::constructor)]
    pub fn new(url: String, token: String) -> Self {
        LinkDingClient { token, url }
    }

    pub fn list_bookmarks(&self) -> Result<ListBookmarksRespone, LinkDingError> {
        let response = self.prepare_request(Endpoint::ListBookmarks).call()?;
        let body: ListBookmarksRespone = response.into_json()?;
        Ok(body)
    }
}
