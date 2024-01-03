mod utils;

use bytes::Bytes;
use redis_async::resp::RespValue;
use redis_async::resp_array;

#[tokio::test]
pub async fn test_start_simple_server_ping() {
    let addr = utils::debug_server();

    let connection = utils::connect_without_auth(addr).await;

    let res_f: String = connection.send(resp_array!["PING"]).await.unwrap();

    assert_eq!(res_f, "PONG");
}

#[tokio::test]
pub async fn test_start_simple_server_ping_msg() {
    let addr = utils::debug_server();

    let connection = utils::connect_without_auth(addr).await;

    let res_f: String =
        connection.send(resp_array!["PING", "msg"]).await.unwrap();

    assert_eq!(res_f, "msg");
}
