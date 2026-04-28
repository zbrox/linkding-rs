use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
pub enum BookmarkAssetType {
    Upload,
    #[default]
    Snapshot,
    Precrawled,
    /// Catch-all for asset types Linkding may add in the future.
    ///
    /// Note: this variant only carries through deserialisation. If you serialise a
    /// `BookmarkAsset` that came back as `Unknown`, the original string is lost.
    #[serde(other)]
    Unknown,
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
    #[serde(default)]
    pub file_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct ListBookmarkAssetsResponse {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<BookmarkAsset>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asset_type_deserializes_known_variants() {
        assert_eq!(
            serde_json::from_str::<BookmarkAssetType>("\"upload\"").unwrap(),
            BookmarkAssetType::Upload
        );
        assert_eq!(
            serde_json::from_str::<BookmarkAssetType>("\"snapshot\"").unwrap(),
            BookmarkAssetType::Snapshot
        );
        assert_eq!(
            serde_json::from_str::<BookmarkAssetType>("\"precrawled\"").unwrap(),
            BookmarkAssetType::Precrawled
        );
    }

    #[test]
    fn asset_type_deserializes_unknown_variant_to_unknown() {
        assert_eq!(
            serde_json::from_str::<BookmarkAssetType>("\"weird_future_type\"").unwrap(),
            BookmarkAssetType::Unknown
        );
    }

    #[test]
    fn bookmark_asset_deserializes_with_file_size() {
        let json = r#"{
            "id": 1,
            "bookmark": 42,
            "asset_type": "snapshot",
            "date_created": "2026-04-29T12:00:00Z",
            "content_type": "text/html",
            "display_name": "snapshot.html",
            "status": "complete",
            "file_size": 12345
        }"#;
        let asset: BookmarkAsset = serde_json::from_str(json).unwrap();
        assert_eq!(asset.file_size, Some(12345));
    }

    #[test]
    fn bookmark_asset_deserializes_without_file_size() {
        let json = r#"{
            "id": 1,
            "bookmark": 42,
            "asset_type": "snapshot",
            "date_created": "2026-04-29T12:00:00Z",
            "content_type": "text/html",
            "display_name": "snapshot.html",
            "status": "complete"
        }"#;
        let asset: BookmarkAsset = serde_json::from_str(json).unwrap();
        assert_eq!(asset.file_size, None);
    }
}
