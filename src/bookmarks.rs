use serde::{Deserialize, Serialize};

use crate::QueryString;

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

#[derive(Debug, Serialize, Deserialize, Default)]
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

#[derive(Debug, Serialize, Deserialize, Default)]
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
pub struct ListBookmarksArgs {
    pub query: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    /// Return only bookmarks modified since this RFC 3339 timestamp.
    pub modified_since: Option<String>,
}

impl QueryString for ListBookmarksArgs {
    fn query_string(&self) -> String {
        [
            ("q", self.query.as_ref().map(|v| v.to_string())),
            ("limit", self.limit.as_ref().map(|v| v.to_string())),
            ("offset", self.offset.as_ref().map(|v| v.to_string())),
            (
                "modified_since",
                self.modified_since.as_ref().map(|v| v.to_string()),
            ),
        ]
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|v| format!("{}={}", k, v)))
        .collect::<Vec<_>>()
        .join("&")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_bookmarks_args_emits_modified_since_only() {
        let args = ListBookmarksArgs {
            modified_since: Some("2026-01-01T00:00:00Z".to_string()),
            ..Default::default()
        };
        assert_eq!(
            args.query_string(),
            "modified_since=2026-01-01T00:00:00Z"
        );
    }

    #[test]
    fn list_bookmarks_args_emits_all_fields() {
        let args = ListBookmarksArgs {
            query: Some("rust".to_string()),
            limit: Some(50),
            offset: Some(10),
            modified_since: Some("2026-01-01T00:00:00Z".to_string()),
        };
        let qs = args.query_string();
        assert!(qs.contains("q=rust"), "{qs}");
        assert!(qs.contains("limit=50"), "{qs}");
        assert!(qs.contains("offset=10"), "{qs}");
        assert!(qs.contains("modified_since=2026-01-01T00:00:00Z"), "{qs}");
    }

    #[test]
    fn list_bookmarks_args_default_is_empty_query() {
        let args = ListBookmarksArgs::default();
        assert_eq!(args.query_string(), "");
    }
}
