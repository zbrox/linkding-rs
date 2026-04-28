#![cfg(feature = "async")]

use linkding::{
    bookmark_assets::{BookmarkAssetType, ListBookmarkAssetsResponse},
    LinkDingAsyncClient, ListBookmarksArgs,
};
use wiremock::{
    matchers::{header, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

mod common;
use common::{ASSETS_PAYLOAD, SINGLE_BOOKMARK_PAYLOAD};

const TOKEN: &str = "test-token";

fn client(server: &MockServer) -> LinkDingAsyncClient {
    LinkDingAsyncClient::new(&server.uri(), TOKEN)
}

#[tokio::test]
async fn list_bookmarks_sends_modified_since_and_auth_header() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/bookmarks/"))
        .and(query_param("modified_since", "2026-01-01T00:00:00Z"))
        .and(query_param("limit", "50"))
        .and(header("authorization", format!("Token {TOKEN}").as_str()))
        .respond_with(
            ResponseTemplate::new(200).set_body_string(format!(
                r#"{{"count":1,"next":null,"previous":null,"results":[{}]}}"#,
                SINGLE_BOOKMARK_PAYLOAD
            )),
        )
        .expect(1)
        .mount(&server)
        .await;

    let resp = client(&server)
        .list_bookmarks(ListBookmarksArgs {
            limit: Some(50),
            modified_since: Some("2026-01-01T00:00:00Z".to_string()),
            ..Default::default()
        })
        .await
        .expect("list_bookmarks");
    assert_eq!(resp.count, 1);
    assert_eq!(resp.results[0].id, 42);
}

#[tokio::test]
async fn list_assets_handles_all_variants_including_unknown() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/bookmarks/42/assets/"))
        .respond_with(ResponseTemplate::new(200).set_body_string(ASSETS_PAYLOAD))
        .expect(1)
        .mount(&server)
        .await;

    let resp: ListBookmarkAssetsResponse = client(&server)
        .list_bookmark_assets(42)
        .await
        .expect("list_bookmark_assets");
    assert_eq!(resp.results.len(), 4);
    assert!(matches!(resp.results[0].asset_type, BookmarkAssetType::Snapshot));
    assert_eq!(resp.results[0].file_size, Some(12345));
    assert!(matches!(resp.results[1].asset_type, BookmarkAssetType::Upload));
    assert_eq!(resp.results[1].file_size, None);
    assert!(matches!(resp.results[2].asset_type, BookmarkAssetType::Precrawled));
    assert!(matches!(resp.results[3].asset_type, BookmarkAssetType::Unknown));
}

#[tokio::test]
async fn download_asset_returns_raw_bytes_with_wildcard_accept() {
    let server = MockServer::start().await;
    let bytes = b"<html>snapshot</html>";
    Mock::given(method("GET"))
        .and(path("/api/bookmarks/42/assets/1/download/"))
        .and(header("accept", "*/*"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(bytes.to_vec()))
        .expect(1)
        .mount(&server)
        .await;

    let result = client(&server)
        .download_bookmark_asset(42, 1)
        .await
        .expect("download_bookmark_asset");
    assert_eq!(result, bytes);
}

#[tokio::test]
async fn http_error_surfaces_as_send_http_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/api/bookmarks/999/"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&server)
        .await;

    // 404 still parses-as-JSON-failure (response body is empty), surfacing as SendHttpError
    let err = client(&server)
        .get_bookmark(999)
        .await
        .expect_err("expected error on 404");
    assert!(matches!(err, linkding::LinkDingError::SendHttpError(_)));
}
