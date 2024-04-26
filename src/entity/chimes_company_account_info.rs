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
 * Generate the file for chimes_company_account_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_account"|table_columns:"account_id,company_id,company_code,username,sign_password,wechat_open_id,role_desc,position,department,remark,create_time,update_time,real_name,telephone,wechat_union_id,role_id,gender,enabled,email")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesCompanyAccountInfo {
    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub account_id: Option<i64>,
    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub company_id: Option<i64>,
    pub company_code: Option<String>,
    pub username: Option<String>,
    pub sign_password: Option<String>,
    pub wechat_open_id: Option<String>,
    pub role_desc: Option<String>,
    pub position: Option<String>,
    pub department: Option<String>,
    pub remark: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
    pub real_name: Option<String>,
    pub telephone: Option<String>,
    pub wechat_union_id: Option<String>,
    pub role_id: Option<String>,
    pub gender: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub enabled: Option<bool>,
    pub email: Option<String>,
}

impl ChimesCompanyAccountInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, account_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("account_id", account_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn find_code(
        rb: &Rbatis,
        company_code: &String,
        username: &String,
    ) -> Result<Option<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .eq("company_code", company_code)
            .eq("username", username);

        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn load_openid(rb: &Rbatis, open_id: &str) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("wechat_open_id", open_id);

        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        match rb.save(self, &[Skip::Column("account_id")]).await {
            Ok(ds) => {
                self.account_id = ds.last_insert_id;
                Ok(ds.rows_affected)
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update_openid(&self, rb: &Rbatis) -> Result<u64, Error> {
        let _wp = rb.new_wrapper().eq("account_id", self.account_id);
        let sql = if self.wechat_open_id.is_none() {
            "update chimes_account set wechat_open_id = null, wechat_union_id = null where account_id = ?"
        } else if self.wechat_union_id.is_some() {
            "update chimes_account set wechat_open_id = ?, wechat_union_id = ? where account_id = ?"
        } else {
            "update chimes_account set wechat_open_id = ? where account_id = ?"
        };
        let mut rb_args = vec![];
        if self.wechat_open_id.is_some() {
            rb_args.push(
                rbson::to_bson(self.wechat_open_id.clone().unwrap_or_default()).unwrap_or_default(),
            );
            if self.wechat_union_id.is_some() {
                rb_args.push(
                    rbson::to_bson(self.wechat_union_id.clone().unwrap_or_default())
                        .unwrap_or_default(),
                );
            }
        }
        rb_args.push(rbson::to_bson(self.account_id.unwrap_or_default()).unwrap_or_default());

        match rb.exec(sql, rb_args).await {
            Ok(t) => Ok(t.rows_affected),
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("account_id", self.account_id);
        rb.update_by_wrapper(self, wp, &[Skip::Column("account_id")])
            .await
    }

    #[allow(dead_code)]
    pub async fn update_selective(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("account_id", self.account_id);
        rb.update_by_wrapper(self, wp, &[Skip::Value(Bson::Null)])
            .await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.account_id.clone().is_some(), |w| {
                w.and().eq("account_id", self.account_id.unwrap())
            })
            .r#if(self.company_id.clone().is_some(), |w| {
                w.and().eq("company_id", self.company_id.unwrap())
            })
            .r#if(self.company_code.clone().is_some(), |w| {
                w.and()
                    .eq("company_code", self.company_code.clone().unwrap())
            })
            .r#if(self.username.clone().is_some(), |w| {
                w.and().eq("username", self.username.clone().unwrap())
            })
            .r#if(self.sign_password.clone().is_some(), |w| {
                w.and()
                    .eq("sign_password", self.sign_password.clone().unwrap())
            })
            .r#if(self.wechat_open_id.clone().is_some(), |w| {
                w.and()
                    .eq("wechat_open_id", self.wechat_open_id.clone().unwrap())
            })
            .r#if(self.role_desc.clone().is_some(), |w| {
                w.and().eq("role_desc", self.role_desc.clone().unwrap())
            })
            .r#if(self.position.clone().is_some(), |w| {
                w.and().eq("position", self.position.clone().unwrap())
            })
            .r#if(self.email.clone().is_some(), |w| {
                w.and().eq("email", self.email.clone().unwrap())
            })
            .r#if(self.department.clone().is_some(), |w| {
                w.and().eq("department", self.department.clone().unwrap())
            })
            .r#if(self.remark.clone().is_some(), |w| {
                w.and().eq("remark", self.remark.clone().unwrap())
            })
            .r#if(self.create_time.clone().is_some(), |w| {
                w.and().eq("create_time", self.create_time.unwrap())
            })
            .r#if(self.update_time.clone().is_some(), |w| {
                w.and().eq("update_time", self.update_time.unwrap())
            })
            .r#if(self.real_name.clone().is_some(), |w| {
                w.and().eq("real_name", self.real_name.clone().unwrap())
            })
            .r#if(self.telephone.clone().is_some(), |w| {
                w.and().eq("telephone", self.telephone.clone().unwrap())
            })
            .r#if(self.wechat_union_id.clone().is_some(), |w| {
                w.and()
                    .eq("wechat_union_id", self.wechat_union_id.clone().unwrap())
            })
            .r#if(self.role_id.clone().is_some(), |w| {
                w.and().eq("role_id", self.role_id.clone().unwrap())
            })
            .r#if(self.gender.clone().is_some(), |w| {
                w.and().eq("gender", self.gender.clone().unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            });
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("account_id", self.account_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_ids(rb: &Rbatis, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#in("account_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.account_id.clone().is_some(), |w| {
                w.and().eq("account_id", self.account_id.unwrap())
            })
            .r#if(self.company_id.clone().is_some(), |w| {
                w.and().eq("company_id", self.company_id.unwrap())
            })
            .r#if(self.company_code.clone().is_some(), |w| {
                w.and()
                    .eq("company_code", self.company_code.clone().unwrap())
            })
            .r#if(self.username.clone().is_some(), |w| {
                w.and().eq("username", self.username.clone().unwrap())
            })
            .r#if(self.sign_password.clone().is_some(), |w| {
                w.and()
                    .eq("sign_password", self.sign_password.clone().unwrap())
            })
            .r#if(self.wechat_open_id.clone().is_some(), |w| {
                w.and()
                    .eq("wechat_open_id", self.wechat_open_id.clone().unwrap())
            })
            .r#if(self.role_desc.clone().is_some(), |w| {
                w.and().eq("role_desc", self.role_desc.clone().unwrap())
            })
            .r#if(self.position.clone().is_some(), |w| {
                w.and().eq("position", self.position.clone().unwrap())
            })
            .r#if(self.email.clone().is_some(), |w| {
                w.and().eq("email", self.email.clone().unwrap())
            })
            .r#if(self.department.clone().is_some(), |w| {
                w.and().eq("department", self.department.clone().unwrap())
            })
            .r#if(self.remark.clone().is_some(), |w| {
                w.and().eq("remark", self.remark.clone().unwrap())
            })
            .r#if(self.create_time.clone().is_some(), |w| {
                w.and().eq("create_time", self.create_time.unwrap())
            })
            .r#if(self.update_time.clone().is_some(), |w| {
                w.and().eq("update_time", self.update_time.unwrap())
            })
            .r#if(self.real_name.clone().is_some(), |w| {
                w.and().eq("real_name", self.real_name.clone().unwrap())
            })
            .r#if(self.telephone.clone().is_some(), |w| {
                w.and().eq("telephone", self.telephone.clone().unwrap())
            })
            .r#if(self.wechat_union_id.clone().is_some(), |w| {
                w.and()
                    .eq("wechat_union_id", self.wechat_union_id.clone().unwrap())
            })
            .r#if(self.role_id.clone().is_some(), |w| {
                w.and().eq("role_id", self.role_id.clone().unwrap())
            })
            .r#if(self.gender.clone().is_some(), |w| {
                w.and().eq("gender", self.gender.clone().unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            });
        rb.fetch_page_by_wrapper::<Self>(wp, &PageRequest::new(curr, ps))
            .await
    }

    #[allow(dead_code)]
    pub async fn query_list(&self, rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.account_id.clone().is_some(), |w| {
                w.and().eq("account_id", self.account_id.unwrap())
            })
            .r#if(self.company_id.clone().is_some(), |w| {
                w.and().eq("company_id", self.company_id.unwrap())
            })
            .r#if(self.company_code.clone().is_some(), |w| {
                w.and()
                    .eq("company_code", self.company_code.clone().unwrap())
            })
            .r#if(self.username.clone().is_some(), |w| {
                w.and().eq("username", self.username.clone().unwrap())
            })
            .r#if(self.sign_password.clone().is_some(), |w| {
                w.and()
                    .eq("sign_password", self.sign_password.clone().unwrap())
            })
            .r#if(self.wechat_open_id.clone().is_some(), |w| {
                w.and()
                    .eq("wechat_open_id", self.wechat_open_id.clone().unwrap())
            })
            .r#if(self.role_desc.clone().is_some(), |w| {
                w.and().eq("role_desc", self.role_desc.clone().unwrap())
            })
            .r#if(self.position.clone().is_some(), |w| {
                w.and().eq("position", self.position.clone().unwrap())
            })
            .r#if(self.email.clone().is_some(), |w| {
                w.and().eq("email", self.email.clone().unwrap())
            })
            .r#if(self.department.clone().is_some(), |w| {
                w.and().eq("department", self.department.clone().unwrap())
            })
            .r#if(self.remark.clone().is_some(), |w| {
                w.and().eq("remark", self.remark.clone().unwrap())
            })
            .r#if(self.create_time.clone().is_some(), |w| {
                w.and().eq("create_time", self.create_time.unwrap())
            })
            .r#if(self.update_time.clone().is_some(), |w| {
                w.and().eq("update_time", self.update_time.unwrap())
            })
            .r#if(self.real_name.clone().is_some(), |w| {
                w.and().eq("real_name", self.real_name.clone().unwrap())
            })
            .r#if(self.telephone.clone().is_some(), |w| {
                w.and().eq("telephone", self.telephone.clone().unwrap())
            })
            .r#if(self.wechat_union_id.clone().is_some(), |w| {
                w.and()
                    .eq("wechat_union_id", self.wechat_union_id.clone().unwrap())
            })
            .r#if(self.role_id.clone().is_some(), |w| {
                w.and().eq("role_id", self.role_id.clone().unwrap())
            })
            .r#if(self.gender.clone().is_some(), |w| {
                w.and().eq("gender", self.gender.clone().unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            });
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_all(rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb.new_wrapper();
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }
}
