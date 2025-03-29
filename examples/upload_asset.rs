use std::io::Read;

use linkding::LinkDingClient;

fn main() {
    let linkding_host =
        std::env::var("LINKDING_HOST").unwrap_or("http://localhost:9090".to_string());
    let linkding_token =
        std::env::var("LINKDING_TOKEN").expect("LINKDING_TOKEN env variable is not set");
    let linkding_client = LinkDingClient::new(&linkding_host, &linkding_token);

    let mut asset_file = std::fs::File::open("examples/asset.txt").unwrap();
    let mut buffer: Vec<u8> = vec![];
    asset_file.read_to_end(&mut buffer).unwrap();

    linkding_client.upload_bookmark_asset(1, &buffer).unwrap();
}
