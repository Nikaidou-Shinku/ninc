#![allow(non_snake_case)]

pub const CONFIG_FILE: &str = "config.toml";
pub const STORAGE_FILE: &str = "storage.json";

pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/102.0.5005.149 Safari/537.36";

pub const AUTH_URL: &str = "https://uis.nwpu.edu.cn/cas/login";
pub const SERVICE_HOME: &str = "https://ecampus.nwpu.edu.cn/?path=https://ecampus.nwpu.edu.cn/main.html#/Tourist";
pub const SERVICE_ESREP: &str = "https://yqtb.nwpu.edu.cn//sso/login.jsp?targetUrl=base64aHR0cHM6Ly95cXRiLm53cHUuZWR1LmNuLy93eC94Zy95ei1tb2JpbGUvaW5kZXguanNw";

pub const USER_URL: &str = "https://personal-security-center.nwpu.edu.cn/api/v1/personal/me/user";

pub const _ESREP_HOME_URL: &str = "https://yqtb.nwpu.edu.cn/wx/xg/yz-mobile/index.jsp";
pub const ESREP_LOGIN_URL: &str = "https://yqtb.nwpu.edu.cn/sso/login.jsp";
pub const ESREP_URL: &str = "https://yqtb.nwpu.edu.cn/wx/ry/jrsb_xs.jsp";
pub const ESREP_POST_URL: &str = "https://yqtb.nwpu.edu.cn/wx/ry/ry_util.jsp";

pub const ECARD_URL: &str = "https://portal-service.nwpu.edu.cn/v2/personalData/getMyECard";
pub const ECARD_DETAIL_URL: &str = "https://portal-service.nwpu.edu.cn/portalCenter/api/rest/center/personalData/getMyCost";

pub const EDU_LOGIN_URL: &str = "https://jwxt.nwpu.edu.cn/student/sso-login";
pub const EDU_EVAL_INIT: &str = "https://jwxt.nwpu.edu.cn/student/for-std/evaluation/summative";
pub const EDU_EVAL_AUTH: &str = "https://jwxt.nwpu.edu.cn/evaluation-student-backend/api/v1/evaluation/token-check";
pub const EDU_EVAL_SEME: &str = "https://jwxt.nwpu.edu.cn/evaluation-student-backend/api/v1/evaluation/get-enable-semesters";

pub fn EDU_EVAL_LIST(seme: i32) -> String {
  format!("https://jwxt.nwpu.edu.cn/evaluation-student-backend/api/v1/student/summative-evaluation/task/semester/{}", seme)
}
pub fn _EDU_EVAL(id: i32) -> String {
  format!("https://jwxt.nwpu.edu.cn/evaluation-student-backend/api/v1/evaluation/survey/{}", id)
}

pub const _PUBKEY_URL: &str = "https://uis.nwpu.edu.cn/cas/jwt/publicKey";
