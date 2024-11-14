#[cfg(feature = "ffi")]
uniffi::setup_scaffolding!();

pub mod bookmarks;
pub mod tags;
pub mod users;

pub use bookmarks::{
    Bookmark, CheckUrlResponse, CreateBookmarkBody, ListBookmarksArgs, ListBookmarksResponse,
    UpdateBookmarkBody,
};
pub use tags::{ListTagsArgs, ListTagsResponse, TagData};
use thiserror::Error;
pub use users::UserProfile;

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
            Self::ListTags(args) => {
                let query_string = args.query_string();
                let path: String = self.into();
                http::Uri::try_from(format!("{}{}", path, query_string))
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
            | Self::GetUserProfile => {
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
            | Self::GetUserProfile => "".to_string(),
        }
    }
}

impl Into<String> for Endpoint {
    fn into(self) -> String {
        match &self {
            Self::ListBookmarks(_) => "/api/bookmarks/".to_string(),
            Self::ListArchivedBookmarks(_) => "/api/bookmarks/archived/".to_string(),
            Self::GetBookmark(id) | Self::UpdateBookmark(id) | Self::DeleteBookmark(id) => {
                format!("/api/bookmarks/{}/", &id)
            }
            Self::CheckUrl(_) => "/api/bookmarks/check/".to_string(),
            Self::CreateBookmark => "/api/bookmarks/".to_string(),
            Self::ArchiveBookmark(id) => format!("/api/bookmarks/{}/archive/", &id),
            Self::UnarchiveBookmark(id) => format!("/api/bookmarks/{}/unarchive/", &id),
            Self::ListTags(_) | Self::CreateTag => "/api/tags/".to_string(),
            Self::GetTag(id) => format!("/api/tags/{}/", &id),
            Self::GetUserProfile => "/api/user/profile/".to_string(),
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
            Self::ArchiveBookmark(_) => http::Method::POST,
            Self::UnarchiveBookmark(_) => http::Method::POST,
            Self::DeleteBookmark(_) => http::Method::DELETE,
            Self::ListTags(_) => http::Method::GET,
            Self::GetTag(_) => http::Method::GET,
            Self::CreateTag => http::Method::POST,
            Self::GetUserProfile => http::Method::GET,
        }
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
}
