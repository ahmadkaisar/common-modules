#[cfg(test)]
mod test {
  use mockall_double;

  #[mockall_double::double]
  use crate::connections::nats::Nats;

  #[tokio::test]
  async fn it_should_connect() {
    let mut mock = Nats::default();
    mock.expect_connect()
        .times(1)
        .return_const(());
    
    mock.connect().await;
  }

  #[tokio::test]
  async fn it_should_publish() {
    let mut mock = Nats::default();
    mock.expect_publish()
        .times(1)
        .return_once(move |_, _| Ok(()));

    assert_eq!(mock.publish("this.is.subject".to_string(), "This is the message".to_string()).await.unwrap(), ());
  }
}