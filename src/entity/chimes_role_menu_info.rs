use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_role_menu_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"chimes_roles_menus"|table_columns:"menu_id,role_id")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesRoleMenuInfo {
    pub menu_id: Option<i64>,
    pub role_id: Option<i64>,
}

impl ChimesRoleMenuInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, role_id: &i64, menu_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .eq("role_id", role_id)
            .and()
            .eq("menu_id", menu_id);
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
            .eq("role_id", self.role_id)
            .and()
            .eq("menu_id", self.menu_id);
        rb.update_by_wrapper(
            self,
            wp,
            &[Skip::Column("role_id"), Skip::Column("menu_id")],
        )
        .await
    }

    #[allow(dead_code)]
    pub async fn remove_role_ids(rb: &Rbatis, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#in("role_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_menu_ids(rb: &Rbatis, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#in("menu_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.menu_id.clone().is_some(), |w| {
                w.and().eq("menu_id", self.menu_id.unwrap())
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
            .eq("role_id", self.role_id)
            .and()
            .eq("menu_id", self.menu_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_list(&self, rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.menu_id.clone().is_some(), |w| {
                w.and().eq("menu_id", self.menu_id.unwrap())
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
