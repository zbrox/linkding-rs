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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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
    query: Option<String>,
    limit: Option<i32>,
    offset: Option<i32>,
}

impl QueryString for ListBookmarksArgs {
    fn query_string(&self) -> String {
        vec![
            ("q", self.query.as_ref().map(|v| v.to_string())),
            ("limit", self.limit.as_ref().map(|v| v.to_string())),
            ("offset", self.offset.as_ref().map(|v| v.to_string())),
        ]
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|v| format!("{}={}", k, v)))
        .collect::<Vec<_>>()
        .join("&")
    }
}
