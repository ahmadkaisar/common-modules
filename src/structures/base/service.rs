pub trait Service {
  fn create(self);
  fn update(self);
  fn delete(self);
  fn validate(self);
}