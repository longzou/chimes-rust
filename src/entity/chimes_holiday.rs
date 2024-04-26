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

#[crud_table(table_name:"chimes_holiday"|table_columns:"id,physical_year,holiday_date,date_type,remark,create_date,update_date")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesHolidayInfo {
    pub id: Option<i64>,
    pub physical_year: Option<i32>,
    pub holiday_date: Option<String>,
    pub date_type: Option<i32>,
    pub remark: Option<String>,
    pub create_date: Option<rbatis::DateTimeNative>,
    pub update_date: Option<rbatis::DateTimeNative>,
}

impl ChimesHolidayInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("id", id);
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
            .r#if(self.physical_year.clone().is_some(), |w| {
                w.and().eq("physical_year", self.physical_year.unwrap())
            })
            .r#if(self.holiday_date.clone().is_some(), |w| {
                w.and()
                    .eq("holiday_date", self.holiday_date.clone().unwrap())
            })
            .r#if(self.date_type.clone().is_some(), |w| {
                w.and().eq("date_type", self.date_type.unwrap())
            })
            .r#if(self.remark.clone().is_some(), |w| {
                w.and().eq("remark", self.remark.clone().unwrap())
            })
            .r#if(self.create_date.clone().is_some(), |w| {
                w.and().eq("create_date", self.create_date.unwrap())
            })
            .r#if(self.update_date.clone().is_some(), |w| {
                w.and().eq("update_date", self.update_date.unwrap())
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
            .r#if(self.physical_year.clone().is_some(), |w| {
                w.and().eq("physical_year", self.physical_year.unwrap())
            })
            .r#if(self.holiday_date.clone().is_some(), |w| {
                w.and()
                    .eq("holiday_date", self.holiday_date.clone().unwrap())
            })
            .r#if(self.date_type.clone().is_some(), |w| {
                w.and().eq("date_type", self.date_type.unwrap())
            })
            .r#if(self.remark.clone().is_some(), |w| {
                w.and().eq("remark", self.remark.clone().unwrap())
            })
            .r#if(self.create_date.clone().is_some(), |w| {
                w.and().eq("create_date", self.create_date.unwrap())
            })
            .r#if(self.update_date.clone().is_some(), |w| {
                w.and().eq("update_date", self.update_date.unwrap())
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
            .r#if(self.physical_year.clone().is_some(), |w| {
                w.and().eq("physical_year", self.physical_year.unwrap())
            })
            .r#if(self.holiday_date.clone().is_some(), |w| {
                w.and()
                    .eq("holiday_date", self.holiday_date.clone().unwrap())
            })
            .r#if(self.date_type.clone().is_some(), |w| {
                w.and().eq("date_type", self.date_type.unwrap())
            })
            .r#if(self.remark.clone().is_some(), |w| {
                w.and().eq("remark", self.remark.clone().unwrap())
            })
            .r#if(self.create_date.clone().is_some(), |w| {
                w.and().eq("create_date", self.create_date.unwrap())
            })
            .r#if(self.update_date.clone().is_some(), |w| {
                w.and().eq("update_date", self.update_date.unwrap())
            });
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_ids(rb: &Rbatis, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#in("id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_physical_year(rb: &Rbatis, fy: i64) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#eq("physical_year", fy);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn find_holiday_info(&self, rb: &Rbatis) -> Result<Option<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#eq("holiday_date", self.holiday_date.clone().unwrap());
        match rb.fetch_list_by_wrapper::<Self>(wp).await {
            Ok(lst) => {
                if lst.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(lst[0].clone()))
                }
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn find_holiday_range(rb: &Rbatis, begin: &rbatis::DateNative, end: &rbatis::DateNative) -> Result<Vec<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .between("holiday_date", begin, end);

        rb.fetch_list_by_wrapper::<Self>(wp).await
    }
}
