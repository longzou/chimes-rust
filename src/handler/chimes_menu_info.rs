use crate::entity::{ChimesMenuInfo, ChimesMenuInfoValue, ChimesRoleMenuInfo, ChimesUserInfo};
use crate::query::{QueryUserMenus, QueryUserMenusParams};
use crate::utils::{MenuMetadata, MenuTreeModel, SystemUser};
use chimes_utils::get_rbatis;
use std::collections::HashMap;

use actix_web::{web, HttpRequest, HttpResponse, Result};
use chimes_auth::{ApiResult, ChimesAuthUser};
use rbatis::Page;

#[post("/api/v1/menu/save")]
pub async fn menu_save(req: web::Json<ChimesMenuInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.create_time = Some(rbatis::DateTimeNative::now());
    val.update_time = Some(rbatis::DateTimeNative::now());
    match val.save(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesMenuInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesMenuInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/menu/update")]
async fn menu_update(req: web::Json<ChimesMenuInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.update_time = Some(rbatis::DateTimeNative::now());
    match val.update_selective(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesMenuInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesMenuInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/menu/delete")]
pub async fn menu_delete(req: web::Json<ChimesMenuInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    match val.remove(rb).await {
        Ok(_st) => {
            let ids = [val.menu_id.unwrap_or_default()];
            let _ = ChimesRoleMenuInfo::remove_menu_ids(rb, &ids).await.is_ok();
            let ret: web::Json<ApiResult<ChimesMenuInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesMenuInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/menu/delete_ids")]
pub async fn menu_delete_ids(req: web::Json<Vec<i64>>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let ids = req.as_slice();
    match ChimesMenuInfo::remove_ids(rb, ids).await {
        Ok(st) => {
            let _ = ChimesRoleMenuInfo::remove_menu_ids(rb, ids).await.is_ok();
            let ret: web::Json<ApiResult<u64>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<u64>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/menu/search")]
pub async fn menu_search(req: web::Json<ChimesMenuInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match val.query_list(rb).await {
        Ok(st) => {
            let mtts: Vec<ChimesMenuInfoValue> = st
                .into_iter()
                .map(|f| ChimesMenuInfoValue::from_entity_with(&f, true, &[]))
                .collect();
            let ret: web::Json<ApiResult<Vec<ChimesMenuInfoValue>>> =
                web::Json(ApiResult::ok(mtts));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesMenuInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/menu/children")]
pub async fn menu_children(
    _req: HttpRequest,
    query: web::Query<HashMap<String, i64>>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mt = query.0;
    let oid = mt["pid"];
    match ChimesMenuInfo::query_children(rb, &oid).await {
        Ok(st) => {
            let mtts: Vec<i64> = st.into_iter().map(|f| f.menu_id.unwrap()).collect();
            let ret: web::Json<ApiResult<Vec<i64>>> = web::Json(ApiResult::ok(mtts));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<i64>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/menu/paged/{current}/{size}")]
pub async fn menu_paged(
    req: web::Json<ChimesMenuInfo>,
    v1: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let (current, size) = v1.into_inner();

    match val.query_paged(rb, current, size).await {
        Ok(st) => {
            let mtts: Vec<ChimesMenuInfoValue> = st
                .records
                .into_iter()
                .map(|f| ChimesMenuInfoValue::from_entity_with(&f, true, &[]))
                .collect();
            let mut newpage = Page::new_total(st.page_no, st.page_size, st.total);
            newpage.records = mtts;
            let ret: web::Json<ApiResult<Page<ChimesMenuInfoValue>>> =
                web::Json(ApiResult::ok(newpage));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Page<ChimesMenuInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/menu/get/{id}")]
pub async fn menu_get(menu_id_req: web::Path<i64>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let menu_id = menu_id_req.to_owned();
    match ChimesMenuInfo::from_id(rb, &menu_id).await {
        Ok(st) => match st {
            Some(tv) => {
                let ret: web::Json<ApiResult<ChimesMenuInfo>> = web::Json(ApiResult::ok(tv));
                Ok(HttpResponse::Ok().json(ret))
            }
            None => {
                let ret: web::Json<ApiResult<ChimesMenuInfo>> =
                    web::Json(ApiResult::error(5040, &"Not-Found".to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesMenuInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/menus/build")]
pub async fn menu_build(su: SystemUser<ChimesUserInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let company_code = su.user.company_code.clone();

    log::info!("MenuBuild: {}", company_code.clone().unwrap_or_default());
    let mut codes = su.user.parse_simulate_roles();

    if company_code.clone().is_some() && company_code == Some("0000000000".to_string()) {
        let unm = su.get_user_name();
        if unm == *"admin" {
            codes.push("ROLE_COMMONUSER".to_string());
            codes.push("ROLE_SUPERADMIN".to_string());
        } else {
            codes.push("ROLE_COMMONUSER".to_string());
        }
    } else if company_code.clone().is_some() && company_code != Some("0000000000".to_string()) {
        codes.push("ROLE_COMMONUSER".to_string());
    } else {
        codes.clear();
    };
    let param = QueryUserMenusParams {
        username: su.get_user_name(),
        role_codes: codes.clone(),
    };
    log::info!(
        "Calc the menu for {}, roles: {}",
        param.username.clone(),
        codes.join(",")
    );
    match QueryUserMenus::query(rb, &param).await {
        Ok(st) => {
            let mut trees = Vec::new();
            let mut treehs = HashMap::new();
            for mt in st.clone() {
                let item = MenuTreeModel {
                    path: if mt.pid.is_none() || mt.pid == Some(0) {
                        "/".to_string() + mt.path.clone().unwrap_or_default().as_str()
                    } else {
                        mt.path.clone().unwrap_or_default()
                    },
                    component: if !mt.i_frame.unwrap_or_default() {
                        if mt.pid.is_none() || mt.pid == Some(0) {
                            match mt.component.clone() {
                                Some(tt) => {
                                    if tt.is_empty() {
                                        "Layout".to_string()
                                    } else {
                                        tt.clone()
                                    }
                                }
                                None => "Layout".to_string(),
                            }
                        } else if mt.r#type.unwrap_or_default() == 0 {
                            match mt.component.clone() {
                                Some(tt) => {
                                    if tt.is_empty() {
                                        "ParentView".to_string()
                                    } else {
                                        tt.clone()
                                    }
                                }
                                None => "ParentView".to_string(),
                            }
                        } else {
                            mt.component.clone().unwrap_or_default()
                        }
                    } else {
                        "".to_string()
                    },
                    always_show: if mt.pid.is_some() && mt.pid != Some(0) {
                        None
                    } else {
                        Some(true)
                    },
                    redirect: if mt.pid.is_none() || mt.pid == Some(0) {
                        Some("noredirect".to_string())
                    } else {
                        None
                    },
                    name: mt.name.clone().unwrap_or_default(),
                    iframe: mt.i_frame.unwrap_or_default(),
                    // item.cache = mt.cache.clone().unwrap_or_default();
                    hidden: mt.hidden.unwrap_or_default(),
                    children: vec![],
                    id: mt.menu_id.unwrap(),
                    pid: mt.pid,
                    sort: mt.menu_sort.unwrap_or_default(),
                    meta: MenuMetadata {
                        no_cache: !mt.cache.unwrap_or_default(),
                        title: mt.title.clone(),
                        icon: mt.icon.clone(),
                    },
                    ..Default::default()
                };

                treehs.insert(mt.menu_id.unwrap_or_default(), item);
            }

            for mt in treehs.clone() {
                let t = mt.1.clone();
                let pid = t.pid;
                if let Some(nid) = pid {
                    if let Some(tx) = treehs.get(&nid) {
                        let mut mutx = tx.clone();
                        mutx.children.push(t);
                        treehs.insert(nid, mutx);
                    };
                }
            }

            for mt in treehs.clone() {
                let cpid = mt.1.clone().pid;
                if cpid.is_none() || cpid == Some(0) {
                    let mut xmt = mt.1.clone();
                    xmt.children.sort_by(|a, b| a.sort.cmp(&b.sort));
                    trees.push(xmt);
                }
            }

            trees.sort_by(|a, b| a.sort.cmp(&b.sort));

            let ret = web::Json(ApiResult::ok(trees));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(_err) => {
            let trees = Vec::<MenuTreeModel>::new();
            let jp: ApiResult<Vec<MenuTreeModel>> = ApiResult::ok(trees);
            let ret: web::Json<ApiResult<Vec<MenuTreeModel>>> = web::Json(jp);
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

/**
 * 根据当前ID
 */
#[post("/api/v1/menu/superior")]
pub async fn menu_superior(pids: web::Json<Vec<i64>>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    // let query = req.query_string();
    // let dic = crate::utils::parse_query(query);
    // let val = crate::utils::get_hash_value(&dic, "pid");
    let _valopt = Some(pids[0]);
    // let mut depts = vec![];
    match ChimesMenuInfo::query_all(rb).await {
        Ok(st) => {
            let valst = st
                .into_iter()
                .map(|f| ChimesMenuInfoValue::from_entity(&f))
                .collect();
            let mtts: Vec<ChimesMenuInfoValue> = ChimesMenuInfoValue::build_tree(&valst);
            let ret: web::Json<ApiResult<Vec<ChimesMenuInfoValue>>> =
                web::Json(ApiResult::ok(mtts));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesMenuInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}
