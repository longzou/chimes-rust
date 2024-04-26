use std::collections::HashMap;

use crate::entity::{ChimesDeptInfo, ChimesDeptInfoValue};
use actix_web::HttpRequest;
use actix_web::{web, HttpResponse, Result};
use chimes_auth::ApiResult;
use chimes_utils::get_rbatis;
/**
 * Generate the file for chimes_dept_info.rs,
 */
use rbatis::Page;

#[post("/api/v1/dept/save")]
pub async fn dept_save(req: web::Json<ChimesDeptInfoValue>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned().to_entity();
    val.create_time = Some(rbatis::DateTimeNative::now());
    val.update_time = Some(rbatis::DateTimeNative::now());
    match val.save(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesDeptInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesDeptInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/dept/update")]
async fn dept_update(req: web::Json<ChimesDeptInfoValue>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned().to_entity();
    val.update_time = Some(rbatis::DateTimeNative::now());
    match val.update_selective(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesDeptInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesDeptInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/dept/delete")]
pub async fn dept_delete(req: web::Json<ChimesDeptInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    match val.remove(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesDeptInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesDeptInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/dept/delete_ids")]
pub async fn dept_delete_ids(req: web::Json<Vec<i64>>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let ids = req.as_slice();
    match ChimesDeptInfo::remove_ids(rb, ids).await {
        Ok(st) => {
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

#[post("/api/v1/dept/search")]
pub async fn dept_search(req: web::Json<ChimesDeptInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match val.query_list(rb).await {
        Ok(st) => {
            let mtts: Vec<ChimesDeptInfoValue> = st
                .into_iter()
                .map(|f| ChimesDeptInfoValue::from_entity_with(&f, true, &[]))
                .collect();
            let ret: web::Json<ApiResult<Vec<ChimesDeptInfoValue>>> =
                web::Json(ApiResult::ok(mtts));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesDeptInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/dept/paged/{current}/{size}")]
pub async fn dept_paged(
    req: web::Json<ChimesDeptInfo>,
    path_param: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let (current, size) = path_param.into_inner();
    match val.query_paged(rb, current, size).await {
        Ok(st) => {
            let mtts: Vec<ChimesDeptInfoValue> = st
                .records
                .into_iter()
                .map(|f| ChimesDeptInfoValue::from_entity_with(&f, true, &[]))
                .collect();
            let mut newpage = Page::new_total(st.page_no, st.page_size, st.total);
            newpage.records = mtts;
            let ret: web::Json<ApiResult<Page<ChimesDeptInfoValue>>> =
                web::Json(ApiResult::ok(newpage));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Page<ChimesDeptInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/dept/get/{id}")]
pub async fn dept_get(dept_id_req: web::Path<i64>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let dept_id = dept_id_req.to_owned();
    match ChimesDeptInfo::from_id(rb, &dept_id).await {
        Ok(st) => match st {
            Some(tv) => {
                let ret: web::Json<ApiResult<ChimesDeptInfo>> = web::Json(ApiResult::ok(tv));
                Ok(HttpResponse::Ok().json(ret))
            }
            None => {
                let ret: web::Json<ApiResult<ChimesDeptInfo>> =
                    web::Json(ApiResult::error(5040, &"Not-Found".to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesDeptInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/dept/tree")]
pub async fn dept_tree(req: HttpRequest) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let query = req.query_string();
    let dic = crate::utils::parse_query(query);
    let val = crate::utils::get_hash_value(&dic, "pid");
    let valopt = match val.parse::<i64>() {
        Ok(tv) => Some(tv),
        Err(_) => None,
    };
    match ChimesDeptInfo::query_tree(rb, &valopt).await {
        Ok(st) => {
            let mtts: Vec<ChimesDeptInfoValue> = st
                .into_iter()
                .map(|f| ChimesDeptInfoValue::from_entity_with(&f, true, &[]))
                .collect();
            let ret: web::Json<ApiResult<Vec<ChimesDeptInfoValue>>> =
                web::Json(ApiResult::ok(mtts));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesDeptInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

/**
 * 根据当前ID
 */
#[post("/api/v1/dept/superior")]
pub async fn dept_superior(pids: web::Json<Vec<i64>>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    // let query = req.query_string();
    // let dic = crate::utils::parse_query(query);
    // let val = crate::utils::get_hash_value(&dic, "pid");
    let _valopt = Some(pids[0]);
    // let mut depts = vec![];
    let ch = ChimesDeptInfo::default();
    match ch.query_all(rb).await {
        Ok(st) => {
            let valst = st
                .into_iter()
                .map(|f| ChimesDeptInfoValue::from_entity(&f))
                .collect();
            let mtts: Vec<ChimesDeptInfoValue> = ChimesDeptInfoValue::build_tree(&valst);
            let ret: web::Json<ApiResult<Vec<ChimesDeptInfoValue>>> =
                web::Json(ApiResult::ok(mtts));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesDeptInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[allow(dead_code)]
fn build_dept_tree(depts: &[ChimesDeptInfoValue]) -> Vec<ChimesDeptInfoValue> {
    let mut tree = vec![];
    let mut treehs = HashMap::new();
    for ix in depts.iter().cloned() {
        treehs.insert(ix.id, ix.clone());
    }
    for ix in depts {
        if treehs.contains_key(&ix.pid.clone()) {
            let pt = treehs.get(&ix.pid.clone()).unwrap();
            let mut nx = pt.clone();
            nx.children.push(ix.to_owned());
            nx.leaf = Some(false);
            treehs.insert(nx.id, nx);
        }
    }
    for mk in treehs {
        let mut kt = mk.1;
        kt.leaf = Some(kt.children.is_empty());
        if kt.pid.is_none() {
            tree.push(kt.to_owned());
        }
    }
    tree
}
