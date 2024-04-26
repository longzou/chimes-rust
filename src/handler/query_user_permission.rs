use crate::query::{QueryUserPermission, QueryUserPermissionParams};
use actix_web::{web, HttpResponse, Result};
use chimes_auth::ApiResult;
/**
 * Generate the file for query_user_permission.rs,
 */
use chimes_utils::get_rbatis;
use rbatis::Page;

#[post("/api/v1/users/paged/{current}/{size}")]
pub async fn query_user_permission_paged(
    req: web::Json<QueryUserPermissionParams>,
    path_param: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let (current, size) = path_param.into_inner();
    match QueryUserPermission::query_paged(rb, &val, current, size).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Page<QueryUserPermission>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Page<QueryUserPermission>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/users/query")]
pub async fn query_user_permission_query(
    req: web::Json<QueryUserPermissionParams>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match QueryUserPermission::query(rb, &val).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Vec<QueryUserPermission>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<QueryUserPermission>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}
