use chimes_utils::{i32_from_str, i64_from_str};
use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use rbson::Bson;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_dict_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_api_server"|table_columns:"id,name,code,server_address,app_id,app_secret,status,protocol,auth_url,auth_type,auth_form,token_path,token_name,token_place,session_timeout,remark,create_time,update_time")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesApiServerInfo {
    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub id: Option<i64>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub server_address: Option<String>,
    pub app_id: Option<String>,
    pub app_secret: Option<String>,
    pub status: Option<String>,
    pub protocol: Option<String>,
    pub auth_url: Option<String>,
    pub auth_type: Option<String>,
    pub auth_form: Option<String>,
    pub token_path: Option<String>,
    pub token_name: Option<String>,
    pub token_place: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "i32_from_str")]
    pub session_timeout: Option<i32>,
    pub remark: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
}

impl ChimesApiServerInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, dict_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("id", dict_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        match rb.save(self, &[Skip::Column("id")]).await {
            Ok(ds) => {
                self.id = ds.last_insert_id;
                Ok(ds.rows_affected)
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("id", self.id);
        rb.update_by_wrapper(self, wp, &[Skip::Column("id")]).await
    }

    #[allow(dead_code)]
    pub async fn update_selective(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("id", self.id);
        rb.update_by_wrapper(self, wp, &[Skip::Value(Bson::Null)])
            .await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.id.clone().is_some(), |w| {
                w.and().eq("id", self.id.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.code.clone().is_some(), |w| {
                w.and().eq("code", self.code.clone().unwrap())
            })
            .r#if(self.server_address.clone().is_some(), |w| {
                w.and()
                    .eq("server_address", self.server_address.clone().unwrap())
            })
            .r#if(self.app_id.clone().is_some(), |w| {
                w.and().eq("app_id", self.app_id.clone().unwrap())
            })
            .r#if(self.status.clone().is_some(), |w| {
                w.and().eq("status", self.status.clone().unwrap())
            })
            .r#if(self.protocol.clone().is_some(), |w| {
                w.and().eq("protocol", self.protocol.clone().unwrap())
            })
            .r#if(self.auth_type.clone().is_some(), |w| {
                w.and().eq("auth_type", self.auth_type.clone().unwrap())
            })
            .r#if(self.token_place.clone().is_some(), |w| {
                w.and().eq("token_place", self.token_place.clone().unwrap())
            })
            .r#if(self.token_name.clone().is_some(), |w| {
                w.and().eq("token_name", self.token_name.clone().unwrap())
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
        let wp = rb.new_wrapper().eq("id", self.id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.id.clone().is_some(), |w| {
                w.and().eq("id", self.id.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.code.clone().is_some(), |w| {
                w.and().eq("code", self.code.clone().unwrap())
            })
            .r#if(self.server_address.clone().is_some(), |w| {
                w.and()
                    .eq("server_address", self.server_address.clone().unwrap())
            })
            .r#if(self.app_id.clone().is_some(), |w| {
                w.and().eq("app_id", self.app_id.clone().unwrap())
            })
            .r#if(self.status.clone().is_some(), |w| {
                w.and().eq("status", self.status.clone().unwrap())
            })
            .r#if(self.protocol.clone().is_some(), |w| {
                w.and().eq("protocol", self.protocol.clone().unwrap())
            })
            .r#if(self.auth_type.clone().is_some(), |w| {
                w.and().eq("auth_type", self.auth_type.clone().unwrap())
            })
            .r#if(self.token_place.clone().is_some(), |w| {
                w.and().eq("token_place", self.token_place.clone().unwrap())
            })
            .r#if(self.token_name.clone().is_some(), |w| {
                w.and().eq("token_name", self.token_name.clone().unwrap())
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
            .r#if(self.id.clone().is_some(), |w| {
                w.and().eq("id", self.id.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.code.clone().is_some(), |w| {
                w.and().eq("code", self.code.clone().unwrap())
            })
            .r#if(self.server_address.clone().is_some(), |w| {
                w.and()
                    .eq("server_address", self.server_address.clone().unwrap())
            })
            .r#if(self.app_id.clone().is_some(), |w| {
                w.and().eq("app_id", self.app_id.clone().unwrap())
            })
            .r#if(self.status.clone().is_some(), |w| {
                w.and().eq("status", self.status.clone().unwrap())
            })
            .r#if(self.protocol.clone().is_some(), |w| {
                w.and().eq("protocol", self.protocol.clone().unwrap())
            })
            .r#if(self.auth_type.clone().is_some(), |w| {
                w.and().eq("auth_type", self.auth_type.clone().unwrap())
            })
            .r#if(self.token_place.clone().is_some(), |w| {
                w.and().eq("token_place", self.token_place.clone().unwrap())
            })
            .r#if(self.token_name.clone().is_some(), |w| {
                w.and().eq("token_name", self.token_name.clone().unwrap())
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
    pub async fn remove_ids(rb: &Rbatis, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#in("id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }
}
