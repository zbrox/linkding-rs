#[cfg(feature = "ffi")]
uniffi::setup_scaffolding!();

pub mod bookmark_assets;
pub mod bookmarks;
pub mod tags;
pub mod users;

use std::io::Read;

use bookmark_assets::{BookmarkAsset, ListBookmarkAssetsResponse};
pub use bookmarks::{
    Bookmark, CheckUrlResponse, CreateBookmarkBody, ListBookmarksArgs, ListBookmarksResponse,
    UpdateBookmarkBody,
};
use reqwest::{
    blocking::multipart::Part,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    StatusCode,
};
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
    #[error("Could not parse response from API")]
    ParseResponse(#[from] std::io::Error),
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
            Endpoint::DownloadBookmarkAsset(_, _) => {}
        };
        headers
    }
}

trait QueryString {
    fn query_string(&self) -> String;
}

/// A sync client for the LinkDing API.
///
/// This client is used to interact with the LinkDing API. It provides methods for
/// managing bookmarks and tags, the full capability of the LinkDing API.
///
/// # Example
///
/// ```
/// use linkding::{LinkDingClient, LinkDingError, CreateBookmarkBody};
///
/// fn main() -> Result<(), LinkDingError> {
///     let client = LinkDingClient::new("https://linkding.local:9090", "YOUR_API_TOKEN");
///     let new_bookmark = CreateBookmarkBody {
///         url: "https://example.com".to_string(),
///         ..Default::default()
///     };
///     let bookmark = client.create_bookmark(new_bookmark)?;
///     println!("Bookmark created: {:?}", bookmark);
///     client.delete_bookmark(bookmark.id)?;
///     println!("Bookmark deleted");
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(uniffi::Object))]
pub struct LinkDingClient {
    token: String,
    url: String,
    client: reqwest::blocking::Client,
}

impl LinkDingClient {
    fn prepare_request(
        &self,
        endpoint: Endpoint,
    ) -> Result<reqwest::blocking::RequestBuilder, LinkDingError> {
        let base_url: reqwest::Url = self.url.parse().map_err(LinkDingError::ParseUrl)?;
        let path_and_query: String = endpoint.clone().into();
        let url = base_url
            .join(&path_and_query)
            .map_err(LinkDingError::ParseUrl)?;
        let method: reqwest::Method = endpoint.clone().into();
        let mut endpoint_headers: reqwest::header::HeaderMap = endpoint.clone().into();
        endpoint_headers.insert(
            AUTHORIZATION,
            format!("Token {}", &self.token)
                .parse()
                .expect("Could not parse authorization header value"),
        );

        let builder = self.client.request(method, url).headers(endpoint_headers);

        Ok(builder)
    }
}

#[cfg_attr(feature = "ffi", uniffi::export)]
impl LinkDingClient {
    #[cfg_attr(feature = "ffi", uniffi::constructor)]
    pub fn new(url: &str, token: &str) -> Self {
        LinkDingClient {
            token: token.to_string(),
            url: url.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    /// List unarchived bookmarks
    pub fn list_bookmarks(
        &self,
        args: ListBookmarksArgs,
    ) -> Result<ListBookmarksResponse, LinkDingError> {
        let endpoint = Endpoint::ListBookmarks(args);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: ListBookmarksResponse = self.client.execute(request)?.json()?;

        Ok(body)
    }

    /// List archived bookmarks
    pub fn list_archived_bookmarks(
        &self,
        args: ListBookmarksArgs,
    ) -> Result<ListBookmarksResponse, LinkDingError> {
        let endpoint = Endpoint::ListArchivedBookmarks(args);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: ListBookmarksResponse = self.client.execute(request)?.json()?;
        Ok(body)
    }

    /// Get a bookmark by ID
    pub fn get_bookmark(&self, id: i32) -> Result<Bookmark, LinkDingError> {
        let endpoint = Endpoint::GetBookmark(id);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: Bookmark = self.client.execute(request)?.json()?;
        Ok(body)
    }

    /// Check if a URL has been bookmarked
    ///
    /// If the URL has already been bookmarked this will return the bookmark
    /// data, otherwise the bookmark data will be `None`. The metadata of the
    /// webpage will always be returned.
    pub fn check_url(&self, url: &str) -> Result<CheckUrlResponse, LinkDingError> {
        let endpoint = Endpoint::CheckUrl(url.to_string());
        let request = self.prepare_request(endpoint)?.build()?;
        let body: CheckUrlResponse = self.client.execute(request)?.json()?;
        Ok(body)
    }

    /// Create a bookmark
    ///
    /// If the bookmark already exists, it will be updated with the new data passed in the `body` parameter.
    pub fn create_bookmark(&self, body: CreateBookmarkBody) -> Result<Bookmark, LinkDingError> {
        let endpoint = Endpoint::CreateBookmark;
        let request = self
            .prepare_request(endpoint)?
            .body(serde_json::to_string(&body)?)
            .build()?;
        let body: Bookmark = self.client.execute(request)?.json()?;
        Ok(body)
    }

    /// Update a bookmark
    ///
    /// Pass only the fields you want to update in the `body` parameter.
    pub fn update_bookmark(
        &self,
        id: i32,
        body: UpdateBookmarkBody,
    ) -> Result<Bookmark, LinkDingError> {
        let endpoint = Endpoint::UpdateBookmark(id);
        let request = self
            .prepare_request(endpoint)?
            .body(serde_json::to_string(&body)?)
            .build()?;
        let body: Bookmark = self.client.execute(request)?.json()?;
        Ok(body)
    }

    /// Archive a bookmark
    pub fn archive_bookmark(&self, id: i32) -> Result<bool, LinkDingError> {
        let endpoint = Endpoint::ArchiveBookmark(id);
        let request = self.prepare_request(endpoint)?.build()?;
        let response = self.client.execute(request)?;

        Ok(response.status() == StatusCode::NO_CONTENT)
    }

    /// Take a bookmark out of the archive
    pub fn unarchive_bookmark(&self, id: i32) -> Result<bool, LinkDingError> {
        let endpoint = Endpoint::UnarchiveBookmark(id);
        let request = self.prepare_request(endpoint)?.build()?;
        let response = self.client.execute(request)?;
        Ok(response.status() == StatusCode::NO_CONTENT)
    }

    /// Delete a bookmark
    pub fn delete_bookmark(&self, id: i32) -> Result<bool, LinkDingError> {
        let endpoint = Endpoint::DeleteBookmark(id);
        let request = self.prepare_request(endpoint)?.build()?;
        let response = self.client.execute(request)?;
        Ok(response.status() == StatusCode::NO_CONTENT)
    }

    /// List tags
    pub fn list_tags(&self, args: ListTagsArgs) -> Result<ListTagsResponse, LinkDingError> {
        let endpoint = Endpoint::ListTags(args);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: ListTagsResponse = self.client.execute(request)?.json()?;
        Ok(body)
    }

    /// Get a tag by ID
    pub fn get_tag(&self, id: i32) -> Result<TagData, LinkDingError> {
        let endpoint = Endpoint::GetTag(id);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: TagData = self.client.execute(request)?.json()?;
        Ok(body)
    }

    /// Create a tag
    pub fn create_tag(&self, name: &str) -> Result<TagData, LinkDingError> {
        let endpoint = Endpoint::CreateTag;
        let body = serde_json::json!({ "name": name });
        let request = self
            .prepare_request(endpoint)?
            .body(serde_json::to_string(&body)?)
            .build()?;
        let body: TagData = self.client.execute(request)?.json()?;
        Ok(body)
    }

    /// Get the user's profile
    pub fn get_user_profile(&self) -> Result<UserProfile, LinkDingError> {
        let endpoint = Endpoint::GetUserProfile;
        let request = self.prepare_request(endpoint)?.build()?;
        let body: UserProfile = self.client.execute(request)?.json()?;
        Ok(body)
    }

    /// Lists a bookmarks' assets
    pub fn list_bookmark_assets(
        &self,
        id: i32,
    ) -> Result<ListBookmarkAssetsResponse, LinkDingError> {
        let endpoint = Endpoint::ListBookmarkAssets(id);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: ListBookmarkAssetsResponse = self.client.execute(request)?.json()?;
        Ok(body)
    }

    /// Retrieve info for a single asset of a bookmark
    pub fn retrieve_bookmark_asset(
        &self,
        bookmark_id: i32,
        asset_id: i32,
    ) -> Result<BookmarkAsset, LinkDingError> {
        let endpoint = Endpoint::RetrieveBookmarkAsset(bookmark_id, asset_id);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: BookmarkAsset = self.client.execute(request)?.json()?;
        Ok(body)
    }

    /// Download a bookmark's asset
    pub fn download_bookmark_asset(
        &self,
        bookmark_id: i32,
        asset_id: i32,
    ) -> Result<Vec<u8>, LinkDingError> {
        let endpoint = Endpoint::DownloadBookmarkAsset(bookmark_id, asset_id);
        let request = self.prepare_request(endpoint)?.build()?;
        let mut buffer: Vec<u8> = Vec::new();
        self.client.execute(request)?.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    /// Upload an asset for a bookmark
    pub fn upload_bookmark_asset(
        &self,
        bookmark_id: i32,
        bytes: &[u8],
    ) -> Result<BookmarkAsset, LinkDingError> {
        let endpoint = Endpoint::UploadBookmarkAsset(bookmark_id);
        let bytes_part = Part::bytes(bytes.to_owned());
        let form = reqwest::blocking::multipart::Form::new().part("file", bytes_part);
        let request = self.prepare_request(endpoint)?.multipart(form).build()?;
        let body: BookmarkAsset = self.client.execute(request)?.json()?;
        Ok(body)
    }

    /// Delete a bookmark's asset
    pub fn delete_bookmark_asset(
        &self,
        bookmark_id: i32,
        asset_id: i32,
    ) -> Result<bool, LinkDingError> {
        let endpoint = Endpoint::DeleteBookmarkAsset(bookmark_id, asset_id);
        let request = self.prepare_request(endpoint)?.build()?;
        let response = self.client.execute(request)?;
        Ok(response.status() == StatusCode::NO_CONTENT)
    }
}
