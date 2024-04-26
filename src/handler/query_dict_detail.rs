use crate::query::{QueryDictDetail, QueryDictDetailParams};
use actix_web::{web, HttpResponse, Result};
use chimes_auth::ApiResult;
/**
 * Generate the file for query_dict_detail.rs,
 */
use chimes_utils::get_rbatis;
use rbatis::Page;

#[post("/api/v1/dictdetail/paged/{current}/{size}")]
pub async fn query_dict_detail_paged(
    req: web::Json<QueryDictDetailParams>,
    path_param: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let (current, size) = path_param.into_inner();
    match QueryDictDetail::query_paged(rb, &val, current, size).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Page<QueryDictDetail>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Page<QueryDictDetail>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/dictdetail/query")]
pub async fn query_dict_detail_query(
    req: web::Json<QueryDictDetailParams>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match QueryDictDetail::query(rb, &val).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Vec<QueryDictDetail>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<QueryDictDetail>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}
