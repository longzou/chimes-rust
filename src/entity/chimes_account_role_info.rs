use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use rbson::Bson;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_account_role_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_account_roles"|table_columns:"account_id,role_id")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesAccountRoleInfo {
    pub account_id: Option<i64>,
    pub role_id: Option<i64>,
}

impl ChimesAccountRoleInfo {
    #[allow(dead_code)]
    pub async fn from_id(
        rb: &Rbatis,
        account_id: &i64,
        role_id: &i64,
    ) -> Result<Option<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .eq("account_id", account_id)
            .and()
            .eq("role_id", role_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        match rb.save(self, &[]).await {
            Ok(ds) => Ok(ds.rows_affected),
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .eq("account_id", self.account_id)
            .and()
            .eq("role_id", self.role_id);
        rb.update_by_wrapper(
            self,
            wp,
            &[Skip::Column("account_id"), Skip::Column("role_id")],
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn update_selective(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .eq("account_id", self.account_id)
            .and()
            .eq("role_id", self.role_id);
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
            .r#if(self.role_id.clone().is_some(), |w| {
                w.and().eq("role_id", self.role_id.unwrap())
            });
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .eq("account_id", self.account_id)
            .and()
            .eq("role_id", self.role_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.account_id.clone().is_some(), |w| {
                w.and().eq("account_id", self.account_id.unwrap())
            })
            .r#if(self.role_id.clone().is_some(), |w| {
                w.and().eq("role_id", self.role_id.unwrap())
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
            .r#if(self.role_id.clone().is_some(), |w| {
                w.and().eq("role_id", self.role_id.unwrap())
            });
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_all(rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb.new_wrapper();
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }
}
