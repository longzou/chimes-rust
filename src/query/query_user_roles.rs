use crate::entity::ChimesRoleInfo;
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for query_user_permission.rs,
 */
use std::fmt::Debug;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryUserRoleParams {
    pub company_code: Option<String>,
    pub username: Option<String>,
    pub user_id: Option<i64>,
    pub role_codes: Vec<String>,
}

#[crud_table(table_name:"QueryUserRoles"|table_columns:"role_id, name, role_code, level, data_scope")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueryUserRole {
    pub role_id: Option<i64>,
    pub name: Option<String>,
    pub role_code: Option<String>,
    pub level: Option<i64>,
    pub data_scope: Option<String>,
}

impl QueryUserRole {
    #[allow(dead_code)]
    pub async fn query(
        rb: &Rbatis,
        param: &QueryUserRoleParams,
    ) -> Result<Vec<QueryUserRole>, Error> {
        if param.role_codes.is_empty() {
            let mut sql = "SELECT cr.role_id, name, role_code, level, cu.data_scope FROM chimes_role cr INNER JOIN chimes_users_roles cur ON cr.role_id  = cur.role_id INNER JOIN chimes_user cu ON cu.user_id  = cur.user_id".to_string();
            let mut rb_args = vec![];
            if param.user_id.is_some() {
                rb_args.push(rbson::to_bson(param.user_id).unwrap_or_default());
                sql.push_str(" WHERE cu.user_id = ? ");
            } else if param.username.is_some() {
                sql.push_str(" WHERE cu.username = ? ");
                rb_args.push(rbson::to_bson(&param.username).unwrap_or_default());
            } else {
                sql.push_str(" WHERE 1 = 0 ");
            }

            rb.fetch(&sql, rb_args).await
        } else {
            match ChimesRoleInfo::query_multicode(rb, &param.role_codes).await {
                Ok(lst) => {
                    let s = lst
                        .into_iter()
                        .map(|f| QueryUserRole {
                            data_scope: f.data_scope.clone(),
                            name: f.name.clone(),
                            role_code: f.role_code.clone(),
                            role_id: f.role_id,
                            level: f.level.map(|f| f as i64),
                        })
                        .collect::<Vec<QueryUserRole>>();

                    Ok(s)
                }
                Err(err) => Err(err),
            }
        }
    }
}
