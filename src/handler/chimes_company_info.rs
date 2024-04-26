use crate::entity::ChimesCompanyInfo;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use chimes_auth::ApiResult;
use chimes_utils::{generate_rand_string, get_rbatis};
/**
 * Generate the file for chimes_company_info.rs,
 */
use rbatis::Page;

#[post("/api/v1/company/save")]
pub async fn company_save(req: web::Json<ChimesCompanyInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.create_time = Some(rbatis::DateTimeNative::now());
    val.update_time = Some(rbatis::DateTimeNative::now());
    match val.save(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesCompanyInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesCompanyInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/company/unioncode")]
pub async fn company_unioncode() -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut gotcode = None;

    while gotcode.is_none() {
        let code = generate_rand_string(10);
        gotcode = match ChimesCompanyInfo::code_exist(rb, &code).await {
            Ok(t) => {
                if t.is_none() {
                    Some(code)
                } else {
                    None
                }
            }
            Err(_) => Some("ERR".to_string()),
        };
    }

    let ret: web::Json<ApiResult<Option<String>>> = web::Json(ApiResult::ok(gotcode));
    Ok(HttpResponse::Ok().json(ret))
}

#[post("/api/v1/company/update")]
async fn company_update(req: web::Json<ChimesCompanyInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.update_time = Some(rbatis::DateTimeNative::now());
    match val.update_selective(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesCompanyInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesCompanyInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/company/delete")]
pub async fn company_delete(req: web::Json<ChimesCompanyInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    match val.remove(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesCompanyInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesCompanyInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/company/delete_ids")]
pub async fn company_delete_ids(req: web::Json<Vec<i64>>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let ids = req.as_slice();
    match ChimesCompanyInfo::remove_ids(rb, ids).await {
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

#[post("/api/v1/company/search")]
pub async fn company_search(req: web::Json<ChimesCompanyInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match val.query_list(rb).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Vec<ChimesCompanyInfo>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesCompanyInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/company/paged/{current}/{size}")]
pub async fn company_paged(
    req: web::Json<ChimesCompanyInfo>,
    path_param: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let (current, size) = path_param.into_inner();
    match val.query_paged(rb, current, size).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Page<ChimesCompanyInfo>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Page<ChimesCompanyInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/company/get/{id}")]
pub async fn company_get(company_id_req: web::Path<i64>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let company_id = company_id_req.to_owned();
    match ChimesCompanyInfo::from_id(rb, &company_id).await {
        Ok(st) => match st {
            Some(tv) => {
                let ret: web::Json<ApiResult<ChimesCompanyInfo>> = web::Json(ApiResult::ok(tv));
                Ok(HttpResponse::Ok().json(ret))
            }
            None => {
                let ret: web::Json<ApiResult<ChimesCompanyInfo>> =
                    web::Json(ApiResult::error(5040, &"Not-Found".to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesCompanyInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/company/register")]
pub async fn company_register(
    req: web::Json<ChimesCompanyInfo>,
    http: HttpRequest,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    let dev = http.headers().get("DeviceId");
    if dev.is_some() {
        let deviceid = dev.unwrap();
        let device = deviceid.to_str().unwrap();
        val.register_openid = Some(device.to_string());
    }

    match val.save(rb).await {
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
