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
 * Generate the file for chimes_attachment_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_attachments"|table_columns:"attachment_id,attach_hash,original_name,attachment_name,storage_path,access_url,file_ext,content_type,file_size,file_size_display,modify_by,create_time,update_time")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesAttachmentInfo {
    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub attachment_id: Option<i64>,
    pub attach_hash: Option<String>,
    pub original_name: Option<String>,
    pub attachment_name: Option<String>,
    pub storage_path: Option<String>,
    pub access_url: Option<String>,
    pub file_ext: Option<String>,
    pub content_type: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub file_size: Option<i64>,
    pub file_size_display: Option<String>,
    pub modify_by: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
}

impl ChimesAttachmentInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, attachment_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("attachment_id", attachment_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn from_attachment_name(
        rb: &Rbatis,
        attachment_name: &String,
    ) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("attachment_name", attachment_name);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &mut RBatisTxExecutor<'_>) -> Result<u64, Error> {
        match rb.save(self, &[Skip::Column("attachment_id")]).await {
            Ok(ds) => {
                self.attachment_id = ds.last_insert_id;
                Ok(ds.rows_affected)
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &mut RBatisTxExecutor<'_>) -> Result<u64, Error> {
        let wp = rb
            .get_rbatis()
            .new_wrapper()
            .eq("attachment_id", self.attachment_id);
        rb.update_by_wrapper(self, wp, &[Skip::Column("attachment_id")])
            .await
    }

    #[allow(dead_code)]
    pub async fn update_selective(&self, rb: &mut RBatisTxExecutor<'_>) -> Result<u64, Error> {
        let wp = rb
            .get_rbatis()
            .new_wrapper()
            .eq("attachment_id", self.attachment_id);
        rb.update_by_wrapper(self, wp, &[Skip::Value(Bson::Null)])
            .await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &mut RBatisTxExecutor<'_>) -> Result<u64, Error> {
        let wp = rb
            .get_rbatis()
            .new_wrapper()
            .r#if(self.attachment_id.clone().is_some(), |w| {
                w.and().eq("attachment_id", self.attachment_id.unwrap())
            })
            .r#if(self.attach_hash.clone().is_some(), |w| {
                w.and().eq("attach_hash", self.attach_hash.clone().unwrap())
            })
            .r#if(self.attachment_name.clone().is_some(), |w| {
                w.and()
                    .eq("attachment_name", self.attachment_name.clone().unwrap())
            })
            .r#if(self.storage_path.clone().is_some(), |w| {
                w.and()
                    .eq("storage_path", self.storage_path.clone().unwrap())
            })
            .r#if(self.access_url.clone().is_some(), |w| {
                w.and().eq("access_url", self.access_url.clone().unwrap())
            })
            .r#if(self.file_ext.clone().is_some(), |w| {
                w.and().eq("file_ext", self.file_ext.clone().unwrap())
            })
            .r#if(self.content_type.clone().is_some(), |w| {
                w.and()
                    .eq("content_type", self.content_type.clone().unwrap())
            })
            .r#if(self.file_size.clone().is_some(), |w| {
                w.and().eq("file_size", self.file_size.unwrap())
            })
            .r#if(self.file_size_display.clone().is_some(), |w| {
                w.and()
                    .eq("file_size_display", self.file_size_display.clone().unwrap())
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
        let wp = rb
            .get_rbatis()
            .new_wrapper()
            .eq("attachment_id", self.attachment_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_ids(rb: &mut RBatisTxExecutor<'_>, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.get_rbatis().new_wrapper().r#in("attachment_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn load_ids(rb: &Rbatis, ids: &[i64]) -> Result<Vec<Self>, Error> {
        let wp = rb.new_wrapper().r#in("attachment_id", ids);
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.attachment_id.clone().is_some(), |w| {
                w.and().eq("attachment_id", self.attachment_id.unwrap())
            })
            .r#if(self.attach_hash.clone().is_some(), |w| {
                w.and().eq("attach_hash", self.attach_hash.clone().unwrap())
            })
            .r#if(self.attachment_name.clone().is_some(), |w| {
                w.and()
                    .eq("attachment_name", self.attachment_name.clone().unwrap())
            })
            .r#if(self.storage_path.clone().is_some(), |w| {
                w.and()
                    .eq("storage_path", self.storage_path.clone().unwrap())
            })
            .r#if(self.access_url.clone().is_some(), |w| {
                w.and().eq("access_url", self.access_url.clone().unwrap())
            })
            .r#if(self.file_ext.clone().is_some(), |w| {
                w.and().eq("file_ext", self.file_ext.clone().unwrap())
            })
            .r#if(self.content_type.clone().is_some(), |w| {
                w.and()
                    .eq("content_type", self.content_type.clone().unwrap())
            })
            .r#if(self.file_size.clone().is_some(), |w| {
                w.and().eq("file_size", self.file_size.unwrap())
            })
            .r#if(self.file_size_display.clone().is_some(), |w| {
                w.and()
                    .eq("file_size_display", self.file_size_display.clone().unwrap())
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
            .r#if(self.attachment_id.clone().is_some(), |w| {
                w.and().eq("attachment_id", self.attachment_id.unwrap())
            })
            .r#if(self.attach_hash.clone().is_some(), |w| {
                w.and().eq("attach_hash", self.attach_hash.clone().unwrap())
            })
            .r#if(self.attachment_name.clone().is_some(), |w| {
                w.and()
                    .eq("attachment_name", self.attachment_name.clone().unwrap())
            })
            .r#if(self.storage_path.clone().is_some(), |w| {
                w.and()
                    .eq("storage_path", self.storage_path.clone().unwrap())
            })
            .r#if(self.access_url.clone().is_some(), |w| {
                w.and().eq("access_url", self.access_url.clone().unwrap())
            })
            .r#if(self.file_ext.clone().is_some(), |w| {
                w.and().eq("file_ext", self.file_ext.clone().unwrap())
            })
            .r#if(self.content_type.clone().is_some(), |w| {
                w.and()
                    .eq("content_type", self.content_type.clone().unwrap())
            })
            .r#if(self.file_size.clone().is_some(), |w| {
                w.and().eq("file_size", self.file_size.unwrap())
            })
            .r#if(self.file_size_display.clone().is_some(), |w| {
                w.and()
                    .eq("file_size_display", self.file_size_display.clone().unwrap())
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

    pub async fn find_attachments(
        rb: &Rbatis,
        busitype: &String,
        busiid: &Option<i64>,
    ) -> Result<Vec<Self>, Error> {
        let sql = "SELECT p.* FROM chimes_attachments p INNER JOIN chimes_attachment_rel rp ON p.attachment_id = rp.attachment_id WHERE rp.business_name = ? AND rp.business_id = ?".to_string();
        let mut rb_args = vec![];
        rb_args.push(rbson::to_bson(busitype).unwrap_or_default());
        rb_args.push(rbson::to_bson(busiid).unwrap_or_default());
        rb.fetch(&sql, rb_args).await
    }
}
