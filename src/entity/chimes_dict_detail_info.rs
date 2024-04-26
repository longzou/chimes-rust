use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use rbson::Bson;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_dict_detail_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_dict_detail"|table_columns:"detail_id,dict_id,label,value,dict_sort,create_by,update_by,create_time,update_time")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesDictDetailInfo {
    pub detail_id: Option<i64>,
    pub dict_id: Option<i64>,
    pub label: Option<String>,
    pub value: Option<String>,
    pub dict_sort: Option<i32>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
}

impl ChimesDictDetailInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, detail_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("detail_id", detail_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        match rb.save(self, &[Skip::Column("detail_id")]).await {
            Ok(ds) => {
                self.detail_id = ds.last_insert_id;
                Ok(ds.rows_affected)
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("detail_id", self.detail_id);
        rb.update_by_wrapper(self, wp, &[Skip::Column("detail_id")])
            .await
    }

    #[allow(dead_code)]
    pub async fn update_selective(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("detail_id", self.detail_id);
        rb.update_by_wrapper(self, wp, &[Skip::Value(Bson::Null)])
            .await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.detail_id.clone().is_some(), |w| {
                w.and().eq("detail_id", self.detail_id.unwrap())
            })
            .r#if(self.dict_id.clone().is_some(), |w| {
                w.and().eq("dict_id", self.dict_id.unwrap())
            })
            .r#if(self.label.clone().is_some(), |w| {
                w.and().eq("label", self.label.clone().unwrap())
            })
            .r#if(self.value.clone().is_some(), |w| {
                w.and().eq("value", self.value.clone().unwrap())
            })
            .r#if(self.dict_sort.clone().is_some(), |w| {
                w.and().eq("dict_sort", self.dict_sort.unwrap())
            })
            .r#if(self.create_by.clone().is_some(), |w| {
                w.and().eq("create_by", self.create_by.clone().unwrap())
            })
            .r#if(self.update_by.clone().is_some(), |w| {
                w.and().eq("update_by", self.update_by.clone().unwrap())
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
        let wp = rb.new_wrapper().eq("detail_id", self.detail_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.detail_id.clone().is_some(), |w| {
                w.and().eq("detail_id", self.detail_id.unwrap())
            })
            .r#if(self.dict_id.clone().is_some(), |w| {
                w.and().eq("dict_id", self.dict_id.unwrap())
            })
            .r#if(self.label.clone().is_some(), |w| {
                w.and().eq("label", self.label.clone().unwrap())
            })
            .r#if(self.value.clone().is_some(), |w| {
                w.and().eq("value", self.value.clone().unwrap())
            })
            .r#if(self.dict_sort.clone().is_some(), |w| {
                w.and().eq("dict_sort", self.dict_sort.unwrap())
            })
            .r#if(self.create_by.clone().is_some(), |w| {
                w.and().eq("create_by", self.create_by.clone().unwrap())
            })
            .r#if(self.update_by.clone().is_some(), |w| {
                w.and().eq("update_by", self.update_by.clone().unwrap())
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
            .r#if(self.detail_id.clone().is_some(), |w| {
                w.and().eq("detail_id", self.detail_id.unwrap())
            })
            .r#if(self.dict_id.clone().is_some(), |w| {
                w.and().eq("dict_id", self.dict_id.unwrap())
            })
            .r#if(self.label.clone().is_some(), |w| {
                w.and().eq("label", self.label.clone().unwrap())
            })
            .r#if(self.value.clone().is_some(), |w| {
                w.and().eq("value", self.value.clone().unwrap())
            })
            .r#if(self.dict_sort.clone().is_some(), |w| {
                w.and().eq("dict_sort", self.dict_sort.unwrap())
            })
            .r#if(self.create_by.clone().is_some(), |w| {
                w.and().eq("create_by", self.create_by.clone().unwrap())
            })
            .r#if(self.update_by.clone().is_some(), |w| {
                w.and().eq("update_by", self.update_by.clone().unwrap())
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
        let wp = rb.new_wrapper().r#in("detail_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }
}
