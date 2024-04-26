use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for query_user_menus.rs,
 */
use std::fmt::Debug;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryUserMenusParams {
    pub username: String,
    pub role_codes: Vec<String>,
}

#[crud_table(table_name:"QueryUserMenus"|table_columns:"menu_id,pid,sub_count,type,title,name,component,menu_sort,icon,path,i_frame,cache,hidden,permission,create_by,update_by,create_time,update_time")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryUserMenus {
    pub menu_id: Option<i64>,
    pub pid: Option<i64>,
    pub sub_count: Option<i32>,
    #[serde(rename(deserialize = "type"))]
    pub r#type: Option<i32>,
    pub title: Option<String>,
    pub name: Option<String>,
    pub component: Option<String>,
    pub menu_sort: Option<i32>,
    pub icon: Option<String>,
    pub path: Option<String>,
    pub i_frame: Option<bool>,
    pub cache: Option<bool>,
    pub hidden: Option<bool>,
    pub permission: Option<String>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
}

impl QueryUserMenus {
    #[allow(dead_code)]
    pub async fn query(
        rb: &Rbatis,
        param: &QueryUserMenusParams,
    ) -> Result<Vec<QueryUserMenus>, Error> {
        if param.role_codes.is_empty() {
            let sql = "SELECT p.* FROM chimes_menu p INNER JOIN chimes_roles_menus rp ON p.menu_id = rp.menu_id INNER JOIN chimes_users_roles cur ON rp.role_id = cur.role_id  INNER JOIN chimes_user cu ON cur.user_id  = cu.user_id AND cu.username = ? order by p.menu_sort asc".to_string();
            let mut rb_args = vec![];
            rb_args.push(rbson::to_bson(&param.username).unwrap_or_default());
            rb.fetch(&sql, rb_args).await
        } else {
            let mut sql = "SELECT p.* FROM chimes_menu p INNER JOIN chimes_roles_menus rp ON p.menu_id = rp.menu_id INNER JOIN chimes_role cr ON rp.role_id = cr.role_id WHERE 1 = 1 ".to_string();
            // order by p.menu_sort asc
            let mut rb_args = vec![];
            sql.push_str(" AND cr.role_code IN (");
            for ct in param.role_codes.clone() {
                if Some(&ct) == param.role_codes.clone().last() {
                    sql.push_str("?)");
                } else {
                    sql.push_str("?,");
                }
                rb_args.push(rbson::to_bson(&ct).unwrap_or_default());
            }

            rb.fetch(&sql, rb_args).await
        }
    }
}
