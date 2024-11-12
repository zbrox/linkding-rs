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
    #[error("Could not serialize JSON body")]
    JsonSerialize(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
pub enum Endpoint {
    ListBookmarks(ListArgs),
    ListArchivedBookmarks(ListArgs),
    GetBookmark(i32),
    CheckUrl(String),
    CreateBookmark,
    UpdateBookmark(i32),
}

impl TryInto<http::Uri> for Endpoint {
    type Error = LinkDingError;

    fn try_into(self) -> Result<http::Uri, Self::Error> {
        match self.clone() {
            Self::ListBookmarks(args) | Self::ListArchivedBookmarks(args) => {
                let query_string = args.query_string();
                let path: String = self.into();
                http::Uri::try_from(format!("{}{}", path, query_string))
                    .map_err(LinkDingError::InvalidUrl)
            }
            Self::GetBookmark(id) | Self::UpdateBookmark(id) => {
                let path: String = self.into();
                http::Uri::try_from(format!("{}{}", path, id)).map_err(LinkDingError::InvalidUrl)
            }
            Self::CheckUrl(url) => {
                let path: String = self.into();
                http::Uri::try_from(format!("{}?url={}", path, url))
                    .map_err(LinkDingError::InvalidUrl)
            }
            Self::CreateBookmark => {
                let path: String = self.into();
                http::Uri::try_from(path).map_err(LinkDingError::InvalidUrl)
            }
        }
    }
}

impl QueryString for Endpoint {
    fn query_string(&self) -> String {
        match self {
            Self::ListBookmarks(args) => args.query_string(),
            Self::ListArchivedBookmarks(args) => args.query_string(),
            Self::GetBookmark(_)
            | Self::CheckUrl(_)
            | Self::CreateBookmark
            | Self::UpdateBookmark(_) => "".to_string(),
        }
    }
}

impl Into<String> for Endpoint {
    fn into(self) -> String {
        match &self {
            Self::ListBookmarks(_) => "/api/bookmarks/".to_string(),
            Self::ListArchivedBookmarks(_) => "/api/bookmarks/archived/".to_string(),
            Self::GetBookmark(id) | Self::UpdateBookmark(id) => {
                format!("/api/bookmarks/{}/", &id)
            }
            Self::CheckUrl(_) => "/api/bookmarks/check/".to_string(),
            Self::CreateBookmark => "/api/bookmarks/".to_string(),
        }
    }
}

impl Into<http::Method> for Endpoint {
    fn into(self) -> http::Method {
        match self {
            Self::ListBookmarks(_) => http::Method::GET,
            Self::ListArchivedBookmarks(_) => http::Method::GET,
            Self::GetBookmark(_) => http::Method::GET,
            Self::CheckUrl(_) => http::Method::GET,
            Self::CreateBookmark => http::Method::POST,
            Self::UpdateBookmark(_) => http::Method::PATCH,
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

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct PageMetadata {
    pub url: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub preview_image: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct CheckUrlResponse {
    pub bookmark: Option<Bookmark>,
    pub metadata: PageMetadata,
    pub auto_tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct CreateBookmarkBody {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_archive_snapshot_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_added: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct UpdateBookmarkBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_archive_snapshot_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unread: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_added: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_modified: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website_description: Option<String>,
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

    pub fn get_bookmark(&self, id: i32) -> Result<Bookmark, LinkDingError> {
        let endpoint = Endpoint::GetBookmark(id);
        let request = self.prepare_request(endpoint)?.body(())?;
        let body: Bookmark = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    pub fn check_url(&self, url: &str) -> Result<CheckUrlResponse, LinkDingError> {
        let endpoint = Endpoint::CheckUrl(url.to_string());
        let request = self.prepare_request(endpoint)?.body(())?;
        let body: CheckUrlResponse = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    pub fn create_bookmark(&self, body: CreateBookmarkBody) -> Result<Bookmark, LinkDingError> {
        let endpoint = Endpoint::CreateBookmark;
        let request = self
            .prepare_request(endpoint)?
            .body(serde_json::to_string(&body)?)?;
        let body: Bookmark = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    pub fn update_bookmark(
        &self,
        id: i32,
        body: UpdateBookmarkBody,
    ) -> Result<Bookmark, LinkDingError> {
        let endpoint = Endpoint::UpdateBookmark(id);
        let request = self
            .prepare_request(endpoint)?
            .body(serde_json::to_string(&body)?)?;
        let body: Bookmark = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }
}
