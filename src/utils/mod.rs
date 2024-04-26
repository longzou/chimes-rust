use std::fmt::Debug;
use std::time::SystemTime;

use chimes_utils::AppConfig;

use chrono::offset::Local;
use chrono::DateTime;
use jsonwebtoken::{DecodingKey, EncodingKey, Header};
use serde_derive::{Deserialize, Serialize};

use std::collections::HashMap;

mod auth;
pub use auth::*;

mod app;
pub use app::*;

#[allow(dead_code)]
pub fn num_to_string(n: i64) -> String {
    let base_codec = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
        'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '2', '3', '4', '5', '7', '8', '9',
    ];
    let len = base_codec.len() as i64;
    let mut t = n;
    let mut result = "".to_string();
    while t > 0 {
        let idx = (t % len) as usize;
        let ch = base_codec[idx];
        t /= len;
        result.insert(0, ch);
    }
    result
}

#[allow(dead_code)]
pub fn f32_to_decimal(f: f32) -> Option<rbatis::Decimal> {
    match rbatis::Decimal::from_str(format!("{:.2}", f).as_str()) {
        Ok(r) => Some(r),
        Err(_) => None,
    }
}

#[allow(dead_code)]
pub fn decimal_to_f32(dc: Option<rbatis::Decimal>) -> f32 {
    match dc {
        Some(r) => r.to_string().parse::<f32>().unwrap_or(0f32),
        None => 0f32,
    }
}

#[allow(dead_code)]
pub fn make_decimal_negative(dc: Option<rbatis::Decimal>) -> Option<rbatis::Decimal> {
    match dc {
        Some(r) => match r.to_string().parse::<f32>() {
            Ok(t) => f32_to_decimal(-t),
            Err(_) => f32_to_decimal(0f32),
        },
        None => f32_to_decimal(0f32),
    }
}

#[allow(dead_code)]
pub fn generate_rand_string(len: usize) -> String {
    let mut retkey = "".to_string();

    while retkey.len() < len {
        let rng = rand::random::<u16>();
        let key = num_to_string(rng as i64);
        retkey += key.as_str();
    }

    retkey.chars().take(len).collect()
}

#[allow(dead_code)]
pub fn get_local_timestamp() -> u64 {
    let now = SystemTime::now();
    let date: DateTime<Local> = now.into();
    date.timestamp_millis() as u64
}

#[allow(dead_code)]
pub fn parse_query(query_string: &str) -> HashMap<String, String> {
    if query_string.is_empty() {
        return HashMap::new();
    }
    let q_a: Vec<&str> = query_string.split('&').collect();
    let mut res: HashMap<String, String> = HashMap::new();
    use percent_encoding::percent_decode;
    for s in q_a {
        // let ss: &str = s;
        let kv: Vec<&str> = s.split('=').collect();
        let kvalue = percent_decode(kv[1].as_bytes()).decode_utf8().unwrap();
        res.insert(kv[0].to_string(), kvalue.to_string());
    }
    res
}

#[allow(dead_code)]
pub fn get_hash_value(query_params: &HashMap<String, String>, key: &str) -> String {
    match query_params.get(key) {
        Some(val) => val.clone(),
        None => "".to_owned(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserClaims {
    pub aud: String,
    pub sub: String,
    pub exp: usize,
}

impl UserClaims {
    pub fn encode(&self) -> Option<String> {
        let conf = AppConfig::get().lock().unwrap().to_owned();

        match jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(conf.webserver_conf.rsa_cert.as_bytes()),
        ) {
            Ok(t) => Some(t),
            Err(_) => None,
        }
    }

    pub fn decode(token: &str) -> Option<Self> {
        let conf = AppConfig::get().lock().unwrap().to_owned();
        let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
        match jsonwebtoken::decode::<UserClaims>(
            token,
            &DecodingKey::from_secret(conf.webserver_conf.rsa_cert.as_bytes()),
            &validation,
        ) {
            Ok(c) => Some(c.claims),
            Err(err) => {
                match *err.kind() {
                    jsonwebtoken::errors::ErrorKind::InvalidToken => {
                        log::error!("Token is invalid")
                    } // Example on how to handle a specific error
                    jsonwebtoken::errors::ErrorKind::InvalidIssuer => {
                        log::error!("Issuer is invalid")
                    } // Example on how to handle a specific error
                    _ => log::error!("Some other errors"),
                };

                None
            }
        }
    }
}

fn field_is_none<T>(t: &Option<T>) -> bool {
    t.is_none()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MenuMetadata {
    pub icon: Option<String>,
    pub no_cache: bool,
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MenuTreeModel {
    #[serde(skip_serializing_if = "field_is_none")]
    pub always_show: Option<bool>,
    pub id: i64,
    pub meta: MenuMetadata,
    pub name: String,
    pub path: String,
    pub hidden: bool,
    pub iframe: bool,
    pub permission: String,
    pub component: String,
    #[serde(skip_serializing_if = "field_is_none")]
    pub pid: Option<i64>,
    pub children: Vec<MenuTreeModel>,
    #[serde(skip_serializing_if = "field_is_none")]
    pub redirect: Option<String>,
    pub sort: i32,
}

#[allow(dead_code)]
pub fn num_to_string_v2(n: i64) -> String {
    let base_codec = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't',
        'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J', 'K', 'L', 'M',
        'N', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '2', '3', '4', '5', '6', '7',
        '8', '9',
    ];
    let len = base_codec.len() as i64;
    let mut t = n;
    let mut result = "".to_string();
    while t > 0 {
        let idx = (t % len) as usize;
        let ch = base_codec[idx];
        t /= len;
        result.insert(0, ch);
    }
    result
}

#[allow(dead_code)]
pub fn generate_rand_string_v2(len: usize) -> String {
    let mut retkey = "".to_string();

    while retkey.len() < len {
        let rng = rand::random::<u16>();
        let key = num_to_string_v2(rng as i64);
        retkey += key.as_str();
    }

    retkey.chars().take(len).collect()
}

#[allow(dead_code)]
pub fn number_to_string(n: i64) -> String {
    let base_codec = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
    let len = base_codec.len() as i64;
    let mut t = n;
    let mut result = "".to_string();
    while t > 0 {
        let idx = (t % len) as usize;
        let ch = base_codec[idx];
        t /= len;
        result.insert(0, ch);
    }
    result
}

#[allow(dead_code)]
pub fn generate_rand_numberstring(len: usize) -> String {
    let mut retkey = "".to_string();

    while retkey.len() < len {
        let rng = rand::random::<u16>();
        let key = number_to_string(rng as i64);
        retkey += key.as_str();
    }

    retkey.chars().take(len).collect()
}
