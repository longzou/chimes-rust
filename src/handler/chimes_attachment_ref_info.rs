use crate::entity::ChimesAttachmentRefInfo;
use crate::entity::ChimesUserInfo;
use crate::utils::SystemUser;
use actix_web::{web, HttpResponse, Result};
use chimes_auth::ApiResult;
use chimes_utils::get_rbatis;
/**
 * Generate the file for chimes_attachment_ref_info.rs,
 */
use rbatis::Page;

#[post("/api/v1/attachref/create")]
pub async fn attachref_save(
    su: SystemUser<ChimesUserInfo>,
    req: web::Json<ChimesAttachmentRefInfo>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.modify_by = su.user.username.clone();
    val.create_time = Some(rbatis::DateTimeNative::now());
    val.update_time = Some(rbatis::DateTimeNative::now());
    match rb.acquire_begin().await {
        Ok(mut tx) => match val.save(&mut tx).await {
            Ok(_st) => match tx.commit().await {
                Ok(_) => {
                    let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                        web::Json(ApiResult::ok(val));
                    Ok(HttpResponse::Ok().json(ret))
                }
                Err(err) => {
                    let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                        web::Json(ApiResult::error(5010, &err.to_string()));
                    Ok(HttpResponse::Ok().json(ret))
                }
            },
            Err(err) => {
                let _ = tx.rollback().await.is_ok();
                let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                    web::Json(ApiResult::error(5010, &err.to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/attachref/update")]
async fn attachref_update(
    su: SystemUser<ChimesUserInfo>,
    req: web::Json<ChimesAttachmentRefInfo>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.modify_by = su.user.username.clone();
    val.update_time = Some(rbatis::DateTimeNative::now());
    match rb.acquire_begin().await {
        Ok(mut tx) => match val.update_selective(&mut tx).await {
            Ok(_st) => match tx.commit().await {
                Ok(_) => {
                    let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                        web::Json(ApiResult::ok(val));
                    Ok(HttpResponse::Ok().json(ret))
                }
                Err(err) => {
                    let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                        web::Json(ApiResult::error(5010, &err.to_string()));
                    Ok(HttpResponse::Ok().json(ret))
                }
            },
            Err(err) => {
                let _ = tx.rollback().await.is_ok();
                let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                    web::Json(ApiResult::error(5010, &err.to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/attachref/delete")]
pub async fn attachref_delete(
    su: SystemUser<ChimesUserInfo>,
    req: web::Json<ChimesAttachmentRefInfo>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.modify_by = su.user.username.clone();
    val.update_time = Some(rbatis::DateTimeNative::now());

    match rb.acquire_begin().await {
        Ok(mut tx) => match val.remove(&mut tx).await {
            Ok(_st) => match tx.commit().await {
                Ok(_) => {
                    let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                        web::Json(ApiResult::ok(val));
                    Ok(HttpResponse::Ok().json(ret))
                }
                Err(err) => {
                    let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                        web::Json(ApiResult::error(5010, &err.to_string()));
                    Ok(HttpResponse::Ok().json(ret))
                }
            },
            Err(err) => {
                let _ = tx.rollback().await.is_ok();
                let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                    web::Json(ApiResult::error(5010, &err.to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/attachref/delete_ids")]
pub async fn attachref_delete_ids(
    _su: SystemUser<ChimesUserInfo>,
    req: web::Json<Vec<i64>>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let ids = req.as_slice();

    match rb.acquire_begin().await {
        Ok(mut tx) => match ChimesAttachmentRefInfo::remove_ids(&mut tx, ids).await {
            Ok(st) => match tx.commit().await {
                Ok(_) => {
                    let ret: web::Json<ApiResult<u64>> = web::Json(ApiResult::ok(st));
                    Ok(HttpResponse::Ok().json(ret))
                }
                Err(err) => {
                    let ret: web::Json<ApiResult<u64>> =
                        web::Json(ApiResult::error(5010, &err.to_string()));
                    Ok(HttpResponse::Ok().json(ret))
                }
            },
            Err(err) => {
                let _ = tx.rollback().await.is_ok();
                let ret: web::Json<ApiResult<u64>> =
                    web::Json(ApiResult::error(5010, &err.to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<u64>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/attachref/search")]
pub async fn attachref_search(
    _su: SystemUser<ChimesUserInfo>,
    req: web::Json<ChimesAttachmentRefInfo>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match val.query_list(rb).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Vec<ChimesAttachmentRefInfo>>> =
                web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesAttachmentRefInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/attachref/paged/{current}/{size}")]
pub async fn attachref_paged(
    _su: SystemUser<ChimesUserInfo>,
    req: web::Json<ChimesAttachmentRefInfo>,
    path_param: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let (current, size) = path_param.into_inner();
    match val.query_paged(rb, current, size).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Page<ChimesAttachmentRefInfo>>> =
                web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Page<ChimesAttachmentRefInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/attachref/get/{id}")]
pub async fn attachref_get(
    _su: SystemUser<ChimesUserInfo>,
    rel_id_req: web::Path<i64>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let rel_id = rel_id_req.to_owned();
    match ChimesAttachmentRefInfo::from_id(rb, &rel_id).await {
        Ok(st) => match st {
            Some(tv) => {
                let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                    web::Json(ApiResult::ok(tv));
                Ok(HttpResponse::Ok().json(ret))
            }
            None => {
                let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                    web::Json(ApiResult::error(5040, &"Not-Found".to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesAttachmentRefInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}
