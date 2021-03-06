use serde::{Serialize, Deserialize};
use crate::data::Storage;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
pub struct ReportAtSchool {
  #[doc = "近 48 小时内是否进行过核酸检测。"]
  #[doc = "0: 否，1: 是。"]
  #[doc = "目前无需填写，缺省为 1。"]
  hsjc: Option<i32>,

  #[doc = "西安市一码通颜色。"]
  #[doc = "1: 绿码，2: 黄码，3: 红码。"]
  xasymt: Option<i32>,

  #[doc = "不知道什么玩意儿，缺省为 addRbxx。"]
  actionType: Option<String>,

  #[doc = "学号。"]
  userLoginId: Option<String>,

  #[doc = "当前所在位置。"]
  #[doc = "1: 在学校，2: 在西安，3: 在国内，4: 在国外。"]
  szcsbm: Option<i32>,

  #[doc = "不知道什么玩意儿，缺省为 1。"]
  bdzt: Option<i32>,

  #[doc = "当前所在位置具体地址。"]
  szcsmc: Option<String>,

  #[doc = "今天的体温范围。"]
  #[doc = "0: 37.3 度以下，1: 37.3 度 ~ 37.8 度，2: 37.8 ~ 39.0 度，3: 39.0 度以上。"]
  sfyzz: Option<i32>,

  #[doc = "疑似症状。"]
  #[doc = "用半角逗号分隔的列表。"]
  #[doc = "0: 无，1: 发热，6: 胸闷，7: 咳嗽，8: 其他症状（咽痛、呼吸困难、乏力、恶心呕吐、腹泻、结膜炎、肌肉酸痛等症状）。"]
  sfqz: Option<String>,

  #[doc = "不知道什么玩意儿，看名字像是填报来源(?)，缺省为 sso。"]
  tbly: Option<String>,

  #[doc = "其它情况说明。"]
  qtqksm: Option<String>,

  #[doc = "疑似症状情况说明。"]
  #[doc = "选无疑似症状不会出现这个框，但是不知道为啥抓包抓到了，不知道他们前端写的什么锤子。"]
  ycqksm: Option<String>,

  #[doc = "不知道什么玩意儿，看名字像是用户类型(?)，缺省为 2。"]
  userType: Option<i32>,

  #[doc = "姓名。"]
  userName: Option<String>,
}

impl ReportAtSchool {
  pub fn new(storage: &Storage) -> Self {
    let uid = if let Some(uid) = storage.basic.uid.as_ref() {
      uid.clone()
    } else {
      String::from("1145141919")
    };

    let name = if let Some(name) = storage.basic.name.as_ref() {
      name.clone()
    } else {
      String::from("田所")
    };

    Self {
      hsjc: Some(1),
      xasymt: Some(1),
      actionType: Some(String::from("addRbxx")),
      userLoginId: Some(uid),
      szcsbm: Some(1),
      bdzt: Some(1),
      szcsmc: Some(String::from("在学校")),
      sfyzz: Some(0),
      sfqz: Some(String::from("0")),
      tbly: Some(String::from("sso")),
      qtqksm: Some(String::from("无")),
      ycqksm: Some(String::new()),
      userType: Some(2),
      userName: Some(name),
    }
  }
}
