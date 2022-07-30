use std::collections::HashMap;

use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;
use reqwest::Client;
use crate::{
  data::{args::edu::eval::{EvalCommands, ListArgs},
  EDU_EVAL_SEME,
  Storage,
  Resp,
  EDU_EVAL_LIST},
  tools::auth::eval,
};

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize)]
struct SemeData {
  code: String,
  countInTerm: bool,
  endDate: String,
  id: i32,
  nameEn: String,
  nameZh: String,
  schoolYear: String,
  season: String,
  startDate: String,
  weekIndices: Vec<i32>,
  weekStartOnSunday: bool,
}

async fn seme(storage: &Storage) -> Result<()> {
  let token = eval::auth(storage).await?;
  let client = Client::new();

  let resp: Resp<Vec<SemeData>> = client.get(EDU_EVAL_SEME)
    .header("Authorization", &token)
    .header("Cookie", format!("student_evaluation_token={}", token))
    .send().await?
    .json().await?;

  for item in resp.data {
    println!("- id: {}\n  name: {}", item.id, item.nameZh);
  }
  Ok(())
}

async fn list(storage: &Storage, args: ListArgs) -> Result<()> {
  let token = eval::auth(storage).await?;
  let client = Client::new();

  let url = EDU_EVAL_LIST(args.seme);
  let resp: Value = client.get(url)
    .header("Authorization", &token)
    .header("Cookie", format!("student_evaluation_token={}", token))
    .send().await?
    .json().await?;

  let mut res: HashMap<String, Vec<(i64, String)>> = HashMap::new();

  for item in resp.get("data").unwrap().as_array().unwrap() {
    let id = item.get("id").unwrap().as_i64().unwrap();
    let lesson = item.get("lesson").unwrap()
      .get("course").unwrap()
      .get("nameZh").unwrap()
      .as_str().unwrap();
    let teacher = item.get("person").unwrap()
      .get("nameZh").unwrap()
      .as_str().unwrap()
      .to_string();

    if res.contains_key(lesson) {
      res.get_mut(lesson).unwrap().push((id, teacher));
    } else {
      res.insert(lesson.to_string(), vec![(id, teacher)]);
    }
  }

  for item in res {
    println!("- {}", item.0);
    for teacher in item.1 {
      println!("    {}: {}", teacher.0, teacher.1);
    }
    println!();
  }

  Ok(())
}

pub async fn eval(storage: &Storage, cmd: EvalCommands) -> Result<()> {
  match cmd {
    EvalCommands::Seme => {
      seme(storage).await?;
    }
    EvalCommands::List(args) => {
      list(storage, args).await?;
    }
  }
  Ok(())
}
