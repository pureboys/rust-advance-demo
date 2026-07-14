use tracing::Level;
use tracing_subscriber::FmtSubscriber;

mod constant;
mod llm;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    let content =
        llm::complete::chat_complete(constant::QWEN_3_MAX, Some("你是一个数据专家"), "1+1=?")
            .await?;
    println!("content: {}", content);

    Ok(())
}
