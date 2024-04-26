use chimes_utils::bool_from_str;
use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use rbson::Bson;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_job_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_job"|table_columns:"job_id,name,enabled,job_sort,create_by,update_by,create_time,update_time")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesJobInfo {
    pub job_id: Option<i64>,
    pub name: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub enabled: Option<bool>,
    pub job_sort: Option<i32>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
}

impl ChimesJobInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, job_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("job_id", job_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        match rb.save(self, &[Skip::Column("job_id")]).await {
            Ok(ds) => {
                self.job_id = ds.last_insert_id;
                Ok(ds.rows_affected)
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("job_id", self.job_id);
        rb.update_by_wrapper(self, wp, &[Skip::Column("job_id")])
            .await
    }

    #[allow(dead_code)]
    pub async fn update_selective(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("job_id", self.job_id);
        rb.update_by_wrapper(self, wp, &[Skip::Value(Bson::Null)])
            .await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.job_id.clone().is_some(), |w| {
                w.and().eq("job_id", self.job_id.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            })
            .r#if(self.job_sort.clone().is_some(), |w| {
                w.and().eq("job_sort", self.job_sort.unwrap())
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
        let wp = rb.new_wrapper().eq("job_id", self.job_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_ids(rb: &Rbatis, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#in("job_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.job_id.clone().is_some(), |w| {
                w.and().eq("job_id", self.job_id.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            })
            .r#if(self.job_sort.clone().is_some(), |w| {
                w.and().eq("job_sort", self.job_sort.unwrap())
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
            .r#if(self.job_id.clone().is_some(), |w| {
                w.and().eq("job_id", self.job_id.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            })
            .r#if(self.job_sort.clone().is_some(), |w| {
                w.and().eq("job_sort", self.job_sort.unwrap())
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
    pub async fn query_all(rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb.new_wrapper();
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesJobInfoQuery {
    #[serde(default)]
    pub job_ids: Vec<i64>,
    pub job_id: Option<i64>,
    pub name: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub enabled: Option<bool>,
    pub job_sort: Option<i32>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    #[serde(default)]
    pub create_time: Vec<rbatis::DateTimeNative>,
    #[serde(default)]
    pub update_time: Vec<rbatis::DateTimeNative>,
}

impl ChimesJobInfoQuery {
    #[allow(dead_code)]
    pub async fn query_paged(
        &self,
        rb: &Rbatis,
        curr: u64,
        ps: u64,
    ) -> Result<Page<ChimesJobInfo>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.job_id.clone().is_some(), |w| {
                w.and().eq("job_id", self.job_id.unwrap())
            })
            .r#if(!self.job_ids.clone().is_empty(), |w| {
                w.and().r#in("job_id", &self.job_ids.clone())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            })
            .r#if(self.job_sort.clone().is_some(), |w| {
                w.and().eq("job_sort", self.job_sort.unwrap())
            })
            .r#if(self.create_by.clone().is_some(), |w| {
                w.and().eq("create_by", self.create_by.clone().unwrap())
            })
            .r#if(self.update_by.clone().is_some(), |w| {
                w.and().eq("update_by", self.update_by.clone().unwrap())
            })
            .r#if(self.create_time.clone().len() == 1, |w| {
                w.and().gt("create_time", self.create_time[0])
            })
            .r#if(self.create_time.clone().len() >= 2, |w| {
                w.and()
                    .between("create_time", self.create_time[0], self.create_time[1])
            })
            .r#if(self.update_time.clone().len() == 1, |w| {
                w.and().gt("update_time", self.update_time[0])
            })
            .r#if(self.update_time.clone().len() >= 2, |w| {
                w.and()
                    .between("update_time", self.update_time[0], self.update_time[1])
            });
        rb.fetch_page_by_wrapper::<ChimesJobInfo>(wp, &PageRequest::new(curr, ps))
            .await
    }

    #[allow(dead_code)]
    pub async fn query_list(&self, rb: &Rbatis) -> Result<Vec<ChimesJobInfo>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.job_id.clone().is_some(), |w| {
                w.and().eq("job_id", self.job_id.unwrap())
            })
            .r#if(!self.job_ids.clone().is_empty(), |w| {
                w.and().r#in("job_id", &self.job_ids.clone())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            })
            .r#if(self.job_sort.clone().is_some(), |w| {
                w.and().eq("job_sort", self.job_sort.unwrap())
            })
            .r#if(self.create_by.clone().is_some(), |w| {
                w.and().eq("create_by", self.create_by.clone().unwrap())
            })
            .r#if(self.update_by.clone().is_some(), |w| {
                w.and().eq("update_by", self.update_by.clone().unwrap())
            })
            .r#if(self.create_time.clone().len() == 1, |w| {
                w.and().gt("create_time", self.create_time[0])
            })
            .r#if(self.create_time.clone().len() >= 2, |w| {
                w.and()
                    .between("create_time", self.create_time[0], self.create_time[1])
            })
            .r#if(self.update_time.clone().len() == 1, |w| {
                w.and().gt("update_time", self.update_time[0])
            })
            .r#if(self.update_time.clone().len() >= 2, |w| {
                w.and()
                    .between("update_time", self.update_time[0], self.update_time[1])
            });
        rb.fetch_list_by_wrapper::<ChimesJobInfo>(wp).await
    }
}
