use tokio_postgres::NoTls;

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./files");
}

#[tokio::main()]
async fn main() -> eyre::Result<()> {
    let (mut client, conn) = tokio_postgres::connect(
        "postgres://user:password@localhost:8001/messaging?sslmode=disable",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            eprintln!("connection error: {}", e);
        }
    });

    let report = embedded::migrations::runner()
        .run_async(&mut client)
        .await?;

    println!("{:?}", report);

    Ok(())
}
