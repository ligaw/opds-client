use dotenv::dotenv;
use opds_client::Auth;
use opds_client::OpdsServer;

fn main() {
    dotenv().ok();
    let server_url = std::env::var("OPDS_SERVER_URL").expect("OPDS_SERVER_URL not set");
    let username = std::env::var("OPDS_USERNAME").expect("OPDS_USERNAME not set");
    let password = std::env::var("OPDS_PASSWORD").expect("OPDS_PASSWORD not set");

    let client = OpdsServer::new(server_url, Some(Auth::Basic(username, password)));

    let feed = client.catalog();
    println!("{:#?}", feed);
}
