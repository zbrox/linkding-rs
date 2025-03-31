use std::{fs::File, io::Write};

use linkding::LinkDingClient;

fn main() {
    let linkding_host =
        std::env::var("LINKDING_HOST").unwrap_or("http://localhost:9090".to_string());
    let linkding_token =
        std::env::var("LINKDING_TOKEN").expect("LINKDING_TOKEN env variable is not set");
    let linkding_client = LinkDingClient::new(&linkding_host, &linkding_token);

    let result = linkding_client
        .download_bookmark_asset(1, 1)
        .expect("Could not download asset");

    let mut asset_file =
        File::create("asset.html").expect("Could not create/overwrite the file asset.html");
    asset_file
        .write(&result)
        .expect("Could not write to asset file");
}
