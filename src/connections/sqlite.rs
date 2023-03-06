/*
    Intented for unittest.
    Don't use it on Prod.
*/

use sea_orm::{Database, DatabaseConnection};

pub struct SQLite {
  pub db_url: String,
  pub connection: Option<DatabaseConnection>,
}

impl SQLite {
  pub async fn connect(&mut self) {
    match Database::connect(&self.db_url).await {
      Ok(value) => self.connection = Some(value),
      Err(err) => {
        println!("failed to init DB connection {:?}", err.to_string());
        self.connection = None;
      }
    }
  }
}