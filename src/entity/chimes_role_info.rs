use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use rbson::Bson;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_role_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_role"|table_columns:"role_id,name,role_code,level,description,data_scope,create_by,update_by,create_time,update_time")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesRoleInfo {
    pub role_id: Option<i64>,
    pub name: Option<String>,
    pub role_code: Option<String>,
    pub level: Option<i32>,
    pub description: Option<String>,
    pub data_scope: Option<String>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
}

impl ChimesRoleInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, role_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("role_id", role_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn from_code(rb: &Rbatis, role_code: &String) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("role_code", role_code);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        match rb.save(self, &[Skip::Column("role_id")]).await {
            Ok(ds) => {
                self.role_id = ds.last_insert_id;
                Ok(ds.rows_affected)
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("role_id", self.role_id);
        rb.update_by_wrapper(self, wp, &[Skip::Column("role_id")])
            .await
    }

    #[allow(dead_code)]
    pub async fn update_selective(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("role_id", self.role_id);
        rb.update_by_wrapper(self, wp, &[Skip::Value(Bson::Null)])
            .await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.role_id.clone().is_some(), |w| {
                w.and().eq("role_id", self.role_id.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.role_code.clone().is_some(), |w| {
                w.and().eq("role_code", self.role_code.clone().unwrap())
            })
            .r#if(self.level.clone().is_some(), |w| {
                w.and().eq("level", self.level.unwrap())
            })
            .r#if(self.description.clone().is_some(), |w| {
                w.and().eq("description", self.description.clone().unwrap())
            })
            .r#if(self.data_scope.clone().is_some(), |w| {
                w.and().eq("data_scope", self.data_scope.clone().unwrap())
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
        let wp = rb.new_wrapper().eq("role_id", self.role_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_ids(rb: &Rbatis, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#in("role_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.role_id.clone().is_some(), |w| {
                w.and().eq("role_id", self.role_id.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.role_code.clone().is_some(), |w| {
                w.and().eq("role_code", self.role_code.clone().unwrap())
            })
            .r#if(self.level.clone().is_some(), |w| {
                w.and().eq("level", self.level.unwrap())
            })
            .r#if(self.description.clone().is_some(), |w| {
                w.and().eq("description", self.description.clone().unwrap())
            })
            .r#if(self.data_scope.clone().is_some(), |w| {
                w.and().eq("data_scope", self.data_scope.clone().unwrap())
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
            .r#if(self.role_id.clone().is_some(), |w| {
                w.and().eq("role_id", self.role_id.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.role_code.clone().is_some(), |w| {
                w.and().eq("role_code", self.role_code.clone().unwrap())
            })
            .r#if(self.level.clone().is_some(), |w| {
                w.and().eq("level", self.level.unwrap())
            })
            .r#if(self.description.clone().is_some(), |w| {
                w.and().eq("description", self.description.clone().unwrap())
            })
            .r#if(self.data_scope.clone().is_some(), |w| {
                w.and().eq("data_scope", self.data_scope.clone().unwrap())
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
    pub async fn query_rolecode(&self, rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.role_code.clone().is_some(), |w| {
                w.and()
                    .like_right("role_code", self.role_code.clone().unwrap())
            })
            .order_by(true, &["create_time"]);
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_multicode(rb: &Rbatis, codes: &Vec<String>) -> Result<Vec<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#in("role_code", codes.as_slice())
            .order_by(true, &["create_time"]);
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_all(rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb.new_wrapper();
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }
}
