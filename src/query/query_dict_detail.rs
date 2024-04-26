use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for query_dict_detail.rs,
 */
use std::fmt::Debug;

use rbatis::crud::CRUD;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryDictDetailParams {
    pub dict_name: String,
}

#[crud_table(table_name:"QueryDictDetail"|table_columns:"dict_name,detail_id,dict_id,label,value,dict_sort,create_by,update_by,create_time,update_time")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryDictDetail {
    pub dict_name: Option<String>,
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

impl QueryDictDetail {
    #[allow(dead_code)]
    pub async fn query_all(rb: &Rbatis) -> Result<Vec<QueryDictDetail>, Error> {
        let sql = "select cd.name dict_name, cdd.* from chimes_dict_detail cdd inner join chimes_dict cd on cdd.dict_id  = cd.dict_id ORDER BY cd.name ASC".to_string();
        let rb_args = vec![];
        // rb_args.push(rbson::to_bson(&param.dict_name).unwrap_or_default());
        rb.fetch(&sql, rb_args).await
    }

    #[allow(dead_code)]
    pub async fn query(
        rb: &Rbatis,
        param: &QueryDictDetailParams,
    ) -> Result<Vec<QueryDictDetail>, Error> {
        let sql = "select cd.name dict_name, cdd.* from chimes_dict_detail cdd inner join chimes_dict cd on cdd.dict_id  = cd.dict_id where cd.name  in (?)".to_string();
        let mut rb_args = vec![];
        rb_args.push(rbson::to_bson(&param.dict_name).unwrap_or_default());
        rb.fetch(&sql, rb_args).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(
        rb: &Rbatis,
        param: &QueryDictDetailParams,
        curr: u64,
        size: u64,
    ) -> Result<Page<QueryDictDetail>, Error> {
        let sql = "select cd.name dict_name, cdd.* from chimes_dict_detail cdd inner join chimes_dict cd on cdd.dict_id  = cd.dict_id where cd.name  in (?)".to_string();
        let mut rb_args = vec![];
        rb_args.push(rbson::to_bson(&param.dict_name).unwrap_or_default());
        rb.fetch_page(&sql, rb_args, &PageRequest::new(curr, size))
            .await
    }
}
