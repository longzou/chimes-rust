use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use rbson::Bson;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chime_profile_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_profile"|table_columns:"user_id,full_name,address,education,birth_day,create_date")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimeProfileInfo {
    pub user_id: Option<i64>,
    pub full_name: Option<String>,
    pub address: Option<String>,
    pub education: Option<String>,
    pub birth_day: Option<rbatis::DateNative>,
    pub create_date: Option<rbatis::DateTimeNative>,
}

impl ChimeProfileInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, user_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("user_id", user_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        match rb.save(self, &[Skip::Column("user_id")]).await {
            Ok(ds) => Ok(ds.rows_affected),
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
            .r#if(self.full_name.clone().is_some(), |w| {
                w.and().eq("full_name", self.full_name.clone().unwrap())
            })
            .r#if(self.address.clone().is_some(), |w| {
                w.and().eq("address", self.address.clone().unwrap())
            })
            .r#if(self.education.clone().is_some(), |w| {
                w.and().eq("education", self.education.clone().unwrap())
            })
            .r#if(self.birth_day.clone().is_some(), |w| {
                w.and().eq("birth_day", self.birth_day.unwrap())
            })
            .r#if(self.create_date.clone().is_some(), |w| {
                w.and().eq("create_date", self.create_date.unwrap())
            });
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("user_id", self.user_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.user_id.clone().is_some(), |w| {
                w.and().eq("user_id", self.user_id.unwrap())
            })
            .r#if(self.full_name.clone().is_some(), |w| {
                w.and().eq("full_name", self.full_name.clone().unwrap())
            })
            .r#if(self.address.clone().is_some(), |w| {
                w.and().eq("address", self.address.clone().unwrap())
            })
            .r#if(self.education.clone().is_some(), |w| {
                w.and().eq("education", self.education.clone().unwrap())
            })
            .r#if(self.birth_day.clone().is_some(), |w| {
                w.and().eq("birth_day", self.birth_day.unwrap())
            })
            .r#if(self.create_date.clone().is_some(), |w| {
                w.and().eq("create_date", self.create_date.unwrap())
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
            .r#if(self.full_name.clone().is_some(), |w| {
                w.and().eq("full_name", self.full_name.clone().unwrap())
            })
            .r#if(self.address.clone().is_some(), |w| {
                w.and().eq("address", self.address.clone().unwrap())
            })
            .r#if(self.education.clone().is_some(), |w| {
                w.and().eq("education", self.education.clone().unwrap())
            })
            .r#if(self.birth_day.clone().is_some(), |w| {
                w.and().eq("birth_day", self.birth_day.unwrap())
            })
            .r#if(self.create_date.clone().is_some(), |w| {
                w.and().eq("create_date", self.create_date.unwrap())
            });
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }
}
