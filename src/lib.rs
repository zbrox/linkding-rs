#[cfg(feature = "ffi")]
uniffi::setup_scaffolding!();

pub mod bookmark_assets;
pub mod bookmarks;
pub mod tags;
pub mod users;

use bookmark_assets::{BookmarkAsset, ListBookmarkAssetsResponse};
pub use bookmarks::{
    Bookmark, CheckUrlResponse, CreateBookmarkBody, ListBookmarksArgs, ListBookmarksResponse,
    UpdateBookmarkBody,
};
use http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
pub use tags::{ListTagsArgs, ListTagsResponse, TagData};
use thiserror::Error;
pub use users::{DateDisplay, LinkTarget, SelectedTheme, SortBy, TagSearchMethod, UserProfile};

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

impl TryInto<http::Uri> for Endpoint {
    type Error = LinkDingError;

    fn try_into(self) -> Result<http::Uri, Self::Error> {
        match self.clone() {
            Self::ListBookmarks(args) | Self::ListArchivedBookmarks(args) => {
                let query_string = args.query_string();
                let path: String = self.into();
                http::Uri::try_from(format!("{}?{}", path, query_string))
                    .map_err(LinkDingError::InvalidUrl)
            }
            Self::ListTags(args) => {
                let query_string = args.query_string();
                let path: String = self.into();
                http::Uri::try_from(format!("{}?{}", path, query_string))
                    .map_err(LinkDingError::InvalidUrl)
            }
            Self::CheckUrl(url) => {
                let path: String = self.into();
                http::Uri::try_from(format!("{}?url={}", path, url))
                    .map_err(LinkDingError::InvalidUrl)
            }
            Self::GetBookmark(_)
            | Self::UpdateBookmark(_)
            | Self::ArchiveBookmark(_)
            | Self::UnarchiveBookmark(_)
            | Self::DeleteBookmark(_)
            | Self::CreateBookmark
            | Self::GetTag(_)
            | Self::CreateTag
            | Self::GetUserProfile
            | Self::ListBookmarkAssets(_)
            | Self::RetrieveBookmarkAsset(_, _)
            | Self::DownloadBookmarkAsset(_, _)
            | Self::UploadBookmarkAsset(_)
            | Self::DeleteBookmarkAsset(_, _) => {
                let path: String = self.into();
                http::Uri::try_from(path).map_err(LinkDingError::InvalidUrl)
            }
        }
    }
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
        match &val {
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
        }
    }
}

impl From<Endpoint> for http::Method {
    fn from(val: Endpoint) -> Self {
        match val {
            Endpoint::ListBookmarks(_) => http::Method::GET,
            Endpoint::ListArchivedBookmarks(_) => http::Method::GET,
            Endpoint::GetBookmark(_) => http::Method::GET,
            Endpoint::CheckUrl(_) => http::Method::GET,
            Endpoint::CreateBookmark => http::Method::POST,
            Endpoint::UpdateBookmark(_) => http::Method::PATCH,
            Endpoint::ArchiveBookmark(_) => http::Method::POST,
            Endpoint::UnarchiveBookmark(_) => http::Method::POST,
            Endpoint::DeleteBookmark(_) => http::Method::DELETE,
            Endpoint::ListTags(_) => http::Method::GET,
            Endpoint::GetTag(_) => http::Method::GET,
            Endpoint::CreateTag => http::Method::POST,
            Endpoint::GetUserProfile => http::Method::GET,
            Endpoint::ListBookmarkAssets(_) => http::Method::GET,
            Endpoint::RetrieveBookmarkAsset(_, _) => http::Method::GET,
            Endpoint::DownloadBookmarkAsset(_, _) => http::Method::GET,
            Endpoint::UploadBookmarkAsset(_) => http::Method::POST,
            Endpoint::DeleteBookmarkAsset(_, _) => http::Method::DELETE,
        }
    }
}

impl From<Endpoint> for http::HeaderMap {
    fn from(val: Endpoint) -> Self {
        let mut headers = http::HeaderMap::new();
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
}

impl LinkDingClient {
    fn prepare_request(&self, endpoint: Endpoint) -> Result<http::request::Builder, LinkDingError> {
        let uri: http::Uri = self.url.parse()?;
        let path_and_query_uri: http::Uri = endpoint.clone().try_into()?;
        let uri = http::uri::Builder::from(uri)
            .path_and_query(path_and_query_uri.to_string())
            .build()
            .map_err(LinkDingError::ParseUrlError)?;

        let mut builder = http::request::Builder::new()
            .method(endpoint.clone())
            .uri(uri);

        let endpoint_headers: http::HeaderMap = endpoint.clone().into();
        let request_header_map = builder
            .headers_mut()
            .expect("Could not get mutable reference to request headers");
        for (header_key, header_value) in endpoint_headers.iter() {
            request_header_map.insert(header_key.clone(), header_value.clone());
        }
        request_header_map.insert(
            AUTHORIZATION,
            format!("Token {}", &self.token)
                .parse()
                .expect("Could not parse authorization header value"),
        );
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
        }
    }

    /// List unarchived bookmarks
    pub fn list_bookmarks(
        &self,
        args: ListBookmarksArgs,
    ) -> Result<ListBookmarksResponse, LinkDingError> {
        let endpoint = Endpoint::ListBookmarks(args);
        let request = self.prepare_request(endpoint)?.body(())?;
        let body: ListBookmarksResponse = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    /// List archived bookmarks
    pub fn list_archived_bookmarks(
        &self,
        args: ListBookmarksArgs,
    ) -> Result<ListBookmarksResponse, LinkDingError> {
        let endpoint = Endpoint::ListArchivedBookmarks(args);
        let request = self.prepare_request(endpoint)?.body(())?;
        let body: ListBookmarksResponse = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    /// Get a bookmark by ID
    pub fn get_bookmark(&self, id: i32) -> Result<Bookmark, LinkDingError> {
        let endpoint = Endpoint::GetBookmark(id);
        let request = self.prepare_request(endpoint)?.body(())?;
        let body: Bookmark = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    /// Check if a URL has been bookmarked
    ///
    /// If the URL has already been bookmarked this will return the bookmark
    /// data, otherwise the bookmark data will be `None`. The metadata of the
    /// webpage will always be returned.
    pub fn check_url(&self, url: &str) -> Result<CheckUrlResponse, LinkDingError> {
        let endpoint = Endpoint::CheckUrl(url.to_string());
        let request = self.prepare_request(endpoint)?.body(())?;
        let body: CheckUrlResponse = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    /// Create a bookmark
    ///
    /// If the bookmark already exists, it will be updated with the new data passed in the `body` parameter.
    pub fn create_bookmark(&self, body: CreateBookmarkBody) -> Result<Bookmark, LinkDingError> {
        let endpoint = Endpoint::CreateBookmark;
        let request = self
            .prepare_request(endpoint)?
            .body(serde_json::to_string(&body)?)?;
        let body: Bookmark = ureq::run(request)?.body_mut().read_json()?;
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
            .body(serde_json::to_string(&body)?)?;
        let body: Bookmark = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    /// Archive a bookmark
    pub fn archive_bookmark(&self, id: i32) -> Result<bool, LinkDingError> {
        let endpoint = Endpoint::ArchiveBookmark(id);
        let request = self.prepare_request(endpoint)?.body(())?;
        Ok(ureq::run(request)?.status() == http::StatusCode::NO_CONTENT)
    }

    /// Take a bookmark out of the archive
    pub fn unarchive_bookmark(&self, id: i32) -> Result<bool, LinkDingError> {
        let endpoint = Endpoint::UnarchiveBookmark(id);
        let request = self.prepare_request(endpoint)?.body(())?;
        Ok(ureq::run(request)?.status() == http::StatusCode::NO_CONTENT)
    }

    /// Delete a bookmark
    pub fn delete_bookmark(&self, id: i32) -> Result<bool, LinkDingError> {
        let endpoint = Endpoint::DeleteBookmark(id);
        let request = self.prepare_request(endpoint)?.body(())?;
        Ok(ureq::run(request)?.status() == http::StatusCode::NO_CONTENT)
    }

    /// List tags
    pub fn list_tags(&self, args: ListTagsArgs) -> Result<ListTagsResponse, LinkDingError> {
        let endpoint = Endpoint::ListTags(args);
        let request = self.prepare_request(endpoint)?.body(())?;
        let body: ListTagsResponse = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    /// Get a tag by ID
    pub fn get_tag(&self, id: i32) -> Result<TagData, LinkDingError> {
        let endpoint = Endpoint::GetTag(id);
        let request = self.prepare_request(endpoint)?.body(())?;
        let body: TagData = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    /// Create a tag
    pub fn create_tag(&self, name: &str) -> Result<TagData, LinkDingError> {
        let endpoint = Endpoint::CreateTag;
        let body = serde_json::json!({ "name": name });
        let request = self
            .prepare_request(endpoint)?
            .body(serde_json::to_string(&body)?)?;
        let body: TagData = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    /// Get the user's profile
    pub fn get_user_profile(&self) -> Result<UserProfile, LinkDingError> {
        let endpoint = Endpoint::GetUserProfile;
        let request = self.prepare_request(endpoint)?.body(())?;
        let body: UserProfile = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    /// Lists a bookmarks' assets
    pub fn list_bookmark_assets(
        &self,
        id: i32,
    ) -> Result<ListBookmarkAssetsResponse, LinkDingError> {
        let endpoint = Endpoint::ListBookmarkAssets(id);
        let request = self.prepare_request(endpoint)?.body(())?;
        let body: ListBookmarkAssetsResponse = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    /// Retrieve info for a single asset of a bookmark
    pub fn retrieve_bookmark_asset(
        &self,
        bookmark_id: i32,
        asset_id: i32,
    ) -> Result<BookmarkAsset, LinkDingError> {
        let endpoint = Endpoint::RetrieveBookmarkAsset(bookmark_id, asset_id);
        let request = self.prepare_request(endpoint)?.body(())?;
        let body: BookmarkAsset = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    /// Download a bookmark's asset
    pub fn download_bookmark_asset(
        &self,
        bookmark_id: i32,
        asset_id: i32,
    ) -> Result<Vec<u8>, LinkDingError> {
        let endpoint = Endpoint::DownloadBookmarkAsset(bookmark_id, asset_id);
        let request = self.prepare_request(endpoint)?.body(())?;
        let mut response = ureq::run(request)?;
        Ok(response.body_mut().read_to_vec()?)
    }

    /// Upload an asset for a bookmark
    pub fn upload_bookmark_asset(
        &self,
        bookmark_id: i32,
        bytes: &[u8],
    ) -> Result<BookmarkAsset, LinkDingError> {
        let endpoint = Endpoint::UploadBookmarkAsset(bookmark_id);
        let request = self.prepare_request(endpoint)?.body(bytes)?;
        let body: BookmarkAsset = ureq::run(request)?.body_mut().read_json()?;
        Ok(body)
    }

    /// Delete a bookmark's asset
    pub fn delete_bookmark_asset(
        &self,
        bookmark_id: i32,
        asset_id: i32,
    ) -> Result<bool, LinkDingError> {
        let endpoint = Endpoint::DeleteBookmarkAsset(bookmark_id, asset_id);
        let request = self.prepare_request(endpoint)?.body(())?;
        Ok(ureq::run(request)?.status() == http::StatusCode::NO_CONTENT)
    }
}
