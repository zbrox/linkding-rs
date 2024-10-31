mod bookmarks;
pub use bookmarks::Bookmark;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LinkDingError {
    #[error("Error sending HTTP request")]
    SendHttpError(#[from] ureq::Error),
    #[error("Could not parse JSON response from API")]
    JsonParse(#[from] std::io::Error),
}
