mod utils;
use redis_async::resp_array;

#[tokio::test]
pub async fn test_simple_client_setinfo() {
    let addr = utils::start_simple_server();

    let connection = utils::connect_without_auth(addr).await;

    let res_f: String = connection
        .send(resp_array!["CLIENT", "SETINFO", "LIB-NAME", "roster"])
        .await
        .unwrap();

    // TODO: need to check connections
    assert_eq!(res_f, "OK");
}

#[tokio::test]
pub async fn test_simple_client_help() {
    let addr = utils::start_simple_server();

    let connection = utils::connect_without_auth(addr).await;

    let res_f: Vec<String> = connection
        .send(resp_array!["CLIENT", "HELP"])
        .await
        .unwrap();

    let joined = res_f.join("\n");

    insta::assert_display_snapshot!(joined, @r###"
    CLIENT <subcommand> [<arg> [value] [opt] ...]. subcommands are:
    ID
        Return the ID of the current connection.
    LIST [options ...]
        Return information about client connections. Options:
        * TYPE (NORMAL|MASTER|REPLICA|PUBSUB)
          Return clients of specified type.
    SETNAME <name>
        Assign the name <name> to the current connection.
    SETINFO <option> <value>
        Set client meta attr. Options are:
        * LIB-NAME: the client lib name.
        * LIB-VER: the client lib version.
    HELP
        Print this help.
    "###);
}
