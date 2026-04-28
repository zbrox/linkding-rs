#[cfg(feature = "ffi")]
uniffi::setup_scaffolding!();

#[cfg(not(any(feature = "rustls-tls", feature = "native-tls")))]
compile_error!(
    "linkding-rs requires one of the `rustls-tls` or `native-tls` features to be enabled"
);

pub mod bookmark_assets;
pub mod bookmarks;
pub mod tags;
pub mod users;

#[cfg(feature = "blocking")]
mod sync_client;
#[cfg(feature = "blocking")]
pub use sync_client::LinkDingClient;

pub use bookmarks::{
    Bookmark, CheckUrlResponse, CreateBookmarkBody, ListBookmarksArgs, ListBookmarksResponse,
    UpdateBookmarkBody,
};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
pub use tags::{ListTagsArgs, ListTagsResponse, TagData};
use thiserror::Error;
pub use users::{DateDisplay, LinkTarget, SelectedTheme, SortBy, TagSearchMethod, UserProfile};

#[derive(Error, Debug)]
#[cfg_attr(feature = "ffi", derive(uniffi::Error))]
#[cfg_attr(feature = "ffi", uniffi(flat_error))]
pub enum LinkDingError {
    #[error("Error building URL")]
    ParseUrl(url::ParseError),
    #[error("Error sending HTTP request")]
    SendHttpError(#[from] reqwest::Error),
    #[error("Could not serialize JSON body")]
    JsonSerialize(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
pub enum Endpoint {
    ListBookmarks(ListBookmarksArgs),
    ListArchivedBookmarks(ListBookmarksArgs),
    GetBookmark(i32),
    CheckUrl(String),
    CreateBookmark,
    UpdateBookmark(i32),
    ArchiveBookmark(i32),
    UnarchiveBookmark(i32),
    DeleteBookmark(i32),
    ListTags(ListTagsArgs),
    GetTag(i32),
    CreateTag,
    GetUserProfile,
    ListBookmarkAssets(i32),
    RetrieveBookmarkAsset(i32, i32),
    DownloadBookmarkAsset(i32, i32),
    UploadBookmarkAsset(i32),
    DeleteBookmarkAsset(i32, i32),
}

impl QueryString for Endpoint {
    fn query_string(&self) -> String {
        match self {
            Self::ListBookmarks(args) | Self::ListArchivedBookmarks(args) => args.query_string(),
            Self::ListTags(args) => args.query_string(),
            Self::GetBookmark(_)
            | Self::CheckUrl(_)
            | Self::CreateBookmark
            | Self::UpdateBookmark(_)
            | Self::ArchiveBookmark(_)
            | Self::UnarchiveBookmark(_)
            | Self::DeleteBookmark(_)
            | Self::GetTag(_)
            | Self::CreateTag
            | Self::GetUserProfile
            | Self::ListBookmarkAssets(_)
            | Self::RetrieveBookmarkAsset(_, _)
            | Self::DownloadBookmarkAsset(_, _)
            | Self::UploadBookmarkAsset(_)
            | Self::DeleteBookmarkAsset(_, _) => "".to_string(),
        }
    }
}

impl From<Endpoint> for String {
    fn from(val: Endpoint) -> Self {
        let path = match &val {
            Endpoint::ListBookmarks(_) => "/api/bookmarks/".to_string(),
            Endpoint::ListArchivedBookmarks(_) => "/api/bookmarks/archived/".to_string(),
            Endpoint::GetBookmark(id)
            | Endpoint::UpdateBookmark(id)
            | Endpoint::DeleteBookmark(id) => {
                format!("/api/bookmarks/{}/", &id)
            }
            Endpoint::CheckUrl(_) => "/api/bookmarks/check/".to_string(),
            Endpoint::CreateBookmark => "/api/bookmarks/".to_string(),
            Endpoint::ArchiveBookmark(id) => format!("/api/bookmarks/{}/archive/", &id),
            Endpoint::UnarchiveBookmark(id) => format!("/api/bookmarks/{}/unarchive/", &id),
            Endpoint::ListTags(_) | Endpoint::CreateTag => "/api/tags/".to_string(),
            Endpoint::GetTag(id) => format!("/api/tags/{}/", &id),
            Endpoint::GetUserProfile => "/api/user/profile/".to_string(),
            Endpoint::ListBookmarkAssets(id) => format!("/api/bookmarks/{}/assets/", id),
            Endpoint::RetrieveBookmarkAsset(bookmark_id, asset_id)
            | Endpoint::DeleteBookmarkAsset(bookmark_id, asset_id) => {
                format!("/api/bookmarks/{}/assets/{}/", bookmark_id, asset_id)
            }
            Endpoint::DownloadBookmarkAsset(bookmark_id, asset_id) => format!(
                "/api/bookmarks/{}/assets/{}/download/",
                bookmark_id, asset_id
            ),
            Endpoint::UploadBookmarkAsset(id) => format!("/api/bookmarks/{}/assets/upload/", id),
        };
        match &val {
            Endpoint::ListBookmarks(args) | Endpoint::ListArchivedBookmarks(args) => {
                let query_string = args.query_string();
                format!("{}?{}", path, query_string)
            }
            Endpoint::ListTags(args) => {
                let query_string = args.query_string();
                format!("{}?{}", path, query_string)
            }
            Endpoint::CheckUrl(url) => {
                format!("{}?url={}", path, url)
            }
            Endpoint::GetBookmark(_)
            | Endpoint::UpdateBookmark(_)
            | Endpoint::ArchiveBookmark(_)
            | Endpoint::UnarchiveBookmark(_)
            | Endpoint::DeleteBookmark(_)
            | Endpoint::CreateBookmark
            | Endpoint::GetTag(_)
            | Endpoint::CreateTag
            | Endpoint::GetUserProfile
            | Endpoint::ListBookmarkAssets(_)
            | Endpoint::RetrieveBookmarkAsset(_, _)
            | Endpoint::DownloadBookmarkAsset(_, _)
            | Endpoint::UploadBookmarkAsset(_)
            | Endpoint::DeleteBookmarkAsset(_, _) => path,
        }
    }
}

impl From<Endpoint> for reqwest::Method {
    fn from(val: Endpoint) -> Self {
        match val {
            Endpoint::ListBookmarks(_) => reqwest::Method::GET,
            Endpoint::ListArchivedBookmarks(_) => reqwest::Method::GET,
            Endpoint::GetBookmark(_) => reqwest::Method::GET,
            Endpoint::CheckUrl(_) => reqwest::Method::GET,
            Endpoint::CreateBookmark => reqwest::Method::POST,
            Endpoint::UpdateBookmark(_) => reqwest::Method::PATCH,
            Endpoint::ArchiveBookmark(_) => reqwest::Method::POST,
            Endpoint::UnarchiveBookmark(_) => reqwest::Method::POST,
            Endpoint::DeleteBookmark(_) => reqwest::Method::DELETE,
            Endpoint::ListTags(_) => reqwest::Method::GET,
            Endpoint::GetTag(_) => reqwest::Method::GET,
            Endpoint::CreateTag => reqwest::Method::POST,
            Endpoint::GetUserProfile => reqwest::Method::GET,
            Endpoint::ListBookmarkAssets(_) => reqwest::Method::GET,
            Endpoint::RetrieveBookmarkAsset(_, _) => reqwest::Method::GET,
            Endpoint::DownloadBookmarkAsset(_, _) => reqwest::Method::GET,
            Endpoint::UploadBookmarkAsset(_) => reqwest::Method::POST,
            Endpoint::DeleteBookmarkAsset(_, _) => reqwest::Method::DELETE,
        }
    }
}

impl From<Endpoint> for reqwest::header::HeaderMap {
    fn from(val: Endpoint) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        match val {
            Endpoint::ListBookmarks(_)
            | Endpoint::ListArchivedBookmarks(_)
            | Endpoint::GetBookmark(_)
            | Endpoint::CheckUrl(_)
            | Endpoint::CreateBookmark
            | Endpoint::UpdateBookmark(_)
            | Endpoint::ArchiveBookmark(_)
            | Endpoint::UnarchiveBookmark(_)
            | Endpoint::DeleteBookmark(_)
            | Endpoint::ListTags(_)
            | Endpoint::GetTag(_)
            | Endpoint::CreateTag
            | Endpoint::GetUserProfile
            | Endpoint::ListBookmarkAssets(_)
            | Endpoint::RetrieveBookmarkAsset(_, _)
            | Endpoint::UploadBookmarkAsset(_)
            | Endpoint::DeleteBookmarkAsset(_, _) => {
                headers.insert(
                    CONTENT_TYPE,
                    "application/json"
                        .parse()
                        .expect("Could not parse content type header value"),
                );
                headers.insert(
                    ACCEPT,
                    "application/json"
                        .parse()
                        .expect("Could not parse accept header value"),
                );
            }
            Endpoint::DownloadBookmarkAsset(_, _) => {
                headers.insert(ACCEPT, reqwest::header::HeaderValue::from_static("*/*"));
            }
        };
        headers
    }
}

trait QueryString {
    fn query_string(&self) -> String;
}

pub(crate) struct RequestSpec {
    pub url: reqwest::Url,
    pub method: reqwest::Method,
    pub headers: reqwest::header::HeaderMap,
}

pub(crate) fn build_request_spec(
    base_url: &str,
    token: &str,
    endpoint: Endpoint,
) -> Result<RequestSpec, LinkDingError> {
    let base: reqwest::Url = base_url.parse().map_err(LinkDingError::ParseUrl)?;
    let path_and_query: String = endpoint.clone().into();
    let url = base
        .join(&path_and_query)
        .map_err(LinkDingError::ParseUrl)?;
    let method: reqwest::Method = endpoint.clone().into();
    let mut headers: reqwest::header::HeaderMap = endpoint.into();
    headers.insert(
        AUTHORIZATION,
        format!("Token {}", token)
            .parse()
            .expect("Could not parse authorization header value"),
    );
    Ok(RequestSpec {
        url,
        method,
        headers,
    })
}
