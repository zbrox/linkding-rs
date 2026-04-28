use reqwest::{
    multipart::{Form, Part},
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

/// An async client for the LinkDing API.
///
/// Mirrors [`LinkDingClient`](crate::LinkDingClient) but every method is `async`.
/// Requires the `async` cargo feature (enabled by default).
///
/// # Example
///
/// ```no_run
/// use linkding::{LinkDingAsyncClient, LinkDingError, CreateBookmarkBody};
///
/// #[tokio::main]
/// async fn main() -> Result<(), LinkDingError> {
///     let client = LinkDingAsyncClient::new("https://linkding.local:9090", "YOUR_API_TOKEN");
///     let new_bookmark = CreateBookmarkBody {
///         url: "https://example.com".to_string(),
///         ..Default::default()
///     };
///     let bookmark = client.create_bookmark(new_bookmark).await?;
///     println!("Bookmark created: {:?}", bookmark);
///     client.delete_bookmark(bookmark.id).await?;
///     println!("Bookmark deleted");
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct LinkDingAsyncClient {
    token: String,
    url: String,
    client: reqwest::Client,
}

impl LinkDingAsyncClient {
    pub fn new(url: &str, token: &str) -> Self {
        LinkDingAsyncClient {
            token: token.to_string(),
            url: url.to_string(),
            client: reqwest::Client::builder()
                .build()
                .expect("Could not create web client"),
        }
    }

    fn prepare_request(
        &self,
        endpoint: Endpoint,
    ) -> Result<reqwest::RequestBuilder, LinkDingError> {
        let spec = build_request_spec(&self.url, &self.token, endpoint)?;
        Ok(self
            .client
            .request(spec.method, spec.url)
            .headers(spec.headers))
    }

    /// List unarchived bookmarks
    pub async fn list_bookmarks(
        &self,
        args: ListBookmarksArgs,
    ) -> Result<ListBookmarksResponse, LinkDingError> {
        let endpoint = Endpoint::ListBookmarks(args);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: ListBookmarksResponse = self.client.execute(request).await?.json().await?;
        Ok(body)
    }

    /// List archived bookmarks
    pub async fn list_archived_bookmarks(
        &self,
        args: ListBookmarksArgs,
    ) -> Result<ListBookmarksResponse, LinkDingError> {
        let endpoint = Endpoint::ListArchivedBookmarks(args);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: ListBookmarksResponse = self.client.execute(request).await?.json().await?;
        Ok(body)
    }

    /// Get a bookmark by ID
    pub async fn get_bookmark(&self, id: i32) -> Result<Bookmark, LinkDingError> {
        let endpoint = Endpoint::GetBookmark(id);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: Bookmark = self.client.execute(request).await?.json().await?;
        Ok(body)
    }

    /// Check if a URL has been bookmarked
    pub async fn check_url(&self, url: &str) -> Result<CheckUrlResponse, LinkDingError> {
        let endpoint = Endpoint::CheckUrl(url.to_string());
        let request = self.prepare_request(endpoint)?.build()?;
        let body: CheckUrlResponse = self.client.execute(request).await?.json().await?;
        Ok(body)
    }

    /// Create a bookmark
    pub async fn create_bookmark(
        &self,
        body: CreateBookmarkBody,
    ) -> Result<Bookmark, LinkDingError> {
        let endpoint = Endpoint::CreateBookmark;
        let request = self
            .prepare_request(endpoint)?
            .body(serde_json::to_string(&body)?)
            .build()?;
        let body: Bookmark = self.client.execute(request).await?.json().await?;
        Ok(body)
    }

    /// Update a bookmark
    pub async fn update_bookmark(
        &self,
        id: i32,
        body: UpdateBookmarkBody,
    ) -> Result<Bookmark, LinkDingError> {
        let endpoint = Endpoint::UpdateBookmark(id);
        let request = self
            .prepare_request(endpoint)?
            .body(serde_json::to_string(&body)?)
            .build()?;
        let body: Bookmark = self.client.execute(request).await?.json().await?;
        Ok(body)
    }

    /// Archive a bookmark
    pub async fn archive_bookmark(&self, id: i32) -> Result<bool, LinkDingError> {
        let endpoint = Endpoint::ArchiveBookmark(id);
        let request = self.prepare_request(endpoint)?.build()?;
        let response = self.client.execute(request).await?;
        Ok(response.status() == StatusCode::NO_CONTENT)
    }

    /// Take a bookmark out of the archive
    pub async fn unarchive_bookmark(&self, id: i32) -> Result<bool, LinkDingError> {
        let endpoint = Endpoint::UnarchiveBookmark(id);
        let request = self.prepare_request(endpoint)?.build()?;
        let response = self.client.execute(request).await?;
        Ok(response.status() == StatusCode::NO_CONTENT)
    }

    /// Delete a bookmark
    pub async fn delete_bookmark(&self, id: i32) -> Result<bool, LinkDingError> {
        let endpoint = Endpoint::DeleteBookmark(id);
        let request = self.prepare_request(endpoint)?.build()?;
        let response = self.client.execute(request).await?;
        Ok(response.status() == StatusCode::NO_CONTENT)
    }

    /// List tags
    pub async fn list_tags(&self, args: ListTagsArgs) -> Result<ListTagsResponse, LinkDingError> {
        let endpoint = Endpoint::ListTags(args);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: ListTagsResponse = self.client.execute(request).await?.json().await?;
        Ok(body)
    }

    /// Get a tag by ID
    pub async fn get_tag(&self, id: i32) -> Result<TagData, LinkDingError> {
        let endpoint = Endpoint::GetTag(id);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: TagData = self.client.execute(request).await?.json().await?;
        Ok(body)
    }

    /// Create a tag
    pub async fn create_tag(&self, name: &str) -> Result<TagData, LinkDingError> {
        let endpoint = Endpoint::CreateTag;
        let body = serde_json::json!({ "name": name });
        let request = self
            .prepare_request(endpoint)?
            .body(serde_json::to_string(&body)?)
            .build()?;
        let body: TagData = self.client.execute(request).await?.json().await?;
        Ok(body)
    }

    /// Get the user's profile
    pub async fn get_user_profile(&self) -> Result<UserProfile, LinkDingError> {
        let endpoint = Endpoint::GetUserProfile;
        let request = self.prepare_request(endpoint)?.build()?;
        let body: UserProfile = self.client.execute(request).await?.json().await?;
        Ok(body)
    }

    /// Lists a bookmark's assets
    pub async fn list_bookmark_assets(
        &self,
        id: i32,
    ) -> Result<ListBookmarkAssetsResponse, LinkDingError> {
        let endpoint = Endpoint::ListBookmarkAssets(id);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: ListBookmarkAssetsResponse =
            self.client.execute(request).await?.json().await?;
        Ok(body)
    }

    /// Retrieve info for a single asset of a bookmark
    pub async fn retrieve_bookmark_asset(
        &self,
        bookmark_id: i32,
        asset_id: i32,
    ) -> Result<BookmarkAsset, LinkDingError> {
        let endpoint = Endpoint::RetrieveBookmarkAsset(bookmark_id, asset_id);
        let request = self.prepare_request(endpoint)?.build()?;
        let body: BookmarkAsset = self.client.execute(request).await?.json().await?;
        Ok(body)
    }

    /// Download a bookmark's asset
    pub async fn download_bookmark_asset(
        &self,
        bookmark_id: i32,
        asset_id: i32,
    ) -> Result<Vec<u8>, LinkDingError> {
        let endpoint = Endpoint::DownloadBookmarkAsset(bookmark_id, asset_id);
        let request = self.prepare_request(endpoint)?.build()?;
        let response = self.client.execute(request).await?;
        Ok(response.bytes().await?.into())
    }

    /// Upload an asset for a bookmark
    pub async fn upload_bookmark_asset(
        &self,
        bookmark_id: i32,
        bytes: &[u8],
    ) -> Result<BookmarkAsset, LinkDingError> {
        let endpoint = Endpoint::UploadBookmarkAsset(bookmark_id);
        let bytes_part = Part::bytes(bytes.to_owned());
        let form = Form::new().part("file", bytes_part);
        let request = self.prepare_request(endpoint)?.multipart(form).build()?;
        let body: BookmarkAsset = self.client.execute(request).await?.json().await?;
        Ok(body)
    }

    /// Delete a bookmark's asset
    pub async fn delete_bookmark_asset(
        &self,
        bookmark_id: i32,
        asset_id: i32,
    ) -> Result<bool, LinkDingError> {
        let endpoint = Endpoint::DeleteBookmarkAsset(bookmark_id, asset_id);
        let request = self.prepare_request(endpoint)?.build()?;
        let response = self.client.execute(request).await?;
        Ok(response.status() == StatusCode::NO_CONTENT)
    }
}
