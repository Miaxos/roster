mod utils;
use redis::ConnectionLike;

#[tokio::test]
pub async fn test_start_simple_server() {
    let addr = utils::debug_server();

    let client = utils::connect_without_auth(addr).await;
    let mut con = client.get_connection().unwrap();

    assert!(con.check_connection());
}
