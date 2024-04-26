use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_user_job_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_users_jobs"|table_columns:"user_id,job_id")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesUserJobInfo {
    pub user_id: Option<i64>,
    pub job_id: Option<i64>,
}

impl ChimesUserJobInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, user_id: &i64, job_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .eq("user_id", user_id)
            .and()
            .eq("job_id", job_id);
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
            .eq("user_id", self.user_id)
            .and()
            .eq("job_id", self.job_id);
        rb.update_by_wrapper(self, wp, &[Skip::Column("user_id"), Skip::Column("job_id")])
            .await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.user_id.clone().is_some(), |w| {
                w.and().eq("user_id", self.user_id.unwrap())
            })
            .r#if(self.job_id.clone().is_some(), |w| {
                w.and().eq("job_id", self.job_id.unwrap())
            });
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .eq("user_id", self.user_id)
            .and()
            .eq("job_id", self.job_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_list(&self, rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.user_id.clone().is_some(), |w| {
                w.and().eq("user_id", self.user_id.unwrap())
            })
            .r#if(self.job_id.clone().is_some(), |w| {
                w.and().eq("job_id", self.job_id.unwrap())
            });
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_all(rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb.new_wrapper();
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }
}
