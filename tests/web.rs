use httpmock::Method::GET;
use httpmock::MockServer;
use opds_client::OpdsClient;

#[tokio::test]
async fn test_opds_server_interaction() {
    let server = MockServer::start();

    let opds_mock = server.mock(|when, then| {
        when.method(GET).path("/catalog");
        then.status(200).body("<feed></feed>");
    });

    let client = OpdsClient::new(String::from("AA"), None);
    let response = client.fetch_feed("/catalog").await;
    opds_mock.assert();
    assert!(response.is_ok());
}
/*
#[test]
fn test_opds_server_interaction_with_auth() {
    // Arrange: Start a mock server.
    let server = MockServer::start();

    // Create a mock on the server.
    let mock: MockRef = mock(GET, "/feed")
        .with_status(200)
        .with_header("content-type", "application/atom+xml")
        .expect_header("Authorization", "Bearer some_token") // Expecting an authorization token
        .with_body_from_file("tests/resources/feed.xml") // assuming you have a file with pre-canned response
        .create_on(&server);

    // Use the mock server in your crate by pointing it to the mock server
    // Instead of the actual OPDS server
    let opds_server_url = server.url("/feed");

    // Act: Now call your crate function that is supposed to interact with the OPDS server.
    // Make sure your function also adds the appropriate headers for authentication.
    let result = your_crate::fetch_feed(&opds_server_url, "Bearer some_token");

    // Assert: Check if the function has behaved as expected.
    assert_eq!(result.is_ok(), true);
    mock.assert_hits(1); // asserting that endpoint was hit exactly once
}
*/
