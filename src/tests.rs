use crate as packetz;

#[test]
fn server_client() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let s = packetz::server::Server::bind("0.0.0.0:5515");
            let listener = s.listen().await.unwrap();
            let cfut = tokio::spawn(async {
                let mut client = packetz::client::connect("0.0.0.0:5515").await.unwrap();
                client.send(b"Hello, Packetz!").await.unwrap();
            });
            let (mut client_conn, _) = listener.accept().await.unwrap();
            assert_eq!(
                String::from_utf8(
                    client_conn.recv().await.unwrap().body
                ).unwrap(),
                "Hello, Packetz!".to_string()
            );
            cfut.await.unwrap();
    });
}