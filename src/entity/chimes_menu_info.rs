use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::rbatis::Rbatis;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_menu_info.rs,
 */
use std::fmt::Debug;

use chimes_utils::{bool_from_str, i32_from_str};
use rbatis::error::Error;
use rbatis::Page;
use rbatis::PageRequest;
use rbson::Bson;

#[crud_table(table_name:"chimes_menu"|table_columns:"menu_id,pid,sub_count,type,title,name,component,menu_sort,icon,path,i_frame,cache,hidden,permission,create_by,update_by,create_time,update_time")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesMenuInfo {
    pub menu_id: Option<i64>,
    pub pid: Option<i64>,
    pub sub_count: Option<i32>,
    #[serde(rename(deserialize = "type"))]
    #[serde(default)]
    #[serde(deserialize_with = "i32_from_str")]
    pub r#type: Option<i32>,
    pub title: Option<String>,
    pub name: Option<String>,
    pub component: Option<String>,
    pub menu_sort: Option<i32>,
    pub icon: Option<String>,
    pub path: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub i_frame: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub cache: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub hidden: Option<bool>,
    pub permission: Option<String>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
}

impl ChimesMenuInfo {
    pub async fn from_id(rb: &Rbatis, menu_id: &i64) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("menu_id", menu_id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    pub async fn save(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        match rb.save(self, &[Skip::Column("menu_id")]).await {
            Ok(ds) => {
                self.menu_id = ds.last_insert_id;
                Ok(ds.rows_affected)
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("menu_id", self.menu_id);
        rb.update_by_wrapper(self, wp, &[Skip::Column("menu_id")])
            .await
    }

    pub async fn update_selective(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("menu_id", self.menu_id);
        rb.update_by_wrapper(self, wp, &[Skip::Value(Bson::Null)])
            .await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.menu_id.clone().is_some(), |w| {
                w.and().eq("menu_id", self.menu_id.unwrap())
            })
            .r#if(self.pid.clone().is_some(), |w| {
                w.and().eq("pid", self.pid.unwrap())
            })
            .r#if(self.pid.clone().is_none(), |w| w.and().eq("pid", 0))
            .r#if(self.sub_count.clone().is_some(), |w| {
                w.and().eq("sub_count", self.sub_count.unwrap())
            })
            .r#if(self.r#type.clone().is_some(), |w| {
                w.and().eq("type", self.r#type.unwrap())
            })
            .r#if(self.title.clone().is_some(), |w| {
                w.and().eq("title", self.title.clone().unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.component.clone().is_some(), |w| {
                w.and().eq("component", self.component.clone().unwrap())
            })
            .r#if(self.menu_sort.clone().is_some(), |w| {
                w.and().eq("menu_sort", self.menu_sort.unwrap())
            })
            .r#if(self.icon.clone().is_some(), |w| {
                w.and().eq("icon", self.icon.clone().unwrap())
            })
            .r#if(self.path.clone().is_some(), |w| {
                w.and().eq("path", self.path.clone().unwrap())
            })
            .r#if(self.i_frame.clone().is_some(), |w| {
                w.and().eq("i_frame", self.i_frame.unwrap())
            })
            .r#if(self.cache.clone().is_some(), |w| {
                w.and().eq("cache", self.cache.unwrap())
            })
            .r#if(self.hidden.clone().is_some(), |w| {
                w.and().eq("hidden", self.hidden.unwrap())
            })
            .r#if(self.permission.clone().is_some(), |w| {
                w.and().eq("permission", self.permission.clone().unwrap())
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
    pub async fn query_all(rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb.new_wrapper();
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    pub async fn remove(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("menu_id", self.menu_id);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_ids(rb: &Rbatis, ids: &[i64]) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#in("menu_id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.menu_id.clone().is_some(), |w| {
                w.and().eq("menu_id", self.menu_id.unwrap())
            })
            .r#if(self.pid.clone().is_some(), |w| {
                w.and().eq("pid", self.pid.unwrap())
            })
            .r#if(self.pid.clone().is_none(), |w| w.and().eq("pid", 0))
            .r#if(self.sub_count.clone().is_some(), |w| {
                w.and().eq("sub_count", self.sub_count.unwrap())
            })
            .r#if(self.r#type.clone().is_some(), |w| {
                w.and().eq("type", self.r#type.unwrap())
            })
            .r#if(self.title.clone().is_some(), |w| {
                w.and().eq("title", self.title.clone().unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.component.clone().is_some(), |w| {
                w.and().eq("component", self.component.clone().unwrap())
            })
            .r#if(self.menu_sort.clone().is_some(), |w| {
                w.and().eq("menu_sort", self.menu_sort.unwrap())
            })
            .r#if(self.icon.clone().is_some(), |w| {
                w.and().eq("icon", self.icon.clone().unwrap())
            })
            .r#if(self.path.clone().is_some(), |w| {
                w.and().eq("path", self.path.clone().unwrap())
            })
            .r#if(self.i_frame.clone().is_some(), |w| {
                w.and().eq("i_frame", self.i_frame.unwrap())
            })
            .r#if(self.cache.clone().is_some(), |w| {
                w.and().eq("cache", self.cache.unwrap())
            })
            .r#if(self.hidden.clone().is_some(), |w| {
                w.and().eq("hidden", self.hidden.unwrap())
            })
            .r#if(self.permission.clone().is_some(), |w| {
                w.and().eq("permission", self.permission.clone().unwrap())
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
    pub async fn query_root_paged(
        &self,
        rb: &Rbatis,
        curr: u64,
        ps: u64,
    ) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.menu_id.clone().is_some(), |w| {
                w.and().eq("menu_id", self.menu_id.unwrap())
            })
            .r#if(self.pid.clone().is_some(), |w| {
                w.and().eq("pid", self.pid.unwrap())
            })
            .r#if(self.pid.clone().is_none(), |w| w.and().eq("pid", 0))
            .r#if(self.sub_count.clone().is_some(), |w| {
                w.and().eq("sub_count", self.sub_count.unwrap())
            })
            .r#if(self.r#type.clone().is_some(), |w| {
                w.and().eq("type", self.r#type.unwrap())
            })
            .r#if(self.title.clone().is_some(), |w| {
                w.and().eq("title", self.title.clone().unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.component.clone().is_some(), |w| {
                w.and().eq("component", self.component.clone().unwrap())
            })
            .r#if(self.menu_sort.clone().is_some(), |w| {
                w.and().eq("menu_sort", self.menu_sort.unwrap())
            })
            .r#if(self.icon.clone().is_some(), |w| {
                w.and().eq("icon", self.icon.clone().unwrap())
            })
            .r#if(self.path.clone().is_some(), |w| {
                w.and().eq("path", self.path.clone().unwrap())
            })
            .r#if(self.i_frame.clone().is_some(), |w| {
                w.and().eq("i_frame", self.i_frame.unwrap())
            })
            .r#if(self.cache.clone().is_some(), |w| {
                w.and().eq("cache", self.cache.unwrap())
            })
            .r#if(self.hidden.clone().is_some(), |w| {
                w.and().eq("hidden", self.hidden.unwrap())
            })
            .r#if(self.permission.clone().is_some(), |w| {
                w.and().eq("permission", self.permission.clone().unwrap())
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
            .r#if(self.menu_id.clone().is_some(), |w| {
                w.and().eq("menu_id", self.menu_id.unwrap())
            })
            .r#if(self.pid.clone().is_some(), |w| {
                w.and().eq("pid", self.pid.unwrap())
            })
            .r#if(self.pid.clone().is_none(), |w| w.and().eq("pid", 0))
            .r#if(self.sub_count.clone().is_some(), |w| {
                w.and().eq("sub_count", self.sub_count.unwrap())
            })
            .r#if(self.r#type.clone().is_some(), |w| {
                w.and().eq("type", self.r#type.unwrap())
            })
            .r#if(self.title.clone().is_some(), |w| {
                w.and().eq("title", self.title.clone().unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.component.clone().is_some(), |w| {
                w.and().eq("component", self.component.clone().unwrap())
            })
            .r#if(self.menu_sort.clone().is_some(), |w| {
                w.and().eq("menu_sort", self.menu_sort.unwrap())
            })
            .r#if(self.icon.clone().is_some(), |w| {
                w.and().eq("icon", self.icon.clone().unwrap())
            })
            .r#if(self.path.clone().is_some(), |w| {
                w.and().eq("path", self.path.clone().unwrap())
            })
            .r#if(self.i_frame.clone().is_some(), |w| {
                w.and().eq("i_frame", self.i_frame.unwrap())
            })
            .r#if(self.cache.clone().is_some(), |w| {
                w.and().eq("cache", self.cache.unwrap())
            })
            .r#if(self.hidden.clone().is_some(), |w| {
                w.and().eq("hidden", self.hidden.unwrap())
            })
            .r#if(self.permission.clone().is_some(), |w| {
                w.and().eq("permission", self.permission.clone().unwrap())
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
    fn find_children(list: &Vec<Self>, pid: &i64) -> Vec<Self> {
        let mut found = vec![];
        for it in list.clone() {
            if it.pid == Some(*pid) {
                found.push(it.clone());
                let nid = it.menu_id.unwrap_or_default();
                let mut mt = Self::find_children(list, &nid);
                if !mt.is_empty() {
                    found.append(&mut mt);
                }
            }
        }
        found
    }

    #[allow(dead_code)]
    pub async fn query_children(rb: &Rbatis, pid: &i64) -> Result<Vec<Self>, Error> {
        match Self::query_all(rb).await {
            Ok(rs) => {
                let mut found = Self::find_children(&rs, pid);
                for it in rs.clone() {
                    if it.menu_id == Some(*pid) {
                        found.insert(0, it);
                    }
                }
                Ok(found)
            }
            Err(err) => Err(err),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesMenuInfoValue {
    pub id: Option<i64>,
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
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub i_frame: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub cache: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub hidden: Option<bool>,
    pub permission: Option<String>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
    pub leaf: bool,
    pub label: Option<String>,
    pub has_children: bool,
    #[serde(default)]
    pub children: Vec<ChimesMenuInfoValue>,
}

impl ChimesMenuInfoValue {
    #[allow(dead_code)]
    pub fn from_entity(param: &ChimesMenuInfo) -> Self {
        Self {
            id: param.menu_id,
            pid: param.pid,
            sub_count: param.sub_count,
            r#type: param.r#type,
            title: param.title.clone(),
            name: param.name.clone(),
            component: param.component.clone(),
            menu_sort: param.menu_sort,
            icon: param.icon.clone(),
            path: param.path.clone(),
            i_frame: param.i_frame,
            cache: param.cache,
            hidden: param.hidden,
            permission: param.permission.clone(),
            create_by: param.create_by.clone(),
            update_by: param.update_by.clone(),
            create_time: param.create_time,
            update_time: param.update_time,
            has_children: false,
            leaf: false,
            children: vec![],
            label: param.title.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn from_entity_with(param: &ChimesMenuInfo, haschild: bool, children: &[Self]) -> Self {
        Self {
            id: param.menu_id,
            pid: param.pid,
            sub_count: param.sub_count,
            r#type: param.r#type,
            title: param.title.clone(),
            name: param.name.clone(),
            component: param.component.clone(),
            menu_sort: param.menu_sort,
            icon: param.icon.clone(),
            path: param.path.clone(),
            i_frame: param.i_frame,
            cache: param.cache,
            hidden: param.hidden,
            permission: param.permission.clone(),
            create_by: param.create_by.clone(),
            update_by: param.update_by.clone(),
            create_time: param.create_time,
            update_time: param.update_time,
            has_children: haschild,
            leaf: !haschild,
            children: children.to_vec(),
            label: param.title.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn to_entity(&self) -> ChimesMenuInfo {
        ChimesMenuInfo {
            menu_id: self.id,
            pid: self.pid,
            sub_count: self.sub_count,
            r#type: self.r#type,
            title: self.title.clone(),
            name: self.name.clone(),
            component: self.component.clone(),
            menu_sort: self.menu_sort,
            icon: self.icon.clone(),
            path: self.path.clone(),
            i_frame: self.i_frame,
            cache: self.cache,
            hidden: self.hidden,
            permission: self.permission.clone(),
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
                    mip.leaf = true;
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
