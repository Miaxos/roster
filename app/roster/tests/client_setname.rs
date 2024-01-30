mod utils;
use redis_async::resp_array;
use regex::Regex;

#[tokio::test]
pub async fn client_setname() {
    let test_re: Regex =
        Regex::new(r"^id=0 addr=.*? laddr=.*? fd=.*? name=$").unwrap();
    let addr = utils::start_simple_server();

    let connection = utils::connect_without_auth(addr).await;

    let mut res_f: Vec<String> = connection
        .send(resp_array!["CLIENT", "LIST"])
        .await
        .unwrap();

    assert_eq!(res_f.len(), 1);
    let first_value = res_f.pop().unwrap();
    assert!(test_re.is_match(&first_value));

    let res_ok: String = connection
        .send(resp_array!["CLIENT", "SETNAME", "newname"])
        .await
        .unwrap();

    assert_eq!(res_ok, "OK");

    let mut res_f: Vec<String> = connection
        .send(resp_array!["CLIENT", "LIST"])
        .await
        .unwrap();

    let test_re: Regex =
        Regex::new(r"^id=0 addr=.*? laddr=.*? fd=.*? name=newname$").unwrap();
    assert_eq!(res_f.len(), 1);
    let first_value = res_f.pop().unwrap();
    assert!(test_re.is_match(&first_value));
}
