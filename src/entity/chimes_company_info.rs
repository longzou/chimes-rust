use chimes_utils::i64_from_str;
use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use rbson::Bson;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_company_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_company"|table_columns:"company_id,company_code,company_name,area_code,unform_code,address,legal_person,contact_no,full_city,contact_name,emails,brief,register_name,register_openid,register_unionid,create_time,update_time")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesCompanyInfo {
    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub company_id: Option<i64>,
    pub company_code: Option<String>,
    pub company_name: Option<String>,
    pub area_code: Option<String>,
    pub unform_code: Option<String>,
    pub address: Option<String>,
    pub legal_person: Option<String>,
    pub contact_no: Option<String>,
    pub full_city: Option<String>,
    pub contact_name: Option<String>,
    pub emails: Option<String>,
    pub brief: Option<String>,
    pub register_name: Option<String>,
    pub register_openid: Option<String>,
    pub register_unionid: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
}

impl ChimesCompanyInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, company_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("company_id", company_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        match rb.save(self, &[Skip::Column("company_id")]).await {
            Ok(ds) => {
                self.company_id = ds.last_insert_id;
                Ok(ds.rows_affected)
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("company_id", self.company_id);
        rb.update_by_wrapper(self, wp, &[Skip::Column("company_id")])
            .await
    }

    #[allow(dead_code)]
    pub async fn update_selective(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("company_id", self.company_id);
        rb.update_by_wrapper(self, wp, &[Skip::Value(Bson::Null)])
            .await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.company_id.clone().is_some(), |w| {
                w.and().eq("company_id", self.company_id.unwrap())
            })
            .r#if(self.company_code.clone().is_some(), |w| {
                w.and()
                    .eq("company_code", self.company_code.clone().unwrap())
            })
            .r#if(self.company_name.clone().is_some(), |w| {
                w.and()
                    .eq("company_name", self.company_name.clone().unwrap())
            })
            .r#if(self.area_code.clone().is_some(), |w| {
                w.and().eq("area_code", self.area_code.clone().unwrap())
            })
            .r#if(self.unform_code.clone().is_some(), |w| {
                w.and().eq("unform_code", self.unform_code.clone().unwrap())
            })
            .r#if(self.address.clone().is_some(), |w| {
                w.and().eq("address", self.address.clone().unwrap())
            })
            .r#if(self.legal_person.clone().is_some(), |w| {
                w.and()
                    .eq("legal_person", self.legal_person.clone().unwrap())
            })
            .r#if(self.contact_no.clone().is_some(), |w| {
                w.and().eq("contact_no", self.contact_no.clone().unwrap())
            })
            .r#if(self.full_city.clone().is_some(), |w| {
                w.and().eq("full_city", self.full_city.clone().unwrap())
            })
            .r#if(self.contact_name.clone().is_some(), |w| {
                w.and()
                    .eq("contact_name", self.contact_name.clone().unwrap())
            })
            .r#if(self.emails.clone().is_some(), |w| {
                w.and().eq("emails", self.emails.clone().unwrap())
            })
            .r#if(self.brief.clone().is_some(), |w| {
                w.and().eq("brief", self.brief.clone().unwrap())
            })
            .r#if(self.register_name.clone().is_some(), |w| {
                w.and()
                    .eq("register_name", self.register_name.clone().unwrap())
            })
            .r#if(self.register_openid.clone().is_some(), |w| {
                w.and()
                    .eq("register_openid", self.register_openid.clone().unwrap())
            })
            .r#if(self.register_unionid.clone().is_some(), |w| {
                w.and()
                    .eq("register_unionid", self.register_unionid.clone().unwrap())
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
        let wp = rb.new_wrapper().eq("company_id", self.company_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_ids(rb: &Rbatis, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#in("company_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.company_id.clone().is_some(), |w| {
                w.and().eq("company_id", self.company_id.unwrap())
            })
            .r#if(self.company_code.clone().is_some(), |w| {
                w.and()
                    .eq("company_code", self.company_code.clone().unwrap())
            })
            .r#if(self.company_name.clone().is_some(), |w| {
                w.and()
                    .eq("company_name", self.company_name.clone().unwrap())
            })
            .r#if(self.area_code.clone().is_some(), |w| {
                w.and().eq("area_code", self.area_code.clone().unwrap())
            })
            .r#if(self.unform_code.clone().is_some(), |w| {
                w.and().eq("unform_code", self.unform_code.clone().unwrap())
            })
            .r#if(self.address.clone().is_some(), |w| {
                w.and().eq("address", self.address.clone().unwrap())
            })
            .r#if(self.legal_person.clone().is_some(), |w| {
                w.and()
                    .eq("legal_person", self.legal_person.clone().unwrap())
            })
            .r#if(self.contact_no.clone().is_some(), |w| {
                w.and().eq("contact_no", self.contact_no.clone().unwrap())
            })
            .r#if(self.full_city.clone().is_some(), |w| {
                w.and().eq("full_city", self.full_city.clone().unwrap())
            })
            .r#if(self.contact_name.clone().is_some(), |w| {
                w.and()
                    .eq("contact_name", self.contact_name.clone().unwrap())
            })
            .r#if(self.emails.clone().is_some(), |w| {
                w.and().eq("emails", self.emails.clone().unwrap())
            })
            .r#if(self.brief.clone().is_some(), |w| {
                w.and().eq("brief", self.brief.clone().unwrap())
            })
            .r#if(self.register_name.clone().is_some(), |w| {
                w.and()
                    .eq("register_name", self.register_name.clone().unwrap())
            })
            .r#if(self.register_openid.clone().is_some(), |w| {
                w.and()
                    .eq("register_openid", self.register_openid.clone().unwrap())
            })
            .r#if(self.register_unionid.clone().is_some(), |w| {
                w.and()
                    .eq("register_unionid", self.register_unionid.clone().unwrap())
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
            .r#if(self.company_id.clone().is_some(), |w| {
                w.and().eq("company_id", self.company_id.unwrap())
            })
            .r#if(self.company_code.clone().is_some(), |w| {
                w.and()
                    .eq("company_code", self.company_code.clone().unwrap())
            })
            .r#if(self.company_name.clone().is_some(), |w| {
                w.and()
                    .eq("company_name", self.company_name.clone().unwrap())
            })
            .r#if(self.area_code.clone().is_some(), |w| {
                w.and().eq("area_code", self.area_code.clone().unwrap())
            })
            .r#if(self.unform_code.clone().is_some(), |w| {
                w.and().eq("unform_code", self.unform_code.clone().unwrap())
            })
            .r#if(self.address.clone().is_some(), |w| {
                w.and().eq("address", self.address.clone().unwrap())
            })
            .r#if(self.legal_person.clone().is_some(), |w| {
                w.and()
                    .eq("legal_person", self.legal_person.clone().unwrap())
            })
            .r#if(self.contact_no.clone().is_some(), |w| {
                w.and().eq("contact_no", self.contact_no.clone().unwrap())
            })
            .r#if(self.full_city.clone().is_some(), |w| {
                w.and().eq("full_city", self.full_city.clone().unwrap())
            })
            .r#if(self.contact_name.clone().is_some(), |w| {
                w.and()
                    .eq("contact_name", self.contact_name.clone().unwrap())
            })
            .r#if(self.emails.clone().is_some(), |w| {
                w.and().eq("emails", self.emails.clone().unwrap())
            })
            .r#if(self.brief.clone().is_some(), |w| {
                w.and().eq("brief", self.brief.clone().unwrap())
            })
            .r#if(self.register_name.clone().is_some(), |w| {
                w.and()
                    .eq("register_name", self.register_name.clone().unwrap())
            })
            .r#if(self.register_openid.clone().is_some(), |w| {
                w.and()
                    .eq("register_openid", self.register_openid.clone().unwrap())
            })
            .r#if(self.register_unionid.clone().is_some(), |w| {
                w.and()
                    .eq("register_unionid", self.register_unionid.clone().unwrap())
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

    /**
     * 检测Code是否存在
     */
    #[allow(dead_code)]
    pub async fn code_exist(rb: &Rbatis, code: &str) -> Result<Option<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .and()
            .eq("company_code", code.to_owned())
            .limit(1u64);
        rb.fetch_by_wrapper(wp).await
    }
}
