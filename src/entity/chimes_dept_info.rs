/**
 * Generate the file for chimes_dept_info.rs,
 */
use std::fmt::Debug;

use async_recursion::async_recursion;
use chimes_utils::bool_from_str;
use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use rbson::Bson;
use serde_derive::{Deserialize, Serialize};

#[crud_table(table_name:"chimes_dept"|table_columns:"dept_id,pid,sub_count,name,dept_sort,enabled,create_by,update_by,create_time,update_time")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesDeptInfo {
    pub dept_id: Option<i64>,
    pub pid: Option<i64>,
    pub sub_count: Option<i32>,
    pub name: Option<String>,
    pub dept_sort: Option<i32>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub enabled: Option<bool>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
}

impl ChimesDeptInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, dept_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("dept_id", dept_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        match rb.save(self, &[Skip::Column("dept_id")]).await {
            Ok(ds) => {
                self.dept_id = ds.last_insert_id;
                Ok(ds.rows_affected)
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("dept_id", self.dept_id);
        rb.update_by_wrapper(self, wp, &[Skip::Column("dept_id")])
            .await
    }

    #[allow(dead_code)]
    pub async fn update_selective(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("dept_id", self.dept_id);
        rb.update_by_wrapper(self, wp, &[Skip::Value(Bson::Null)])
            .await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.dept_id.clone().is_some(), |w| {
                w.and().eq("dept_id", self.dept_id.unwrap())
            })
            .r#if(self.pid.clone().is_some(), |w| {
                w.and().eq("pid", self.pid.unwrap())
            })
            .r#if(self.sub_count.clone().is_some(), |w| {
                w.and().eq("sub_count", self.sub_count.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.dept_sort.clone().is_some(), |w| {
                w.and().eq("dept_sort", self.dept_sort.unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
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
        let wp = rb.new_wrapper().eq("dept_id", self.dept_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_ids(rb: &Rbatis, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#in("dept_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.dept_id.clone().is_some(), |w| {
                w.and().eq("dept_id", self.dept_id.unwrap())
            })
            .r#if(self.pid.clone().is_some(), |w| {
                w.and().eq("pid", self.pid.unwrap())
            })
            .r#if(self.pid.clone().is_none(), |w| w.and().is_null("pid"))
            .r#if(self.sub_count.clone().is_some(), |w| {
                w.and().eq("sub_count", self.sub_count.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.dept_sort.clone().is_some(), |w| {
                w.and().eq("dept_sort", self.dept_sort.unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
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
            .r#if(self.dept_id.clone().is_some(), |w| {
                w.and().eq("dept_id", self.dept_id.unwrap())
            })
            .r#if(self.pid.clone().is_some(), |w| {
                w.and().eq("pid", self.pid.unwrap())
            })
            .r#if(self.pid.clone().is_none(), |w| w.and().is_null("pid"))
            .r#if(self.sub_count.clone().is_some(), |w| {
                w.and().eq("sub_count", self.sub_count.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.dept_sort.clone().is_some(), |w| {
                w.and().eq("dept_sort", self.dept_sort.unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
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
    pub async fn query_all(&self, rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb.new_wrapper();
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_tree(rb: &Rbatis, pid: &Option<i64>) -> Result<Vec<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(pid.clone().is_some(), |w| w.and().eq("pid", pid.unwrap()))
            .r#if(pid.clone().is_none(), |w| w.and().is_null("pid"));
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    #[async_recursion(?Send)]
    pub async fn query_superior(
        rb: &Rbatis,
        pid: &Option<i64>,
        depts: &mut Vec<ChimesDeptInfo>,
    ) -> Result<Vec<ChimesDeptInfoValue>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(pid.clone().is_some(), |w| w.and().eq("pid", pid.unwrap()))
            .r#if(pid.clone().is_none(), |w| w.and().is_null("pid"));
        match rb.fetch_list_by_wrapper::<Self>(wp).await {
            Ok(rs) => {
                let mut mrs = rs;
                depts.append(&mut mrs);
                if pid.is_none() {
                    return Ok(depts
                        .iter_mut()
                        .map(|f| ChimesDeptInfoValue::from_entity(f))
                        .collect());
                } else {
                    match Self::from_id(rb, &pid.unwrap()).await {
                        Ok(rx) => match rx {
                            Some(x) => {
                                return Self::query_superior(rb, &x.pid, depts).await;
                            }
                            None => {
                                return Ok(depts
                                    .iter_mut()
                                    .map(|f| ChimesDeptInfoValue::from_entity(f))
                                    .collect())
                            }
                        },
                        Err(err) => Err(err),
                    }
                }
            }
            Err(err) => {
                log::warn!("Find the superior of dept with an error, {}", err);
                Err(err)
            }
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesDeptInfoValue {
    pub id: Option<i64>,
    pub pid: Option<i64>,
    pub sub_count: Option<i32>,
    pub name: Option<String>,
    pub label: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub leaf: Option<bool>,
    pub dept_sort: Option<i32>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub enabled: Option<bool>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
    pub has_children: bool,
    #[serde(default)]
    pub children: Vec<ChimesDeptInfoValue>,
}

impl ChimesDeptInfoValue {
    #[allow(dead_code)]
    pub fn from_entity(param: &ChimesDeptInfo) -> Self {
        Self {
            id: param.dept_id,
            pid: param.pid,
            sub_count: param.sub_count,
            name: param.name.clone(),
            label: param.name.clone(),
            leaf: Some(false),
            dept_sort: param.dept_sort,
            enabled: param.enabled,
            create_by: param.create_by.clone(),
            update_by: param.update_by.clone(),
            create_time: param.create_time,
            update_time: param.update_time,
            has_children: false,
            children: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn from_entity_with(param: &ChimesDeptInfo, haschild: bool, children: &[Self]) -> Self {
        Self {
            id: param.dept_id,
            pid: param.pid,
            sub_count: param.sub_count,
            name: param.name.clone(),
            label: param.name.clone(),
            leaf: Some(false),
            dept_sort: param.dept_sort,
            enabled: param.enabled,
            create_by: param.create_by.clone(),
            update_by: param.update_by.clone(),
            create_time: param.create_time,
            update_time: param.update_time,
            has_children: haschild,
            children: children.to_vec(),
        }
    }

    #[allow(dead_code)]
    pub fn to_entity(&self) -> ChimesDeptInfo {
        ChimesDeptInfo {
            dept_id: self.id,
            pid: self.pid,
            sub_count: self.sub_count,
            name: self.name.clone(),
            dept_sort: self.dept_sort,
            enabled: self.enabled,
            create_by: self.create_by.clone(),
            update_by: self.update_by.clone(),
            create_time: self.create_time,
            update_time: self.update_time,
        }
    }

    #[allow(dead_code)]
    fn recurse_build_tree(items: &Vec<Self>, parent_item: &mut Self) {
        for xip in items.clone() {
            if xip.pid == parent_item.id {
                let mut mip = xip;
                Self::recurse_build_tree(items, &mut mip);
                if mip.children.is_empty() {
                    mip.leaf = Some(true);
                    mip.has_children = false;
                }
                parent_item.children.push(mip);
            }
        }
    }

    #[allow(dead_code)]
    pub fn build_tree(items: &Vec<Self>) -> Vec<Self> {
        let mut tmptree = vec![];
        for xip in items.clone() {
            if xip.pid.is_none() || xip.pid == Some(0) {
                tmptree.push(xip.clone());
            }
        }
        let mut tree = vec![];
        for mut it in tmptree {
            Self::recurse_build_tree(items, &mut it);
            tree.push(it);
        }
        tree
    }
}
