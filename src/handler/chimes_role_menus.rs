use crate::entity::ChimesRoleMenus;
use actix_web::{web, HttpResponse, Result};
use chimes_auth::ApiResult;
/**
 * Generate the file for chimes_role_menus.rs,
 */
use chimes_utils::get_rbatis;

#[get("/api/v1/role_menus/load/{role_id}")]
pub async fn role_menus_rel_load(role_id: web::Path<i64>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    match ChimesRoleMenus::load(rb, &role_id).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Option<ChimesRoleMenus>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Option<ChimesRoleMenus>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/role_menus/remove/{role_id}")]
pub async fn role_menus_rel_remove(role_id: web::Path<i64>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    match ChimesRoleMenus::load(rb, &role_id).await {
        Ok(st) => match st {
            Some(cst) => match cst.remove(rb).await {
                Ok(_) => {
                    let ret: web::Json<ApiResult<ChimesRoleMenus>> = web::Json(ApiResult::ok(cst));
                    Ok(HttpResponse::Ok().json(ret))
                }
                Err(err) => {
                    let ret: web::Json<ApiResult<ChimesRoleMenus>> =
                        web::Json(ApiResult::error(5011, &err.to_string()));
                    Ok(HttpResponse::Ok().json(ret))
                }
            },
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

#[post("/api/v1/role_menus/remove_ids")]
pub async fn role_menus_rel_remove_ids(role_ids: web::Json<Vec<i64>>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut ret = None;
    for id in role_ids.clone() {
        if ret.is_none() {
            match ChimesRoleMenus::load(rb, &id).await {
                Ok(st) => {
                    match st {
                        Some(cst) => {
                            match cst.remove(rb).await {
                                Ok(_) => {}
                                Err(err) => {
                                    ret = Some(err);
                                }
                            };
                        }
                        None => {
                            ret = None;
                        }
                    };
                }
                Err(err) => {
                    ret = Some(err);
                }
            };
        }
    }

    match ret {
        None => {
            let ret: web::Json<ApiResult<&str>> = web::Json(ApiResult::ok("OK"));
            Ok(HttpResponse::Ok().json(ret))
        }
        Some(err) => {
            let ret: web::Json<ApiResult<&str>> =
                web::Json(ApiResult::error(5011, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/role_menus/save")]
pub async fn role_menus_rel_save(req: web::Json<ChimesRoleMenus>) -> Result<HttpResponse> {
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
