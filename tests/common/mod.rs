/// Sample list-bookmark-assets payload exercising every asset type variant
/// (including a forward-looking `weird_future_type` to verify `Unknown` parsing)
/// and an asset with an explicit `file_size` field.
pub const ASSETS_PAYLOAD: &str = r#"{
    "count": 4,
    "next": null,
    "previous": null,
    "results": [
        {
            "id": 1,
            "bookmark": 42,
            "asset_type": "snapshot",
            "date_created": "2026-04-01T12:00:00Z",
            "content_type": "text/html",
            "display_name": "snapshot.html",
            "status": "complete",
            "file_size": 12345
        },
        {
            "id": 2,
            "bookmark": 42,
            "asset_type": "upload",
            "date_created": "2026-04-02T12:00:00Z",
            "content_type": "application/pdf",
            "display_name": "paper.pdf",
            "status": "complete"
        },
        {
            "id": 3,
            "bookmark": 42,
            "asset_type": "precrawled",
            "date_created": "2026-04-03T12:00:00Z",
            "content_type": "text/html",
            "display_name": "precrawled.html",
            "status": "pending"
        },
        {
            "id": 4,
            "bookmark": 42,
            "asset_type": "weird_future_type",
            "date_created": "2026-04-04T12:00:00Z",
            "content_type": "application/octet-stream",
            "display_name": "future.bin",
            "status": "complete"
        }
    ]
}"#;

pub const SINGLE_BOOKMARK_PAYLOAD: &str = r#"{
    "id": 42,
    "url": "https://example.com",
    "title": "Example",
    "description": "An example bookmark",
    "notes": "",
    "web_archive_snapshot_url": "",
    "favicon_url": null,
    "preview_image_url": null,
    "is_archived": false,
    "unread": false,
    "shared": false,
    "tag_names": ["rust", "linkding"],
    "date_added": "2026-04-01T00:00:00Z",
    "date_modified": "2026-04-15T00:00:00Z",
    "website_title": "Example",
    "website_description": null
}"#;
