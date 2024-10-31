use serde::{Deserialize, Serialize};

use crate::LinkDingError;

#[derive(Debug, Clone)]
enum Endpoint {
    ListBookmarks,
    ListArchivedBookmarks,
}

impl<'a> Into<&'a str> for Endpoint {
    fn into(self) -> &'a str {
        match self {
            Self::ListBookmarks => "/api/bookmarks",
            Self::ListArchivedBookmarks => "/api/bookmarks/archived",
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
pub struct Bookmark {
    id: i32,
    #[serde(with = "http_serde::uri")]
    url: http::Uri,
    title: String,
    description: String,
    notes: String,
    #[serde(with = "http_serde::uri")]
    web_archive_snapshot_url: http::Uri,
    #[serde(with = "http_serde::uri")]
    favicon_url: http::Uri,
    #[serde(with = "http_serde::uri")]
    preview_image_url: http::Uri,
    is_archived: bool,
    unread: bool,
    shared: bool,
    tag_names: Vec<String>,
    date_added: String,
    date_modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListBookmarksRespone {
    count: i32,
    #[serde(with = "http_serde::option::uri")]
    next: Option<http::Uri>,
    #[serde(with = "http_serde::option::uri")]
    previous: Option<http::Uri>,
    results: Vec<Bookmark>,
}

#[derive(Debug)]
pub struct LinkDingClient {
    token: String,
    url: http::uri::Uri,
}

impl LinkDingClient {
    pub fn new(url: http::uri::Uri, token: String) -> Self {
        LinkDingClient { token, url }
    }

    fn prepare_request(self, endpoint: Endpoint) -> ureq::Request {
        let method: http::Method = endpoint.clone().into();
        ureq::request(method.as_str(), endpoint.into())
            .set("Authorization", &format!("Token {}", &self.token))
            .set("Accept", "application/json")
    }

    pub fn list_bookmarks(self) -> Result<ListBookmarksRespone, LinkDingError> {
        let response = self.prepare_request(Endpoint::ListBookmarks).call()?;
        let body: ListBookmarksRespone = response.into_json()?;
        Ok(body)
    }
}
