use anyhow::Result;
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use reqwest::Client;
use crate::data::{Storage, EDU_EVAL_INIT, EDU_EVAL_AUTH, Resp};
use super::edu;

#[allow(dead_code)]
#[derive(Deserialize)]
struct RespData {
  token: String,
  url: String,
}

// Auth eval system
// - GET https://jwxt.nwpu.edu.cn/student/for-std/evaluation/summative with SESSION & __pstsid__
// - Regex https://jwxt.nwpu.edu.cn/evaluation-student-frontend/#/sso-login?token=${JWT}
// - POST https://jwxt.nwpu.edu.cn/evaluation-student-backend/api/v1/evaluation/token-check with {"token":"${JWT}"}
// - Parse res["data"]["token"] as ${TOKEN}
// Visit with
// - Authorization: ${TOKEN}
// - Cookie: student_evaluation_token=${TOKEN}
pub async fn auth(storage: &Storage) -> Result<String> {
  let token = edu::auth(storage).await?;
  let client = Client::new();

  let resp = client.get(EDU_EVAL_INIT)
    .header("Cookie", format!("SESSION={}; __pstsid__={}", token.session, token.pstsid))
    .send().await?
    .text().await?;
  let re = Regex::new(r"https://jwxt\.nwpu\.edu\.cn/evaluation-student-frontend/#/sso-login\?token=(.+?)'")?;
  let jwt = re.captures(&resp).unwrap().get(1).unwrap().as_str();

  let resp = client.post(EDU_EVAL_AUTH)
    .json(&json!({"token": jwt}))
    .send().await?
    .text().await?;
  let res: Resp<RespData> = serde_json::from_str(&resp)?;

  Ok(res.data.token)
}
