/*
use hyper::header::CONTENT_TYPE;

mod utils;

#[tokio::test]
pub async fn test_simple_flow() -> anyhow::Result<()> {
    let addr = utils::start_simple_server().await?;

    let client = reqwest::Client::new();

    let res = client
        .put(format!("http://{addr}/new-buck"))
        .header(CONTENT_TYPE, "application/xml")
        .body(
            r###"
<CreateBucketConfiguration xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
</CreateBucketConfiguration>
    "###,
        )
        .send()
        .await?;

    assert_eq!(res.status().as_u16(), 200);

    Ok(())
}
*/
