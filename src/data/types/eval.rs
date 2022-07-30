use serde::Deserialize;

#[derive(Deserialize)]
pub struct Resp<T> {
  pub data: T,
  pub message: Option<String>,
  pub result: i32,
}
