use mockall;
use std::io::{Error, ErrorKind};
use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub struct MySQL {
  pub db_url: String,
  pub connection: Option<DatabaseConnection>,
}

#[mockall::automock]
impl MySQL {
  pub async fn connect(&mut self) {
    let mut opt = ConnectOptions::new(self.db_url.to_owned());
    opt.max_connections(100)
      .min_connections(5)
      .connect_timeout(Duration::from_secs(8))
      .idle_timeout(Duration::from_secs(8))
      .max_lifetime(Duration::from_secs(8))
      .sqlx_logging(true);

    self.connection = match Database::connect(opt).await {
      Ok(connection) => Some(connection),
      Err(_) => {
        println!("{:?}", Error::new(ErrorKind::Other, "Failed to connect to DB"));
        None
      }
    }
  }
}
