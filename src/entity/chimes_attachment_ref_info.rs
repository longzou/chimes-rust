use chimes_utils::i64_from_str;
use rbatis::crud::{CRUDMut, Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::executor::{RBatisTxExecutor, RbatisRef};
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use rbson::Bson;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_attachment_ref_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_attachment_rel"|table_columns:"rel_id,attachment_id,business_name,business_id,modify_by,create_time,update_time")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesAttachmentRefInfo {
    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub rel_id: Option<i64>,
    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub attachment_id: Option<i64>,
    pub business_name: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub business_id: Option<i64>,
    pub modify_by: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
}

impl ChimesAttachmentRefInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, rel_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("rel_id", rel_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &mut RBatisTxExecutor<'_>) -> Result<u64, Error> {
        match rb.save(self, &[Skip::Column("rel_id")]).await {
            Ok(ds) => {
                self.rel_id = ds.last_insert_id;
                Ok(ds.rows_affected)
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &mut RBatisTxExecutor<'_>) -> Result<u64, Error> {
        let wp = rb.get_rbatis().new_wrapper().eq("rel_id", self.rel_id);
        rb.update_by_wrapper(self, wp, &[Skip::Column("rel_id")])
            .await
    }

    #[allow(dead_code)]
    pub async fn update_selective(&self, rb: &mut RBatisTxExecutor<'_>) -> Result<u64, Error> {
        let wp = rb.get_rbatis().new_wrapper().eq("rel_id", self.rel_id);
        rb.update_by_wrapper(self, wp, &[Skip::Value(Bson::Null)])
            .await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &mut RBatisTxExecutor<'_>) -> Result<u64, Error> {
        let wp = rb
            .get_rbatis()
            .new_wrapper()
            .r#if(self.rel_id.clone().is_some(), |w| {
                w.and().eq("rel_id", self.rel_id.unwrap())
            })
            .r#if(self.attachment_id.clone().is_some(), |w| {
                w.and().eq("attachment_id", self.attachment_id.unwrap())
            })
            .r#if(self.business_name.clone().is_some(), |w| {
                w.and()
                    .eq("business_name", self.business_name.clone().unwrap())
            })
            .r#if(self.business_id.clone().is_some(), |w| {
                w.and().eq("business_id", self.business_id.unwrap())
            })
            .r#if(self.modify_by.clone().is_some(), |w| {
                w.and().eq("modify_by", self.modify_by.clone().unwrap())
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
    pub async fn remove(&mut self, rb: &mut RBatisTxExecutor<'_>) -> Result<u64, Error> {
        let wp = rb.get_rbatis().new_wrapper().eq("rel_id", self.rel_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_ids(rb: &mut RBatisTxExecutor<'_>, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.get_rbatis().new_wrapper().r#in("rel_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_attachment_ids(
        rb: &mut RBatisTxExecutor<'_>,
        ids: &[i64],
    ) -> Result<u64, Error> {
        let wp = rb.get_rbatis().new_wrapper().r#in("attachment_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn load_ids(rb: &Rbatis, ids: &[i64]) -> Result<Vec<Self>, Error> {
        let wp = rb.new_wrapper().r#in("rel_id", ids);
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.rel_id.clone().is_some(), |w| {
                w.and().eq("rel_id", self.rel_id.unwrap())
            })
            .r#if(self.attachment_id.clone().is_some(), |w| {
                w.and().eq("attachment_id", self.attachment_id.unwrap())
            })
            .r#if(self.business_name.clone().is_some(), |w| {
                w.and()
                    .eq("business_name", self.business_name.clone().unwrap())
            })
            .r#if(self.business_id.clone().is_some(), |w| {
                w.and().eq("business_id", self.business_id.unwrap())
            })
            .r#if(self.modify_by.clone().is_some(), |w| {
                w.and().eq("modify_by", self.modify_by.clone().unwrap())
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
            .r#if(self.rel_id.clone().is_some(), |w| {
                w.and().eq("rel_id", self.rel_id.unwrap())
            })
            .r#if(self.attachment_id.clone().is_some(), |w| {
                w.and().eq("attachment_id", self.attachment_id.unwrap())
            })
            .r#if(self.business_name.clone().is_some(), |w| {
                w.and()
                    .eq("business_name", self.business_name.clone().unwrap())
            })
            .r#if(self.business_id.clone().is_some(), |w| {
                w.and().eq("business_id", self.business_id.unwrap())
            })
            .r#if(self.modify_by.clone().is_some(), |w| {
                w.and().eq("modify_by", self.modify_by.clone().unwrap())
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
}
