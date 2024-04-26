use std::collections::HashMap;

use crate::entity::{ChimesPermissionInfo, ChimesPermissionInfoValue};
use actix_web::HttpRequest;
use actix_web::{web, HttpResponse, Result};
use chimes_auth::ApiResult;
use chimes_utils::get_rbatis;
/**
 * Generate the file for chimes_permission_info.rs,
 */
use rbatis::Page;

#[post("/api/v1/permission/save")]
pub async fn permission_save(req: web::Json<ChimesPermissionInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.create_time = Some(rbatis::DateTimeNative::now());
    match val.save(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesPermissionInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesPermissionInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/permission/update")]
async fn permission_update(req: web::Json<ChimesPermissionInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match val.update_selective(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesPermissionInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesPermissionInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/permission/delete")]
pub async fn permission_delete(req: web::Json<ChimesPermissionInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    match val.remove(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesPermissionInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesPermissionInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/permission/delete_ids")]
pub async fn permission_delete_ids(req: web::Json<Vec<i64>>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let ids = req.as_slice();
    match ChimesPermissionInfo::remove_ids(rb, ids).await {
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

#[post("/api/v1/permission/search")]
pub async fn permission_search(req: web::Json<ChimesPermissionInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match val.query_list(rb).await {
        Ok(st) => {
            let mtts: Vec<ChimesPermissionInfoValue> = st
                .into_iter()
                .map(|f| ChimesPermissionInfoValue::from_entity_with(&f, true, &[]))
                .collect();
            let ret: web::Json<ApiResult<Vec<ChimesPermissionInfoValue>>> =
                web::Json(ApiResult::ok(mtts));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesPermissionInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/permission/paged/{current}/{size}")]
pub async fn permission_paged(
    req: web::Json<ChimesPermissionInfo>,
    path_param: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let (current, size) = path_param.into_inner();
    match val.query_paged(rb, current, size).await {
        Ok(st) => {
            let mtts: Vec<ChimesPermissionInfoValue> = st
                .records
                .into_iter()
                .map(|f| ChimesPermissionInfoValue::from_entity_with(&f, true, &[]))
                .collect();
            let mut newpage = Page::new_total(st.page_no, st.page_size, st.total);
            newpage.records = mtts;
            let ret: web::Json<ApiResult<Page<ChimesPermissionInfoValue>>> =
                web::Json(ApiResult::ok(newpage));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Page<ChimesPermissionInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/permission/get/{id}")]
pub async fn permission_get(id_req: web::Path<i64>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let id = id_req.to_owned();
    match ChimesPermissionInfo::from_id(rb, &id).await {
        Ok(st) => match st {
            Some(tv) => {
                let ret: web::Json<ApiResult<ChimesPermissionInfo>> = web::Json(ApiResult::ok(tv));
                Ok(HttpResponse::Ok().json(ret))
            }
            None => {
                let ret: web::Json<ApiResult<ChimesPermissionInfo>> =
                    web::Json(ApiResult::error(5040, &"Not-Found".to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesPermissionInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/permission/tree")]
pub async fn permission_tree(req: HttpRequest) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let query = req.query_string();
    let dic = crate::utils::parse_query(query);
    let val = crate::utils::get_hash_value(&dic, "pid");
    let valopt = match val.parse::<i64>() {
        Ok(tv) => Some(tv),
        Err(_) => None,
    };
    match ChimesPermissionInfo::query_tree(rb, &valopt).await {
        Ok(st) => {
            let mtts: Vec<ChimesPermissionInfoValue> = st
                .into_iter()
                .map(|f| ChimesPermissionInfoValue::from_entity_with(&f, true, &[]))
                .collect();
            let ret: web::Json<ApiResult<Vec<ChimesPermissionInfoValue>>> =
                web::Json(ApiResult::ok(mtts));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesPermissionInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/permission/children")]
pub async fn permission_children(
    _req: HttpRequest,
    query: web::Query<HashMap<String, i64>>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mt = query.0;
    let oid = mt["pid"];
    match ChimesPermissionInfo::query_children(rb, &oid).await {
        Ok(st) => {
            let mtts: Vec<i64> = st.into_iter().map(|f| f.id.unwrap()).collect();
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
