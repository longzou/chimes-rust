use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use rbson::Bson;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_log_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_log"|table_columns:"log_id,description,log_type,method,params,request_ip,time,username,address,browser,exception_detail,create_time")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesLogInfo {
    pub log_id: Option<i64>,
    pub description: Option<String>,
    pub log_type: Option<String>,
    pub method: Option<String>,
    pub params: Option<String>,
    pub request_ip: Option<String>,
    pub time: Option<i64>,
    pub username: Option<String>,
    pub address: Option<String>,
    pub browser: Option<String>,
    pub exception_detail: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
}

impl ChimesLogInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, log_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("log_id", log_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        match rb.save(self, &[Skip::Column("log_id")]).await {
            Ok(ds) => {
                self.log_id = ds.last_insert_id;
                Ok(ds.rows_affected)
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("log_id", self.log_id);
        rb.update_by_wrapper(self, wp, &[Skip::Column("log_id")])
            .await
    }

    #[allow(dead_code)]
    pub async fn update_selective(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("log_id", self.log_id);
        rb.update_by_wrapper(self, wp, &[Skip::Value(Bson::Null)])
            .await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.log_id.clone().is_some(), |w| {
                w.and().eq("log_id", self.log_id.unwrap())
            })
            .r#if(self.description.clone().is_some(), |w| {
                w.and().eq("description", self.description.clone().unwrap())
            })
            .r#if(self.log_type.clone().is_some(), |w| {
                w.and().eq("log_type", self.log_type.clone().unwrap())
            })
            .r#if(self.method.clone().is_some(), |w| {
                w.and().eq("method", self.method.clone().unwrap())
            })
            .r#if(self.params.clone().is_some(), |w| {
                w.and().eq("params", self.params.clone().unwrap())
            })
            .r#if(self.request_ip.clone().is_some(), |w| {
                w.and().eq("request_ip", self.request_ip.clone().unwrap())
            })
            .r#if(self.time.clone().is_some(), |w| {
                w.and().eq("time", self.time.unwrap())
            })
            .r#if(self.username.clone().is_some(), |w| {
                w.and().eq("username", self.username.clone().unwrap())
            })
            .r#if(self.address.clone().is_some(), |w| {
                w.and().eq("address", self.address.clone().unwrap())
            })
            .r#if(self.browser.clone().is_some(), |w| {
                w.and().eq("browser", self.browser.clone().unwrap())
            })
            .r#if(self.exception_detail.clone().is_some(), |w| {
                w.and()
                    .eq("exception_detail", self.exception_detail.clone().unwrap())
            })
            .r#if(self.create_time.clone().is_some(), |w| {
                w.and().eq("create_time", self.create_time.unwrap())
            });
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("log_id", self.log_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_ids(rb: &Rbatis, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#in("log_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.log_id.clone().is_some(), |w| {
                w.and().eq("log_id", self.log_id.unwrap())
            })
            .r#if(self.description.clone().is_some(), |w| {
                w.and().eq("description", self.description.clone().unwrap())
            })
            .r#if(self.log_type.clone().is_some(), |w| {
                w.and().eq("log_type", self.log_type.clone().unwrap())
            })
            .r#if(self.method.clone().is_some(), |w| {
                w.and().eq("method", self.method.clone().unwrap())
            })
            .r#if(self.params.clone().is_some(), |w| {
                w.and().eq("params", self.params.clone().unwrap())
            })
            .r#if(self.request_ip.clone().is_some(), |w| {
                w.and().eq("request_ip", self.request_ip.clone().unwrap())
            })
            .r#if(self.time.clone().is_some(), |w| {
                w.and().eq("time", self.time.unwrap())
            })
            .r#if(self.username.clone().is_some(), |w| {
                w.and().eq("username", self.username.clone().unwrap())
            })
            .r#if(self.address.clone().is_some(), |w| {
                w.and().eq("address", self.address.clone().unwrap())
            })
            .r#if(self.browser.clone().is_some(), |w| {
                w.and().eq("browser", self.browser.clone().unwrap())
            })
            .r#if(self.exception_detail.clone().is_some(), |w| {
                w.and()
                    .eq("exception_detail", self.exception_detail.clone().unwrap())
            })
            .r#if(self.create_time.clone().is_some(), |w| {
                w.and().eq("create_time", self.create_time.unwrap())
            });
        rb.fetch_page_by_wrapper::<Self>(wp, &PageRequest::new(curr, ps))
            .await
    }

    #[allow(dead_code)]
    pub async fn query_list(&self, rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.log_id.clone().is_some(), |w| {
                w.and().eq("log_id", self.log_id.unwrap())
            })
            .r#if(self.description.clone().is_some(), |w| {
                w.and().eq("description", self.description.clone().unwrap())
            })
            .r#if(self.log_type.clone().is_some(), |w| {
                w.and().eq("log_type", self.log_type.clone().unwrap())
            })
            .r#if(self.method.clone().is_some(), |w| {
                w.and().eq("method", self.method.clone().unwrap())
            })
            .r#if(self.params.clone().is_some(), |w| {
                w.and().eq("params", self.params.clone().unwrap())
            })
            .r#if(self.request_ip.clone().is_some(), |w| {
                w.and().eq("request_ip", self.request_ip.clone().unwrap())
            })
            .r#if(self.time.clone().is_some(), |w| {
                w.and().eq("time", self.time.unwrap())
            })
            .r#if(self.username.clone().is_some(), |w| {
                w.and().eq("username", self.username.clone().unwrap())
            })
            .r#if(self.address.clone().is_some(), |w| {
                w.and().eq("address", self.address.clone().unwrap())
            })
            .r#if(self.browser.clone().is_some(), |w| {
                w.and().eq("browser", self.browser.clone().unwrap())
            })
            .r#if(self.exception_detail.clone().is_some(), |w| {
                w.and()
                    .eq("exception_detail", self.exception_detail.clone().unwrap())
            })
            .r#if(self.create_time.clone().is_some(), |w| {
                w.and().eq("create_time", self.create_time.unwrap())
            });
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }
}
