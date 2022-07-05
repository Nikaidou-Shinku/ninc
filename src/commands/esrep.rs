use std::io::Write;
use anyhow::{bail, Result};
use serde::Serialize;
use serde_json::Value;
use regex::Regex;
use reqwest::Client;
use crate::{
  data::{
    Storage,
    Config,
    ESREP_URL,
    ESREP_POST_URL,
    EsrepReport,
    CONFIG_FILE, EsrepConfig
  },
  tools::auth::esrep
};

#[allow(non_snake_case)]
#[derive(Serialize)]
struct Params {
  sign: String,
  timeStamp: String,
}

struct Res {
  already: bool,
  params: Params,
}

async fn fetch_params(jsessionid: &str, client: &Client) -> Result<Res> {
  let re = Regex::new(r"url:'ry_util\.jsp\?sign=(.+)&timeStamp=(\d+)',")?;

  let html = client.get(ESREP_URL)
    .header("Cookie", format!("JSESSIONID={}", jsessionid))
    .send().await?
    .text().await?;

  let already = html.find("您已提交今日填报，重新提交将覆盖上一次的信息。").is_some();

  let res = re.captures(&html).unwrap();

  Ok(Res {
    already,
    params: Params {
      sign: res.get(1).unwrap().as_str().to_string(),
      timeStamp: res.get(2).unwrap().as_str().to_string(),
    }
  })
}

async fn report(
  content: &EsrepReport,
  is_yes: bool,
  jsessionid: &str,
) -> Result<()> {
  let client = Client::new();
  let res = fetch_params(jsessionid, &client).await?;

  if res.already && !is_yes {
    loop {
      print!("Already reported today. Are you sure to report again? (Y/n) ");
      std::io::stdout().flush()?;
      let mut input = String::new();
      std::io::stdin().read_line(&mut input)?;
      match input.trim() {
        "Y" | "y" => {
          break;
        },
        "N" | "n" => {
          bail!("Aborted.");
        },
        _ => { }
      }
    }
  }

  let raw = client.post(ESREP_POST_URL)
    .query(&res.params)
    .form(&content)
    .header("Cookie", format!("JSESSIONID={}", jsessionid))
    .header("Referer", ESREP_URL)
    .send().await?
    .text().await?
    .replace("－", "-"); // 😅

  let res: Value = serde_json::from_str(&raw)?;

  match &res["state"] {
    Value::String(code) => {
      if code == "1" {
        Ok(())
      } else {
        bail!("{}", res)
      }
    },
    Value::Number(code) => {
      if code.is_i64() {
        if code.as_i64().unwrap() == -1 {
          bail!("{}", res["err-msg"].as_str().unwrap());
        } else {
          bail!("{}", res);
        }
      } else {
        bail!("{}", res);
      }
    },
    _ => {
      bail!("{}", res);
    }
  }
}

pub async fn esrep(
  config: &mut Config,
  storage: &Storage,
  is_yes: bool
) -> Result<()> {
  if config.esrep.is_none() {
    config.esrep = Some(EsrepConfig {
      report: Some(EsrepReport::new())
    });
    config.save(CONFIG_FILE).await?;
    bail!("Please set the report content in config file first!");
  }
  let esrep = config.esrep.as_mut().unwrap();
  if esrep.report.is_none() {
    esrep.report = Some(EsrepReport::new());
    config.save(CONFIG_FILE).await?;
    bail!("Please set the report content in config file first!");
  }
  let content = esrep.report.as_ref().unwrap();
  let jsessionid = esrep::auth(&storage).await?;
  report(&content, is_yes, &jsessionid).await?;
  Ok(())
}
