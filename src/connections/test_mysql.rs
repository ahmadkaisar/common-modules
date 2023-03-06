#[cfg(test)]
mod test {
  use mockall_double;

  #[mockall_double::double]
  use crate::connections::mysql::MySQL;

  #[tokio::test]
  async fn it_should_connect() {
    let mut mock = MySQL::default();
    mock.expect_connect()
        .once()
        .return_const(());
    
    mock.connect().await;
  }
}