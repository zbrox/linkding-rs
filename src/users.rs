use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct UserProfile {
    theme: String,
    bookmark_date_display: String,
    bookmark_link_target: String,
    web_archive_integration: String,
    tag_search: String,
    enable_sharing: bool,
    enable_public_sharing: bool,
    enable_favicons: bool,
    display_url: bool,
    permanent_notes: bool,
    search_preferences: UserSearchPreferences,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct UserSearchPreferences {
    pub sort: SortBy,
    pub shared: bool,
    pub unread: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
pub enum SortBy {
    TitleAsc,
    TitleDesc,
}
