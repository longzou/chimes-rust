use chimes_utils::rbatis_compatible_sql;
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for query_user_permission.rs,
 */
use std::fmt::Debug;

use rbatis::crud::CRUD;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryUserPermissionParams {
    pub api_method: String,
    pub api_pattern: String,
    pub username: String,
}

#[crud_table(table_name:"QueryUserPermission"|table_columns:"id,alias,create_time,name,pid,api_pattern,service_id,api_method,api_bypass,username")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryUserPermission {
    pub id: Option<i64>,
    pub alias: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub name: Option<String>,
    pub pid: Option<i64>,
    pub api_pattern: Option<String>,
    pub service_id: Option<String>,
    pub api_method: Option<String>,
    pub api_bypass: Option<String>,
    pub username: Option<String>,
}

impl QueryUserPermission {
    #[allow(dead_code)]
    pub async fn query(
        rb: &Rbatis,
        param: &QueryUserPermissionParams,
    ) -> Result<Vec<QueryUserPermission>, Error> {
        let sql = "SELECT distinct p.*, cu.username FROM chimes_permission p LEFT JOIN chimes_roles_permissions rp ON p.id = rp.id  LEFT JOIN chimes_users_roles cur ON rp.role_id = cur.role_id  LEFT JOIN chimes_user cu ON cur.user_id  = cu.user_id AND cu.username = ?  WHERE (p.api_method = ? OR p.api_method = '*') AND p.api_pattern  = ?".to_string();
        let mut rb_args = vec![];

        let sql = rbatis_compatible_sql(rb, &sql);

        log::info!("refined sql: {}", sql);

        rb_args.push(rbson::to_bson(&param.username).unwrap_or_default());
        rb_args.push(rbson::to_bson(&param.api_method).unwrap_or_default());
        rb_args.push(rbson::to_bson(&param.api_pattern).unwrap_or_default());

        rb.fetch(&sql, rb_args).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(
        rb: &Rbatis,
        param: &QueryUserPermissionParams,
        curr: u64,
        size: u64,
    ) -> Result<Page<QueryUserPermission>, Error> {
        let sql = "SELECT  distinct p.*, cu.username FROM chimes_permission p LEFT JOIN chimes_roles_permissions rp ON p.id = rp.id  LEFT JOIN chimes_users_roles cur ON rp.role_id = cur.role_id  LEFT JOIN chimes_user cu ON cur.user_id  = cu.user_id AND cu.username = ? WHERE (p.api_method = ? OR p.api_method = '*') AND p.api_pattern  = ?".to_string();
        let mut rb_args = vec![];
        rb_args.push(rbson::to_bson(&param.username).unwrap_or_default());
        rb_args.push(rbson::to_bson(&param.api_method).unwrap_or_default());
        rb_args.push(rbson::to_bson(&param.api_pattern).unwrap_or_default());

        rb.fetch_page(&sql, rb_args, &PageRequest::new(curr, size))
            .await
    }
}
