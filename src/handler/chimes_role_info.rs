use std::collections::HashMap;

use crate::entity::{
    ChimesRoleDeptInfo, ChimesRoleMenuInfo, ChimesRolePermissionInfo, ChimesUserRoleInfo,
};
use crate::entity::{ChimesRoleInfo, ChimesRoleMenus, ChimesUserInfo, ChimesUserRoles};
use crate::utils::SystemUser;
use actix_web::{web, HttpResponse, Result};
use chimes_auth::ApiResult;
use chimes_utils::get_rbatis;
/**
 * Generate the file for chimes_role_info.rs,
 */
use rbatis::Page;

#[post("/api/v1/role/save")]
pub async fn role_save(req: web::Json<ChimesRoleInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.create_time = Some(rbatis::DateTimeNative::now());
    val.update_time = Some(rbatis::DateTimeNative::now());
    match val.save(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesRoleInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesRoleInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/role/update")]
async fn role_update(req: web::Json<ChimesRoleInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.update_time = Some(rbatis::DateTimeNative::now());
    match val.update_selective(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesRoleInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesRoleInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/role/delete")]
pub async fn role_delete(req: web::Json<ChimesRoleInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    match val.remove(rb).await {
        Ok(_st) => {
            let ids = [val.role_id.unwrap_or_default()];
            let _ = ChimesRoleDeptInfo::remove_role_ids(rb, &ids).await.is_ok();
            let _ = ChimesRolePermissionInfo::remove_role_ids(rb, &ids)
                .await
                .is_ok();
            let _ = ChimesUserRoleInfo::remove_role_ids(rb, &ids).await.is_ok();
            let _ = ChimesRoleMenuInfo::remove_role_ids(rb, &ids).await.is_ok();
            let ret: web::Json<ApiResult<ChimesRoleInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesRoleInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/role/delete_ids")]
pub async fn role_delete_ids(req: web::Json<Vec<i64>>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let ids = req.as_slice();
    match ChimesRoleInfo::remove_ids(rb, ids).await {
        Ok(st) => {
            let _ = ChimesRoleDeptInfo::remove_role_ids(rb, ids).await.is_ok();
            let _ = ChimesRolePermissionInfo::remove_role_ids(rb, ids)
                .await
                .is_ok();
            let _ = ChimesUserRoleInfo::remove_role_ids(rb, ids).await.is_ok();
            let _ = ChimesRoleMenuInfo::remove_role_ids(rb, ids).await.is_ok();

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

#[post("/api/v1/role/search")]
pub async fn role_search(req: web::Json<ChimesRoleInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    if val.role_code.is_some()
        && val.role_id.is_none()
        && val.name.is_none()
        && val.level.is_none()
        && val.data_scope.is_none()
    {
        match val.query_rolecode(rb).await {
            Ok(st) => {
                let ret: web::Json<ApiResult<Vec<ChimesRoleInfo>>> = web::Json(ApiResult::ok(st));
                Ok(HttpResponse::Ok().json(ret))
            }
            Err(err) => {
                let ret: web::Json<ApiResult<Vec<ChimesRoleInfo>>> =
                    web::Json(ApiResult::error(5010, &err.to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        }
    } else {
        match val.query_list(rb).await {
            Ok(st) => {
                let ret: web::Json<ApiResult<Vec<ChimesRoleInfo>>> = web::Json(ApiResult::ok(st));
                Ok(HttpResponse::Ok().json(ret))
            }
            Err(err) => {
                let ret: web::Json<ApiResult<Vec<ChimesRoleInfo>>> =
                    web::Json(ApiResult::error(5010, &err.to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        }
    }
}

#[post("/api/v1/role/paged/{current}/{size}")]
pub async fn role_paged(
    req: web::Json<ChimesRoleInfo>,
    path_param: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let (current, size) = path_param.into_inner();
    match val.query_paged(rb, current, size).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Page<ChimesRoleInfo>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Page<ChimesRoleInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/role/get/{id}")]
pub async fn role_get(role_id_req: web::Path<i64>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let role_id = role_id_req.to_owned();
    match ChimesRoleMenus::load(rb, &role_id).await {
        Ok(st) => match st {
            Some(tv) => {
                let ret: web::Json<ApiResult<ChimesRoleMenus>> = web::Json(ApiResult::ok(tv));
                Ok(HttpResponse::Ok().json(ret))
            }
            None => {
                let ret: web::Json<ApiResult<ChimesRoleMenus>> =
                    web::Json(ApiResult::error(5040, &"Not-Found".to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesRoleMenus>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/role/level")]
pub async fn role_level(su: SystemUser<ChimesUserInfo>) -> Result<HttpResponse> {
    let _rb = get_rbatis();

    let user = su.user;
    let user_with_roles = ChimesUserRoles::from_user(&user);
    let min = user_with_roles
        .roles
        .into_iter()
        .map(|f| f.level.unwrap_or_default())
        .min();
    let mut map = HashMap::new();
    map.insert("level", min);
    let ret: web::Json<ApiResult<HashMap<&str, Option<i32>>>> = web::Json(ApiResult::ok(map));
    Ok(HttpResponse::Ok().json(ret))
}
