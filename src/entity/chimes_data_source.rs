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

#[crud_table(table_name:"chimes_data_source"|table_columns:"id,name,code,database_type,server_address,port,db_name,username,password,connection_timeout,execution_timeout,properties,remark,status,create_time,update_time")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesDataSourceInfo {
    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub id: Option<i64>,
    pub name: Option<String>,
    pub code: Option<String>,
    pub database_type: Option<String>,
    pub server_address: Option<String>,
    pub port: Option<String>,
    pub db_name: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "i32_from_str")]
    pub connection_timeout: Option<i32>,
    #[serde(default)]
    #[serde(deserialize_with = "i32_from_str")]
    pub execution_timeout: Option<i32>,
    pub properties: Option<String>,
    pub remark: Option<String>,
    pub status: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
}

impl ChimesDataSourceInfo {
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
            .r#if(self.database_type.clone().is_some(), |w| {
                w.and()
                    .eq("database_type", self.database_type.clone().unwrap())
            })
            .r#if(self.db_name.clone().is_some(), |w| {
                w.and().eq("db_name", self.db_name.clone().unwrap())
            })
            .r#if(self.status.clone().is_some(), |w| {
                w.and().eq("status", self.status.clone().unwrap())
            })
            .r#if(self.remark.clone().is_some(), |w| {
                w.and().eq("remark", self.remark.clone().unwrap())
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
            .r#if(self.database_type.clone().is_some(), |w| {
                w.and()
                    .eq("database_type", self.database_type.clone().unwrap())
            })
            .r#if(self.db_name.clone().is_some(), |w| {
                w.and().eq("db_name", self.db_name.clone().unwrap())
            })
            .r#if(self.status.clone().is_some(), |w| {
                w.and().eq("status", self.status.clone().unwrap())
            })
            .r#if(self.remark.clone().is_some(), |w| {
                w.and().eq("remark", self.remark.clone().unwrap())
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
            .r#if(self.database_type.clone().is_some(), |w| {
                w.and()
                    .eq("database_type", self.database_type.clone().unwrap())
            })
            .r#if(self.db_name.clone().is_some(), |w| {
                w.and().eq("db_name", self.db_name.clone().unwrap())
            })
            .r#if(self.status.clone().is_some(), |w| {
                w.and().eq("status", self.status.clone().unwrap())
            })
            .r#if(self.remark.clone().is_some(), |w| {
                w.and().eq("remark", self.remark.clone().unwrap())
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
