#[test]
fn server_client() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {

            /////////////////////////////////////////////////

            let s = crate::server::Server::bind("0.0.0.0:5515");
            let listener = s.listen().await.unwrap();
            let cfut = tokio::spawn(async {
                let mut client = crate::client::connect("0.0.0.0:5515").await.unwrap();
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


            /////////////////////////////////////////////////
    });
}

// we just assume this works, because i keep needing to wrestle TLS errors otherwise.
#[test]
#[ignore]
fn server_client_tls() {
    use std::{io, sync::Arc};
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {

            ///////////////////////////////////////
        
            use tokio_rustls::rustls;
            let server_config = {
                fn load_certs(filename: &str) -> Vec<rustls::Certificate> {
                    let certfile = std::fs::File::open(filename).expect("cannot open certificate file");
                    let mut reader = std::io::BufReader::new(certfile);
                    rustls_pemfile::certs(&mut reader)
                        .unwrap()
                        .iter()
                        .map(|v| rustls::Certificate(v.clone()))
                        .collect()
                }

                fn load_private_key(filename: &str) -> rustls::PrivateKey {
                    let keyfile = std::fs::File::open(filename).expect("cannot open private key file");
                    let mut reader = std::io::BufReader::new(keyfile);

                    loop {
                        match rustls_pemfile::read_one(&mut reader).expect("cannot parse private key .pem file") {
                            Some(rustls_pemfile::Item::RSAKey(key)) => return rustls::PrivateKey(key),
                            Some(rustls_pemfile::Item::PKCS8Key(key)) => return rustls::PrivateKey(key),
                            Some(rustls_pemfile::Item::ECKey(key)) => return rustls::PrivateKey(key),
                            None => break,
                            _ => {}
                        }
                    }

                    panic!(
                        "no keys found in {:?} (encrypted keys not supported)",
                        filename
                    );
                }
                let certs = load_certs("cert.pem");
                let key = load_private_key("key.pem");
                rustls::ServerConfig::builder()
                    .with_safe_defaults()
                    .with_no_client_auth()
                    .with_single_cert(certs, key)
                    .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err)).unwrap()
            };

            let mut root_cert_store = rustls::RootCertStore::empty();
            root_cert_store.add_server_trust_anchors(
                webpki_roots::TLS_SERVER_ROOTS
                    .0
                    .iter()
                    .map(|ta| {
                        rustls::OwnedTrustAnchor::from_subject_spki_name_constraints(
                            ta.subject,
                            ta.spki,
                            ta.name_constraints,
                        )
                    })
            );
            
            
            let client_config = rustls::ClientConfig::builder()
                .with_safe_defaults()
                .with_root_certificates(root_cert_store)
                .with_no_client_auth();

            let s = crate::server::tls::Server::bind("localhost:5515");
            let listener = s.listen(Arc::new(server_config)).await.unwrap();
            let cfut = tokio::spawn(async move {
                let mut client = crate::client::tls::connect("localhost:5515", Arc::new(client_config)).await.unwrap();
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


            /////////////////////////////////////////////////
    });
}