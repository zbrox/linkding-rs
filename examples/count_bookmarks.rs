use linkding::{LinkDingClient, ListBookmarksArgs};

fn main() {
    let linkding_host =
        std::env::var("LINKDING_HOST").unwrap_or("http://localhost:9090".to_string());
    let linkding_token =
        std::env::var("LINKDING_TOKEN").expect("LINKDING_TOKEN env variable is not set");
    let linkding_client = LinkDingClient::new(&linkding_host, &linkding_token);

    let mut total_bookmarks = 0;
    let mut offset = 0;

    loop {
        let response = linkding_client
            .list_bookmarks(ListBookmarksArgs {
                query: None,
                limit: None,
                offset: Some(offset),
            })
            .expect("Couldn't fetch bookmarks");
        total_bookmarks += response.results.len();
        if response.next.is_none() {
            break;
        }
        offset += 100;
    }

    println!("Total bookmarks: {}", total_bookmarks);
}
