mod utils;
use redis_async::resp_array;
use regex::Regex;

#[tokio::test]
pub async fn client_info() {
    let test_re: Regex =
        Regex::new(r"^id=0 addr=.*? laddr=.*? fd=.*? name=$").unwrap();
    let addr = utils::start_simple_server();

    let connection = utils::connect_without_auth(addr).await;

    let res_f: String = connection
        .send(resp_array!["CLIENT", "INFO"])
        .await
        .unwrap();

    assert!(test_re.is_match(&res_f));
}
