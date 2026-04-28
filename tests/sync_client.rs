#![cfg(feature = "blocking")]

use linkding::{
    bookmark_assets::{BookmarkAssetType, ListBookmarkAssetsResponse},
    LinkDingClient, ListBookmarksArgs,
};
use wiremock::{
    matchers::{header, method, path, query_param},
    Mock, MockServer, ResponseTemplate,
};

mod common;
use common::{ASSETS_PAYLOAD, SINGLE_BOOKMARK_PAYLOAD};

const TOKEN: &str = "test-token";

/// Run an async setup block on a fresh runtime, returning the started mock
/// server. The sync client then talks to it from the test thread.
fn with_mock_server<F>(setup: F) -> (tokio::runtime::Runtime, MockServer)
where
    F: std::future::Future<Output = MockServer>,
{
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let server = rt.block_on(setup);
    (rt, server)
}

fn client(server: &MockServer) -> LinkDingClient {
    LinkDingClient::new(&server.uri(), TOKEN)
}

#[test]
fn list_bookmarks_sends_modified_since_and_auth_header() {
    let (_rt, server) = with_mock_server(async {
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
        server
    });

    let resp = client(&server)
        .list_bookmarks(ListBookmarksArgs {
            limit: Some(50),
            modified_since: Some("2026-01-01T00:00:00Z".to_string()),
            ..Default::default()
        })
        .expect("list_bookmarks");
    assert_eq!(resp.count, 1);
    assert_eq!(resp.results[0].id, 42);
}

#[test]
fn list_assets_handles_all_variants_including_unknown() {
    let (_rt, server) = with_mock_server(async {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/bookmarks/42/assets/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(ASSETS_PAYLOAD))
            .expect(1)
            .mount(&server)
            .await;
        server
    });

    let resp: ListBookmarkAssetsResponse = client(&server)
        .list_bookmark_assets(42)
        .expect("list_bookmark_assets");
    assert_eq!(resp.results.len(), 4);
    assert!(matches!(resp.results[0].asset_type, BookmarkAssetType::Snapshot));
    assert_eq!(resp.results[0].file_size, Some(12345));
    assert!(matches!(resp.results[1].asset_type, BookmarkAssetType::Upload));
    assert_eq!(resp.results[1].file_size, None);
    assert!(matches!(resp.results[2].asset_type, BookmarkAssetType::Precrawled));
    assert!(matches!(resp.results[3].asset_type, BookmarkAssetType::Unknown));
}

#[test]
fn download_asset_returns_raw_bytes_with_wildcard_accept() {
    let (_rt, server) = with_mock_server(async {
        let server = MockServer::start().await;
        let bytes = b"<html>snapshot</html>";
        Mock::given(method("GET"))
            .and(path("/api/bookmarks/42/assets/1/download/"))
            .and(header("accept", "*/*"))
            .respond_with(ResponseTemplate::new(200).set_body_bytes(bytes.to_vec()))
            .expect(1)
            .mount(&server)
            .await;
        server
    });

    let result = client(&server)
        .download_bookmark_asset(42, 1)
        .expect("download_bookmark_asset");
    assert_eq!(result, b"<html>snapshot</html>");
}

#[test]
fn http_error_surfaces_as_send_http_error() {
    let (_rt, server) = with_mock_server(async {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/bookmarks/999/"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&server)
            .await;
        server
    });

    let err = client(&server)
        .get_bookmark(999)
        .expect_err("expected error on 404");
    assert!(matches!(err, linkding::LinkDingError::SendHttpError(_)));
}
