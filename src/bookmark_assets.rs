use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
pub enum BookmarkAssetType {
    Upload,
    #[default]
    Snapshot,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
pub enum BookmarkAssetStatus {
    #[default]
    Pending,
    Complete,
    Failure,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct BookmarkAsset {
    pub id: i32,
    pub bookmark: i32,
    pub asset_type: BookmarkAssetType,
    pub date_created: String,
    pub content_type: String,
    pub display_name: String,
    pub status: BookmarkAssetStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct ListBookmarkAssetsResponse {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<BookmarkAsset>,
}
