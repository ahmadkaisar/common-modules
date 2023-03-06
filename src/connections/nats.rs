use futures::StreamExt;
use mockall;

pub struct Nats {
  pub nats_url: String,
  pub client: Option<async_nats::Client>,
}

#[mockall::automock]
impl Nats {
  pub async fn connect(&mut self) {
    self.client = match async_nats::connect(self.nats_url.to_string()).await {
      Ok(client) => Some(client),
      Err(_)=> None,
    }
  }

  pub async fn publish(&self, subject: String, msg: String) -> Result<(), std::io::Error> {
    if self.client.is_none() {
      return Err(std::io::ErrorKind::NotConnected.into());
    }

    let client = self.client.as_ref().unwrap();
    match client.publish(subject, msg.into()).await {
      Ok(_) => Ok(()),
      Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "error on publishing message")),
    }
  }

  pub async fn subscribe(&self, subject: String) -> Option<async_nats::Message> {
    if self.client.is_none() {
      return None;
    }

    let client = self.client.as_ref().unwrap();
    let mut subscriber = client.subscribe(subject).await.unwrap().take(1);

    while let Some(msg) = subscriber.next().await {
      println!("{:?}", msg);
    }

    None
  }
}