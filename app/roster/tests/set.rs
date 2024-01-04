mod utils;
use redis_async::resp_array;

#[tokio::test]
pub async fn test_simple_client_set_key() {
    let addr = utils::start_simple_server();

    let connection = utils::connect_without_auth(addr).await;

    let res_f: String = connection
        .send(resp_array!["SET", "mykey", "hello"])
        .await
        .unwrap();

    assert_eq!(res_f, "OK");

    let res_f: String =
        connection.send(resp_array!["GET", "mykey"]).await.unwrap();

    // TODO: need to check connections
    assert_eq!(res_f, "hello");
}
