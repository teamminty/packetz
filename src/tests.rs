use crate as packetz;

#[test]
fn server_client() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let s = packetz::server::Server::bind("0.0.0.0:5515");
            s.listen().await.unwrap();
    });
    
}