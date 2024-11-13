use serde::{Deserialize, Serialize};

use crate::QueryString;

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct ListTagsResponse {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<TagData>,
}

#[derive(Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct TagData {
    id: i32,
    name: String,
    date_added: String,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "ffi", derive(uniffi::Record))]
pub struct ListTagsArgs {
    limit: Option<i32>,
    offset: Option<i32>,
}

impl QueryString for ListTagsArgs {
    fn query_string(&self) -> String {
        vec![
            ("limit", self.limit.as_ref().map(|v| v.to_string())),
            ("offset", self.offset.as_ref().map(|v| v.to_string())),
        ]
        .iter()
        .filter_map(|(k, v)| v.as_ref().map(|v| format!("{}={}", k, v)))
        .collect::<Vec<_>>()
        .join("&")
    }
}
