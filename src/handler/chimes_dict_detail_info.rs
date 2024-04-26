use crate::entity::ChimesDictDetailInfo;
use actix_web::{web, HttpResponse, Result};
use chimes_auth::ApiResult;
use chimes_utils::{get_rbatis, reload_translate_dicts};
/**
 * Generate the file for chimes_dict_detail_info.rs,
 */
use rbatis::Page;

#[post("/api/v1/dict_detail/save")]
pub async fn dict_detail_save(req: web::Json<ChimesDictDetailInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.create_time = Some(rbatis::DateTimeNative::now());
    val.update_time = Some(rbatis::DateTimeNative::now());
    match val.save(rb).await {
        Ok(_st) => {
            reload_translate_dicts().await;
            let ret: web::Json<ApiResult<ChimesDictDetailInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesDictDetailInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/dict_detail/update")]
async fn dict_detail_update(req: web::Json<ChimesDictDetailInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.update_time = Some(rbatis::DateTimeNative::now());
    match val.update_selective(rb).await {
        Ok(_st) => {
            reload_translate_dicts().await;
            let ret: web::Json<ApiResult<ChimesDictDetailInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesDictDetailInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/dict_detail/delete")]
pub async fn dict_detail_delete(req: web::Json<ChimesDictDetailInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    match val.remove(rb).await {
        Ok(_st) => {
            reload_translate_dicts().await;
            let ret: web::Json<ApiResult<ChimesDictDetailInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesDictDetailInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/dict_detail/delete_ids")]
pub async fn dict_detail_delete_ids(req: web::Json<Vec<i64>>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let ids = req.as_slice();
    match ChimesDictDetailInfo::remove_ids(rb, ids).await {
        Ok(st) => {
            reload_translate_dicts().await;
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

#[post("/api/v1/dict_detail/search")]
pub async fn dict_detail_search(req: web::Json<ChimesDictDetailInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match val.query_list(rb).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Vec<ChimesDictDetailInfo>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesDictDetailInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/dict_detail/paged/{current}/{size}")]
pub async fn dict_detail_paged(
    req: web::Json<ChimesDictDetailInfo>,
    path_param: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let (current, size) = path_param.into_inner();
    match val.query_paged(rb, current, size).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Page<ChimesDictDetailInfo>>> =
                web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Page<ChimesDictDetailInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/dict_detail/get/{id}")]
pub async fn dict_detail_get(detail_id_req: web::Path<i64>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let detail_id = detail_id_req.to_owned();
    match ChimesDictDetailInfo::from_id(rb, &detail_id).await {
        Ok(st) => match st {
            Some(tv) => {
                let ret: web::Json<ApiResult<ChimesDictDetailInfo>> = web::Json(ApiResult::ok(tv));
                Ok(HttpResponse::Ok().json(ret))
            }
            None => {
                let ret: web::Json<ApiResult<ChimesDictDetailInfo>> =
                    web::Json(ApiResult::error(5040, &"Not-Found".to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesDictDetailInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}
