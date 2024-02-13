use std::str::FromStr;

use bytes::Bytes;
use h2::client;
use http::header::CONTENT_TYPE;
use http::{Method, Request};
use tokio::net::TcpStream;

mod utils;

#[tokio::test]
#[ntest::timeout(10_000)]
pub async fn test_simple_flow() -> anyhow::Result<()> {
    let addr = utils::start_simple_server().await?;

    let tcp = TcpStream::connect(&addr).await?;
    let (mut client, h2) = client::handshake(tcp).await?;

    let req = Request::builder()
        .method(Method::PUT)
        .uri(format!("http://{addr}/new-buck"))
        .header(CONTENT_TYPE, "application/xml")
        .body(())?;

    tokio::spawn(async move {
        if let Err(e) = h2.await {
            println!("GOT ERR={:?}", e);
        }
    });

    let (response, mut stream) = client.send_request(req, false).unwrap();
    let a = r###"
    <CreateBucketConfiguration xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
    </CreateBucketConfiguration>
        "###;
    stream.send_data(Bytes::from(a), true)?;

    let res = response.await?;
    assert_eq!(res.status().as_u16(), 200);

    Ok(())
}
