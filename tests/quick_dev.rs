use anyhow::Result;

#[tokio::test]
async fn test() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:8080")?;
    hc.do_get("/api/user/me").await?.print().await?;
    hc.do_get("/api/user/all").await?.print().await?;
    
    Ok(())
}
