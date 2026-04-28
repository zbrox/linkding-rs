use reqwest::{
    blocking::multipart::{Form, Part},
    StatusCode,
};

use crate::{
    bookmark_assets::{BookmarkAsset, ListBookmarkAssetsResponse},
    bookmarks::{
        Bookmark, CheckUrlResponse, CreateBookmarkBody, ListBookmarksArgs, ListBookmarksResponse,
        UpdateBookmarkBody,
    },
    build_request_spec,
    tags::{ListTagsArgs, ListTagsResponse, TagData},
    users::UserProfile,
    Endpoint, LinkDingError,
};

/// A sync client for the LinkDing API.
///
/// This client is used to interact with the LinkDing API. It provides methods for
/// managing bookmarks and tags, the full capability of the LinkDing API.
///
/// # Example
///
/// ```no_run
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
        let spec = build_request_spec(&self.url, &self.token, endpoint)?;
        Ok(self
            .client
            .request(spec.method, spec.url)
            .headers(spec.headers))
    }
}

#[cfg_attr(feature = "ffi", uniffi::export)]
impl LinkDingClient {
    #[cfg_attr(feature = "ffi", uniffi::constructor)]
    pub fn new(url: &str, token: &str) -> Self {
        LinkDingClient {
            token: token.to_string(),
            url: url.to_string(),
            client: reqwest::blocking::Client::builder()
                .build()
                .expect("Could not create web client"),
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
        let response = self.client.execute(request)?;
        Ok(response.bytes()?.into())
    }

    /// Upload an asset for a bookmark
    pub fn upload_bookmark_asset(
        &self,
        bookmark_id: i32,
        bytes: &[u8],
    ) -> Result<BookmarkAsset, LinkDingError> {
        let endpoint = Endpoint::UploadBookmarkAsset(bookmark_id);
        let bytes_part = Part::bytes(bytes.to_owned());
        let form = Form::new().part("file", bytes_part);
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
