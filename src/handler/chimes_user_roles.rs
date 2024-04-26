use crate::entity::ChimesUserRoles;
use actix_web::{web, HttpResponse, Result};
use chimes_auth::ApiResult;
/**
 * Generate the file for chimes_user_roles.rs,
 */
use chimes_utils::get_rbatis;

#[get("/api/v1/user_roles/load/{user_id}")]
pub async fn user_roles_rel_load(user_id: web::Path<i64>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    match ChimesUserRoles::load(rb, &user_id).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Option<ChimesUserRoles>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Option<ChimesUserRoles>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/user_roles/remove/{user_id}")]
pub async fn user_roles_rel_remove(user_id: web::Path<i64>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    match ChimesUserRoles::load(rb, &user_id).await {
        Ok(st) => match st {
            Some(cst) => match cst.remove(rb).await {
                Ok(_) => {
                    let ret: web::Json<ApiResult<ChimesUserRoles>> = web::Json(ApiResult::ok(cst));
                    Ok(HttpResponse::Ok().json(ret))
                }
                Err(err) => {
                    let ret: web::Json<ApiResult<ChimesUserRoles>> =
                        web::Json(ApiResult::error(5011, &err.to_string()));
                    Ok(HttpResponse::Ok().json(ret))
                }
            },
            None => {
                let ret: web::Json<ApiResult<ChimesUserRoles>> =
                    web::Json(ApiResult::error(5040, &"Not-Found".to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesUserRoles>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/user_roles/save")]
pub async fn user_roles_rel_save(req: web::Json<ChimesUserRoles>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match val.save(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<String>> = web::Json(ApiResult::ok("SUCCESS".to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<String>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}
