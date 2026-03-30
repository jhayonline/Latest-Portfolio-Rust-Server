use dotenvy::dotenv;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello Universe");

    todo!()
}
