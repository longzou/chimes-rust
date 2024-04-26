use crate::entity::ChimesCompanyAccountInfo;
use chimes_utils::{bool_from_str, i64_from_str};
use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use rbson::Bson;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_user_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_user"|table_columns:"user_id,dept_id,username,nick_name,gender,phone,email,area_id,avatar_name,avatar_path,password,is_admin,open_id,union_id,enabled,create_by,update_by,data_scope,pwd_reset_time,create_time,update_time,company_id,company_code")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesUserInfo {
    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub user_id: Option<i64>,
    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub dept_id: Option<i64>,
    pub username: Option<String>,
    pub nick_name: Option<String>,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub area_id: Option<String>,
    pub avatar_name: Option<String>,
    pub avatar_path: Option<String>,
    pub password: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub is_admin: Option<bool>,
    pub open_id: Option<String>,
    pub union_id: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub enabled: Option<bool>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub data_scope: Option<String>,
    pub pwd_reset_time: Option<rbatis::DateTimeNative>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,

    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub company_id: Option<i64>,
    pub company_code: Option<String>,
    pub simulate_roles: Option<String>,
}

impl ChimesUserInfo {
    pub fn from_account(param: &ChimesCompanyAccountInfo) -> Self {
        Self {
            user_id: param.account_id,
            dept_id: None,
            username: param.username.clone(),
            nick_name: param.real_name.clone(),
            gender: param.gender.clone(),
            phone: param.telephone.clone(),
            email: param.email.clone(),
            area_id: None,
            company_id: param.company_id,
            company_code: param.company_code.clone(),
            simulate_roles: param.role_id.clone(),
            avatar_name: None, // param.avatar_name.clone(),
            avatar_path: None, // param.avatar_path.clone(),
            data_scope: None,
            password: param.sign_password.clone(),
            open_id: param.wechat_open_id.clone(),
            union_id: param.wechat_union_id.clone(),
            is_admin: Some(false),
            enabled: param.enabled,
            create_by: None,
            update_by: None,
            pwd_reset_time: None,
            create_time: param.create_time,
            update_time: param.update_time,
        }
    }

    pub fn to_account(&self) -> ChimesCompanyAccountInfo {
        ChimesCompanyAccountInfo {
            account_id: self.user_id,
            username: self.username.clone(),
            real_name: self.nick_name.clone(),
            gender: self.gender.clone(),
            telephone: self.phone.clone(),
            email: self.email.clone(),
            company_id: self.company_id,
            company_code: self.company_code.clone(),
            role_id: None,
            role_desc: None,
            remark: None,
            position: None,
            department: None,
            sign_password: self.password.clone(),
            wechat_open_id: self.open_id.clone(),
            wechat_union_id: self.union_id.clone(),
            enabled: self.enabled,
            create_time: self.create_time,
            update_time: self.update_time,
        }
    }

    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, user_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("user_id", user_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        match rb.save(self, &[Skip::Column("user_id")]).await {
            Ok(ds) => {
                self.user_id = ds.last_insert_id;
                Ok(ds.rows_affected)
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("user_id", self.user_id);
        rb.update_by_wrapper(self, wp, &[Skip::Column("user_id")])
            .await
    }

    #[allow(dead_code)]
    pub async fn update_selective(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("user_id", self.user_id);
        rb.update_by_wrapper(self, wp, &[Skip::Value(Bson::Null)])
            .await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.user_id.clone().is_some(), |w| {
                w.and().eq("user_id", self.user_id.unwrap())
            })
            .r#if(self.dept_id.clone().is_some(), |w| {
                w.and().eq("dept_id", self.dept_id.unwrap())
            })
            .r#if(self.username.clone().is_some(), |w| {
                w.and().eq("username", self.username.clone().unwrap())
            })
            .r#if(self.nick_name.clone().is_some(), |w| {
                w.and().eq("nick_name", self.nick_name.clone().unwrap())
            })
            .r#if(self.gender.clone().is_some(), |w| {
                w.and().eq("gender", self.gender.clone().unwrap())
            })
            .r#if(self.phone.clone().is_some(), |w| {
                w.and().eq("phone", self.phone.clone().unwrap())
            })
            .r#if(self.email.clone().is_some(), |w| {
                w.and().eq("email", self.email.clone().unwrap())
            })
            .r#if(self.avatar_name.clone().is_some(), |w| {
                w.and().eq("avatar_name", self.avatar_name.clone().unwrap())
            })
            .r#if(self.avatar_path.clone().is_some(), |w| {
                w.and().eq("avatar_path", self.avatar_path.clone().unwrap())
            })
            .r#if(self.password.clone().is_some(), |w| {
                w.and().eq("password", self.password.clone().unwrap())
            })
            .r#if(self.is_admin.clone().is_some(), |w| {
                w.and().eq("is_admin", self.is_admin.unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            })
            .r#if(self.create_by.clone().is_some(), |w| {
                w.and().eq("create_by", self.create_by.clone().unwrap())
            })
            .r#if(self.update_by.clone().is_some(), |w| {
                w.and().eq("update_by", self.update_by.clone().unwrap())
            })
            .r#if(self.pwd_reset_time.clone().is_some(), |w| {
                w.and().eq("pwd_reset_time", self.pwd_reset_time.unwrap())
            })
            .r#if(self.data_scope.clone().is_some(), |w| {
                w.and().eq("data_scope", self.data_scope.clone().unwrap())
            })
            .r#if(self.company_id.clone().is_some(), |w| {
                w.and().eq("company_id", self.company_id.unwrap())
            })
            .r#if(self.company_code.clone().is_some(), |w| {
                w.and()
                    .eq("company_code", self.company_code.clone().unwrap())
            })
            .r#if(self.create_time.clone().is_some(), |w| {
                w.and().eq("create_time", self.create_time.unwrap())
            })
            .r#if(self.update_time.clone().is_some(), |w| {
                w.and().eq("update_time", self.update_time.unwrap())
            });
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("user_id", self.user_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_ids(rb: &Rbatis, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#in("user_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.user_id.clone().is_some(), |w| {
                w.and().eq("user_id", self.user_id.unwrap())
            })
            .r#if(self.dept_id.clone().is_some(), |w| {
                w.and().eq("dept_id", self.dept_id.unwrap())
            })
            .r#if(self.username.clone().is_some(), |w| {
                w.and().eq("username", self.username.clone().unwrap())
            })
            .r#if(self.nick_name.clone().is_some(), |w| {
                w.and().eq("nick_name", self.nick_name.clone().unwrap())
            })
            .r#if(self.gender.clone().is_some(), |w| {
                w.and().eq("gender", self.gender.clone().unwrap())
            })
            .r#if(self.phone.clone().is_some(), |w| {
                w.and().eq("phone", self.phone.clone().unwrap())
            })
            .r#if(self.email.clone().is_some(), |w| {
                w.and().eq("email", self.email.clone().unwrap())
            })
            .r#if(self.avatar_name.clone().is_some(), |w| {
                w.and().eq("avatar_name", self.avatar_name.clone().unwrap())
            })
            .r#if(self.avatar_path.clone().is_some(), |w| {
                w.and().eq("avatar_path", self.avatar_path.clone().unwrap())
            })
            .r#if(self.password.clone().is_some(), |w| {
                w.and().eq("password", self.password.clone().unwrap())
            })
            .r#if(self.is_admin.clone().is_some(), |w| {
                w.and().eq("is_admin", self.is_admin.unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            })
            .r#if(self.create_by.clone().is_some(), |w| {
                w.and().eq("create_by", self.create_by.clone().unwrap())
            })
            .r#if(self.update_by.clone().is_some(), |w| {
                w.and().eq("update_by", self.update_by.clone().unwrap())
            })
            .r#if(self.data_scope.clone().is_some(), |w| {
                w.and().eq("data_scope", self.data_scope.clone().unwrap())
            })
            .r#if(self.company_id.clone().is_some(), |w| {
                w.and().eq("company_id", self.company_id.unwrap())
            })
            .r#if(self.company_code.clone().is_some(), |w| {
                w.and()
                    .eq("company_code", self.company_code.clone().unwrap())
            })
            .r#if(self.pwd_reset_time.clone().is_some(), |w| {
                w.and().eq("pwd_reset_time", self.pwd_reset_time.unwrap())
            })
            .r#if(self.create_time.clone().is_some(), |w| {
                w.and().eq("create_time", self.create_time.unwrap())
            })
            .r#if(self.update_time.clone().is_some(), |w| {
                w.and().eq("update_time", self.update_time.unwrap())
            });
        rb.fetch_page_by_wrapper::<Self>(wp, &PageRequest::new(curr, ps))
            .await
    }

    #[allow(dead_code)]
    pub async fn query_list(&self, rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.user_id.clone().is_some(), |w| {
                w.and().eq("user_id", self.user_id.unwrap())
            })
            .r#if(self.dept_id.clone().is_some(), |w| {
                w.and().eq("dept_id", self.dept_id.unwrap())
            })
            .r#if(self.username.clone().is_some(), |w| {
                w.and().eq("username", self.username.clone().unwrap())
            })
            .r#if(self.nick_name.clone().is_some(), |w| {
                w.and().eq("nick_name", self.nick_name.clone().unwrap())
            })
            .r#if(self.gender.clone().is_some(), |w| {
                w.and().eq("gender", self.gender.clone().unwrap())
            })
            .r#if(self.phone.clone().is_some(), |w| {
                w.and().eq("phone", self.phone.clone().unwrap())
            })
            .r#if(self.email.clone().is_some(), |w| {
                w.and().eq("email", self.email.clone().unwrap())
            })
            .r#if(self.avatar_name.clone().is_some(), |w| {
                w.and().eq("avatar_name", self.avatar_name.clone().unwrap())
            })
            .r#if(self.avatar_path.clone().is_some(), |w| {
                w.and().eq("avatar_path", self.avatar_path.clone().unwrap())
            })
            .r#if(self.password.clone().is_some(), |w| {
                w.and().eq("password", self.password.clone().unwrap())
            })
            .r#if(self.is_admin.clone().is_some(), |w| {
                w.and().eq("is_admin", self.is_admin.unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            })
            .r#if(self.create_by.clone().is_some(), |w| {
                w.and().eq("create_by", self.create_by.clone().unwrap())
            })
            .r#if(self.update_by.clone().is_some(), |w| {
                w.and().eq("update_by", self.update_by.clone().unwrap())
            })
            .r#if(self.data_scope.clone().is_some(), |w| {
                w.and().eq("data_scope", self.data_scope.clone().unwrap())
            })
            .r#if(self.company_id.clone().is_some(), |w| {
                w.and().eq("company_id", self.company_id.unwrap())
            })
            .r#if(self.company_code.clone().is_some(), |w| {
                w.and()
                    .eq("company_code", self.company_code.clone().unwrap())
            })
            .r#if(self.pwd_reset_time.clone().is_some(), |w| {
                w.and().eq("pwd_reset_time", self.pwd_reset_time.unwrap())
            })
            .r#if(self.create_time.clone().is_some(), |w| {
                w.and().eq("create_time", self.create_time.unwrap())
            })
            .r#if(self.update_time.clone().is_some(), |w| {
                w.and().eq("update_time", self.update_time.unwrap())
            });
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_all(rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb.new_wrapper();
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn load_username(rb: &Rbatis, username: &str) -> Result<Option<Self>, Error> {
        let ucs = username.split("$$").collect::<Vec<&str>>();
        if ucs.len() < 2 {
            let wp = rb.new_wrapper().and().eq("username", username.to_owned());
            rb.fetch_by_wrapper::<Option<Self>>(wp).await
        } else {
            let compcode = ucs[0].to_string();
            let unms = ucs[1].to_string();
            match ChimesCompanyAccountInfo::find_code(rb, &compcode.clone(), &unms.clone()).await {
                Ok(ts) => {
                    match ts {
                        Some(mt) => {
                            let cds = Self::from_account(&mt);
                            // let mut codes = vec![];

                            // codes.push("ROLE_COMMONUSER".to_string());
                            // if mt.role_id.is_some() {
                            //     // Should we have multi role code in this field
                            //     codes.push(mt.role_id.unwrap_or_default());
                            // }

                            // match ChimesRoleInfo::query_multicode(rb, &codes).await {
                            //     Ok(mlst) => {
                            //         cds.roles = mlst;
                            //     }
                            //     Err(err) => {
                            //         log::warn!("Could not fetch the roles {}", err);
                            //     }
                            // }

                            Ok(Some(cds))
                        }
                        None => Ok(None),
                    }
                }
                Err(err) => Err(err),
            }
        }
    }

    #[allow(dead_code)]
    pub async fn load_openid(rb: &Rbatis, open_id: &str) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().and().eq("open_id", open_id.to_owned());
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn load_unionid(rb: &Rbatis, union_id: &str) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().and().eq("union_id", union_id.to_owned());
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    pub fn parse_simulate_roles(&self) -> Vec<String> {
        if self.simulate_roles.is_some() {
            let mt = self.simulate_roles.clone().unwrap_or_default();
            let mrs = mt
                .split(',')
                .map(|f| f.to_string())
                .collect::<Vec<String>>();
            mrs
        } else {
            vec![]
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserUpdatePasswordRequest {
    pub old_pwd: String,
    pub new_pwd: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserUpdateInfoRequest {
    pub nick_name: String,
    pub gender: String,
    pub phone: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserUpdateEmailRequest {
    pub password: String,
    pub email: String,
    pub code: String,
    pub codekey: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserResetEmailRequest {
    pub email: String,
    pub codekey: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserUpdateAvatarRequest {
    pub old_pwd: String,
    pub new_pwd: String,
}
