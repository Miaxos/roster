mod utils;
use std::collections::HashMap;

use redis_async::resp::RespValue;
use redis_async::resp_array;

#[tokio::test]
#[ignore = "redis-async doesn't support map from resp 3 properly"]
pub async fn hello() {
    let addr = utils::start_simple_server();

    let connection = utils::connect_without_auth(addr).await;

    let _res_f: HashMap<String, RespValue> =
        connection.send(resp_array!["HELLO"]).await.unwrap();

    assert!(false);
}
