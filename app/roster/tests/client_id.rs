mod utils;
use redis_async::resp_array;

#[tokio::test]
pub async fn client_id() {
    let addr = utils::start_simple_server();

    let connection = utils::connect_without_auth(addr).await;

    let res_f: String =
        connection.send(resp_array!["CLIENT", "ID"]).await.unwrap();

    assert_eq!(res_f, "0");

    let res_f: String =
        connection.send(resp_array!["CLIENT", "ID"]).await.unwrap();

    assert_eq!(res_f, "0");

    drop(connection);

    let connection = utils::connect_without_auth(addr).await;

    let res_f: String =
        connection.send(resp_array!["CLIENT", "ID"]).await.unwrap();

    assert_eq!(res_f, "1");
}
