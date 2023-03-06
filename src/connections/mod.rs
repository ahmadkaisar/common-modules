pub mod mysql;
pub mod nats;
pub mod sqlite;
pub mod test_mysql;
pub mod test_nats;

use sea_orm::{DatabaseConnection};

#[derive(Clone)]
pub struct Connection (pub DatabaseConnection);