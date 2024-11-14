use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct UserProfile {
    theme: SelectedTheme,
    bookmark_date_display: DateDisplay,
    bookmark_link_target: LinkTarget,
    #[serde(serialize_with = "serialize_webarchive_integration")]
    #[serde(deserialize_with = "deserialize_webarchive_integration")]
    web_archive_integration: bool,
    tag_search: TagSearchMethod,
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
    #[serde(default)]
    pub sort: SortBy,
    #[serde(default)]
    pub shared: bool,
    #[serde(default)]
    pub unread: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
pub enum SortBy {
    #[default]
    TitleAsc,
    TitleDesc,
    AddedAsc,
    AddedDesc,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
pub enum SelectedTheme {
    Light,
    Dark,
    Auto,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
pub enum DateDisplay {
    Relative,
    Absolute,
    Hidden,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
pub enum LinkTarget {
    SameWindow,
    NewWindow,
}

impl Serialize for LinkTarget {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = match *self {
            LinkTarget::SameWindow => "_self",
            LinkTarget::NewWindow => "_blank",
        };
        serializer.serialize_str(s)
    }
}

impl<'de> Deserialize<'de> for LinkTarget {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        match s {
            "_self" => Ok(LinkTarget::SameWindow),
            "_blank" => Ok(LinkTarget::NewWindow),
            _ => Err(serde::de::Error::custom("invalid variant")),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ffi", derive(uniffi::Enum))]
pub enum TagSearchMethod {
    Strict,
    Lax,
}

fn serialize_webarchive_integration<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string_value = match value {
        true => "enabled",
        false => "disabled",
    };
    serializer.serialize_str(string_value)
}

fn deserialize_webarchive_integration<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let string_value = String::deserialize(deserializer)?;
    match string_value.as_str() {
        "enabled" => Ok(true),
        "disabled" => Ok(false),
        _ => Err(serde::de::Error::custom("invalid variant")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_user_profile() {
        let json = r#"{
            "theme": "auto",
            "bookmark_date_display": "relative",
            "bookmark_link_target": "_blank",
            "web_archive_integration": "disabled",
            "tag_search": "strict",
            "enable_sharing": false,
            "enable_public_sharing": false,
            "enable_favicons": false,
            "display_url": false,
            "permanent_notes": false,
            "search_preferences": {
                "sort": "title_asc",
                "shared": true,
                "unread": true
            }
        }"#;
        let user_profile: UserProfile = serde_json::from_str(json).unwrap();
        assert_eq!(user_profile.theme, SelectedTheme::Auto);
        assert_eq!(user_profile.bookmark_date_display, DateDisplay::Relative);
        assert_eq!(user_profile.bookmark_link_target, LinkTarget::NewWindow);
        assert_eq!(user_profile.web_archive_integration, false);
        assert_eq!(user_profile.tag_search, TagSearchMethod::Strict);
        assert_eq!(user_profile.enable_sharing, false);
        assert_eq!(user_profile.enable_public_sharing, false);
        assert_eq!(user_profile.enable_favicons, false);
        assert_eq!(user_profile.display_url, false);
        assert_eq!(user_profile.permanent_notes, false);
        assert_eq!(user_profile.search_preferences.sort, SortBy::TitleAsc);
        assert_eq!(user_profile.search_preferences.shared, true);
        assert_eq!(user_profile.search_preferences.unread, true);
    }

    #[test]
    fn deserialize_user_profile_without_search_preferences() {
        let json = r#"{
            "theme": "auto",
            "bookmark_date_display": "relative",
            "bookmark_link_target": "_blank",
            "web_archive_integration": "disabled",
            "tag_search": "strict",
            "enable_sharing": false,
            "enable_public_sharing": false,
            "enable_favicons": false,
            "display_url": false,
            "permanent_notes": false,
            "search_preferences": {
            }
        }"#;
        let user_profile: UserProfile = serde_json::from_str(json).unwrap();
        assert_eq!(user_profile.theme, SelectedTheme::Auto);
        assert_eq!(user_profile.bookmark_date_display, DateDisplay::Relative);
        assert_eq!(user_profile.bookmark_link_target, LinkTarget::NewWindow);
        assert_eq!(user_profile.web_archive_integration, false);
        assert_eq!(user_profile.tag_search, TagSearchMethod::Strict);
        assert_eq!(user_profile.enable_sharing, false);
        assert_eq!(user_profile.enable_public_sharing, false);
        assert_eq!(user_profile.enable_favicons, false);
        assert_eq!(user_profile.display_url, false);
        assert_eq!(user_profile.permanent_notes, false);
        assert_eq!(user_profile.search_preferences.sort, SortBy::TitleAsc);
        assert_eq!(user_profile.search_preferences.shared, false);
        assert_eq!(user_profile.search_preferences.unread, false);
    }
}
