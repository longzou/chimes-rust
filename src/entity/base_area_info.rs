use async_recursion::async_recursion;
use chimes_utils::{bool_from_str, get_local_timestamp};
use rbatis::crud::{Skip, CRUD};
use rbatis::crud_table;
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use rbatis::Page;
use rbatis::PageRequest;
use rbson::Bson;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for base_area_info.rs,
 */
use std::fmt::Debug;

#[crud_table(table_name:"base_area"|table_columns:"id,create_ms,create_id,update_ms,update_id,del,ver,level,enabled,name,pinyin,ccode,pcode,type,has_children")]
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BaseAreaInfo {
    pub id: Option<String>,
    pub create_ms: Option<i64>,
    pub create_id: Option<String>,
    pub update_ms: Option<i64>,
    pub update_id: Option<String>,
    pub del: Option<i32>,
    pub ver: Option<i32>,
    pub level: Option<i32>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub pinyin: Option<String>,
    pub ccode: Option<String>,
    pub pcode: Option<String>,
    #[serde(rename(deserialize = "type"))]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub has_children: Option<bool>,
}

impl BaseAreaInfo {
    #[allow(dead_code)]
    pub async fn from_id(rb: &Rbatis, id: &String) -> Result<Option<Self>, Error> {
        let wp = rb.new_wrapper().eq("id", id);
        rb.fetch_by_wrapper::<Option<Self>>(wp).await
    }

    #[allow(dead_code)]
    pub async fn save(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        self.create_id = Some("1".to_string());
        self.create_ms = Some(get_local_timestamp() as i64);
        self.update_id = Some("1".to_string());
        self.update_ms = Some(get_local_timestamp() as i64);

        match rb.save(self, &[]).await {
            Ok(ds) => {
                self.update_pcode_haschildren(rb).await?;
                Ok(ds.rows_affected)
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn update(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("id", self.id.clone());
        rb.update_by_wrapper(self, wp, &[Skip::Column("id")]).await
    }

    #[allow(dead_code)]
    pub async fn update_selective(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("id", self.id.clone());
        rb.update_by_wrapper(self, wp, &[Skip::Value(Bson::Null)])
            .await
    }

    #[allow(dead_code)]
    pub async fn update_pcode_haschildren(&self, rb: &Rbatis) -> Result<u64, Error> {
        let pct = Self {
            id: self.pcode.clone(),
            has_children: Some(true),
            ..Default::default()
        };

        pct.update_selective(rb).await
    }

    #[allow(dead_code)]
    pub async fn remove_batch(&self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.id.clone().is_some(), |w| {
                w.and().eq("id", self.id.clone().unwrap())
            })
            .r#if(self.create_ms.clone().is_some(), |w| {
                w.and().eq("create_ms", self.create_ms.unwrap())
            })
            .r#if(self.create_id.clone().is_some(), |w| {
                w.and().eq("create_id", self.create_id.clone().unwrap())
            })
            .r#if(self.update_ms.clone().is_some(), |w| {
                w.and().eq("update_ms", self.update_ms.unwrap())
            })
            .r#if(self.update_id.clone().is_some(), |w| {
                w.and().eq("update_id", self.update_id.clone().unwrap())
            })
            .r#if(self.del.clone().is_some(), |w| {
                w.and().eq("del", self.del.unwrap())
            })
            .r#if(self.ver.clone().is_some(), |w| {
                w.and().eq("ver", self.ver.unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.pinyin.clone().is_some(), |w| {
                w.and().eq("pinyin", self.pinyin.clone().unwrap())
            })
            .r#if(self.ccode.clone().is_some(), |w| {
                w.and().eq("ccode", self.ccode.clone().unwrap())
            })
            .r#if(self.pcode.clone().is_some(), |w| {
                w.and().eq("pcode", self.pcode.clone().unwrap())
            })
            .r#if(self.r#type.clone().is_some(), |w| {
                w.and().eq("type", self.r#type.clone().unwrap())
            })
            .r#if(self.has_children.is_some(), |w| {
                w.and().eq("has_children", self.has_children.unwrap())
            });
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove(&mut self, rb: &Rbatis) -> Result<u64, Error> {
        let wp = rb.new_wrapper().eq("id", self.id.clone());
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn remove_ids(rb: &Rbatis, ids: &[String]) -> Result<u64, Error> {
        let wp = rb.new_wrapper().r#in("id", ids);
        rb.remove_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_paged(&self, rb: &Rbatis, curr: u64, ps: u64) -> Result<Page<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.id.clone().is_some(), |w| {
                w.and().eq("id", self.id.clone().unwrap())
            })
            .r#if(self.create_ms.clone().is_some(), |w| {
                w.and().eq("create_ms", self.create_ms.unwrap())
            })
            .r#if(self.create_id.clone().is_some(), |w| {
                w.and().eq("create_id", self.create_id.clone().unwrap())
            })
            .r#if(self.update_ms.clone().is_some(), |w| {
                w.and().eq("update_ms", self.update_ms.unwrap())
            })
            .r#if(self.update_id.clone().is_some(), |w| {
                w.and().eq("update_id", self.update_id.clone().unwrap())
            })
            .r#if(self.del.clone().is_some(), |w| {
                w.and().eq("del", self.del.unwrap())
            })
            .r#if(self.ver.clone().is_some(), |w| {
                w.and().eq("ver", self.ver.unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.pinyin.clone().is_some(), |w| {
                w.and().eq("pinyin", self.pinyin.clone().unwrap())
            })
            .r#if(self.ccode.clone().is_some(), |w| {
                w.and().eq("ccode", self.ccode.clone().unwrap())
            })
            .r#if(self.pcode.clone().is_some(), |w| {
                w.and().eq("pcode", self.pcode.clone().unwrap())
            })
            .r#if(self.pcode.clone().is_none(), |w| w.and().is_null("pcode"))
            .r#if(self.r#type.clone().is_some(), |w| {
                w.and().eq("type", self.r#type.clone().unwrap())
            })
            .r#if(self.has_children.is_some(), |w| {
                w.and().eq("has_children", self.has_children.unwrap())
            });
        rb.fetch_page_by_wrapper::<Self>(wp, &PageRequest::new(curr, ps))
            .await
    }

    #[allow(dead_code)]
    pub async fn query_list(&self, rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.id.clone().is_some(), |w| {
                w.and().eq("id", self.id.clone().unwrap())
            })
            .r#if(self.create_ms.clone().is_some(), |w| {
                w.and().eq("create_ms", self.create_ms.unwrap())
            })
            .r#if(self.create_id.clone().is_some(), |w| {
                w.and().eq("create_id", self.create_id.clone().unwrap())
            })
            .r#if(self.update_ms.clone().is_some(), |w| {
                w.and().eq("update_ms", self.update_ms.unwrap())
            })
            .r#if(self.update_id.clone().is_some(), |w| {
                w.and().eq("update_id", self.update_id.clone().unwrap())
            })
            .r#if(self.del.clone().is_some(), |w| {
                w.and().eq("del", self.del.unwrap())
            })
            .r#if(self.ver.clone().is_some(), |w| {
                w.and().eq("ver", self.ver.unwrap())
            })
            .r#if(self.enabled.clone().is_some(), |w| {
                w.and().eq("enabled", self.enabled.unwrap())
            })
            .r#if(self.name.clone().is_some(), |w| {
                w.and().eq("name", self.name.clone().unwrap())
            })
            .r#if(self.pinyin.clone().is_some(), |w| {
                w.and().eq("pinyin", self.pinyin.clone().unwrap())
            })
            .r#if(self.ccode.clone().is_some(), |w| {
                w.and().eq("ccode", self.ccode.clone().unwrap())
            })
            .r#if(self.pcode.clone().is_some(), |w| {
                w.and().eq("pcode", self.pcode.clone().unwrap())
            })
            .r#if(self.pcode.clone().is_none(), |w| w.and().is_null("pcode"))
            .r#if(self.r#type.clone().is_some(), |w| {
                w.and().eq("type", self.r#type.clone().unwrap())
            })
            .r#if(self.has_children.is_some(), |w| {
                w.and().eq("has_children", self.has_children.unwrap())
            });
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_pcode(&self, rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(self.del.clone().is_some(), |w| {
                w.and().eq("del", self.del.unwrap())
            })
            .r#if(
                self.enabled.clone().is_some(),
                |w: rbatis::wrapper::Wrapper| w.and().eq("enabled", self.enabled.unwrap()),
            )
            .r#if(self.pcode.clone().is_some(), |w| {
                w.and().eq("pcode", self.pcode.clone().unwrap())
            })
            .r#if(self.pcode.clone().is_none(), |w| w.and().is_null("pcode"));
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_all(rb: &Rbatis) -> Result<Vec<Self>, Error> {
        let wp = rb.new_wrapper();
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn load_children(rb: &Rbatis, pcode: &str) -> Result<Vec<Self>, Error> {
        let wp = rb.new_wrapper().and().eq("pcode", pcode);
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_tree(rb: &Rbatis, pcode: &Option<String>) -> Result<Vec<Self>, Error> {
        let wp = rb
            .new_wrapper()
            .r#if(pcode.clone().is_some(), |w| {
                w.and().eq("pcode", pcode.clone().unwrap())
            })
            .r#if(pcode.clone().is_none(), |w| w.and().is_null("pcode"));
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }

    #[allow(dead_code)]
    pub async fn query_by_level(rb: &Rbatis, level: &Option<i32>) -> Result<Vec<Self>, Error> {
        let wp = rb.new_wrapper().le("level", level);
        rb.fetch_list_by_wrapper::<Self>(wp).await
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BaseAreaInfoValue {
    pub id: Option<String>,
    pub create_ms: Option<i64>,
    pub create_id: Option<String>,
    pub update_ms: Option<i64>,
    pub update_id: Option<String>,
    pub del: Option<i32>,
    pub ver: Option<i32>,
    pub level: Option<i32>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub enabled: Option<bool>,
    pub name: Option<String>,
    pub pinyin: Option<String>,
    pub ccode: Option<String>,
    pub pcode: Option<String>,
    #[serde(rename(deserialize = "type"))]
    pub r#type: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub has_children: Option<bool>,
    pub leaf: bool,
    pub label: Option<String>,
    #[serde(default)]
    pub children: Vec<BaseAreaInfoValue>,
}

impl BaseAreaInfoValue {
    #[allow(dead_code)]
    pub fn from_entity(param: &BaseAreaInfo) -> Self {
        Self {
            id: param.id.clone(),
            create_ms: param.create_ms,
            create_id: param.create_id.clone(),
            update_ms: param.update_ms,
            update_id: param.update_id.clone(),
            del: param.del,
            ver: param.ver,
            level: param.level,
            enabled: param.enabled,
            name: param.name.clone(),
            pinyin: param.pinyin.clone(),
            ccode: param.ccode.clone(),
            pcode: param.pcode.clone(),
            r#type: param.r#type.clone(),
            has_children: param.has_children,
            leaf: false,
            children: vec![],
            label: param.name.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn from_entity_with(param: &BaseAreaInfo, haschild: bool, children: &[Self]) -> Self {
        Self {
            id: param.id.clone(),
            create_ms: param.create_ms,
            create_id: param.create_id.clone(),
            update_ms: param.update_ms,
            update_id: param.update_id.clone(),
            del: param.del,
            ver: param.ver,
            level: param.level,
            enabled: param.enabled,
            name: param.name.clone(),
            pinyin: param.pinyin.clone(),
            ccode: param.ccode.clone(),
            pcode: param.pcode.clone(),
            r#type: param.r#type.clone(),
            has_children: param.has_children,
            leaf: !haschild,
            children: children.to_vec(),
            label: param.name.clone(),
        }
    }

    #[allow(dead_code)]
    pub fn to_entity(&self) -> BaseAreaInfo {
        BaseAreaInfo {
            id: self.id.clone(),
            create_ms: self.create_ms,
            create_id: self.create_id.clone(),
            update_ms: self.update_ms,
            update_id: self.update_id.clone(),
            del: self.del,
            ver: self.ver,
            level: self.level,
            enabled: self.enabled,
            name: self.name.clone(),
            pinyin: self.pinyin.clone(),
            ccode: self.ccode.clone(),
            pcode: self.pcode.clone(),
            r#type: self.r#type.clone(),
            has_children: self.has_children,
        }
    }

    fn recurse_build_tree(items: &[Self], parent_item: &mut Self) {
        for xip in items.iter().cloned() {
            if xip.pcode == parent_item.id {
                let mut mip = xip;
                Self::recurse_build_tree(items, &mut mip);
                if mip.children.is_empty() {
                    mip.leaf = true;
                    mip.has_children = Some(false);
                }
                parent_item.children.push(mip);
            }
        }
    }

    /**
     * root 为最后的根的
     * items 为上次查询出来的
     * parent_item 为上次查询出
     */
    #[async_recursion]
    async fn recurse_parent_tree(
        root: &mut Vec<Self>,
        items: &mut Vec<Self>,
        parent_item: &mut Self,
        rb: &Rbatis,
    ) {
        let pit = parent_item.pcode.clone();
        if pit.is_some() && pit != Some("".to_string()) {
            // load all parents
            let cb = BaseAreaInfo {
                pcode: pit.clone(),
                ..Default::default()
            };

            if let Ok(rs) = cb.query_pcode(rb).await {
                let mut valst: Vec<BaseAreaInfoValue> = rs
                    .into_iter()
                    .map(|f| BaseAreaInfoValue::from_entity(&f))
                    .map(|mut f| {
                        if f.id == parent_item.id {
                            f.children.append(items);
                        }
                        f.has_children = Some(true);
                        f.leaf = false;
                        f
                    })
                    .collect();

                if !valst.is_empty() {
                    let pit = valst[0].clone();
                    let pcode = pit.pcode.clone();
                    if pcode.is_some() && pcode != Some("".to_string()) {
                        if let Ok(ss) = BaseAreaInfo::from_id(rb, &pcode.unwrap_or_default()).await
                        {
                            if ss.is_some() {
                                let mit = ss.unwrap();
                                let mut mpit = BaseAreaInfoValue::from_entity(&mit);
                                Self::recurse_parent_tree(root, &mut valst, &mut mpit, rb).await;
                            }
                        }
                    }
                }
            }
        } else {
            // root.push(parent_item.clone());
            parent_item.children.append(items);
            root.push(parent_item.clone());
        }
    }

    #[allow(dead_code)]
    pub fn build_tree(items: &[Self]) -> Vec<Self> {
        let mut tmptree = vec![];
        for xip in items {
            if xip.pcode.is_none() || xip.pcode == Some("".to_owned()) {
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

    #[allow(dead_code)]
    pub async fn build_parent_tree(items: &mut Vec<Self>, rb: &Rbatis) -> Vec<Self> {
        let mut it = if !items.is_empty() {
            items[0].clone()
        } else {
            BaseAreaInfoValue::default()
        };

        let mut tree: Vec<BaseAreaInfoValue> = vec![];
        BaseAreaInfoValue::recurse_parent_tree(&mut tree, items, &mut it, rb).await;
        tree
    }

    #[allow(dead_code)]
    pub fn build_normal_tree(items: &mut [Self], level: i32) -> Vec<Self> {
        if level > 0 {
            let optlevel = Some(level);
            let mut parent = vec![];
            let mut children = vec![];
            for it in items.iter().cloned() {
                if it.level == optlevel {
                    children.push(it);
                } else {
                    parent.push(it);
                }
            }

            let mut tree = vec![];

            for mut pit in parent {
                for cit in children.clone() {
                    if cit.pcode == pit.id {
                        pit.children.push(cit);
                    }
                }
                tree.push(pit)
            }
            Self::build_normal_tree(&mut tree, level - 1)
        } else {
            items.to_vec()
        }
    }
}
