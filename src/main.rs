use dotenvy::dotenv;
use openai_cli_rust::fetch;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let resp = fetch().await?;
    if let Some(choices) = resp.choices {
        for choice in choices.iter() {
            println!("{:?}", choice.text);
        }
    }

    Ok(())
}
