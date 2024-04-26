/**
 * Generate the file for chimes_dept_info.rs,
 */
use rbatis::{DateTimeNative, Page};

use crate::entity::ChimesHolidayInfo;
use actix_web::{web, HttpResponse, Result};
use chimes_auth::ApiResult;
use chimes_utils::get_rbatis;

#[post("/api/v1/holiday/save")]
pub async fn holiday_save(req: web::Json<ChimesHolidayInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();

    match val.save(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesHolidayInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesHolidayInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/holiday/update")]
async fn holiday_update(req: web::Json<ChimesHolidayInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let mut tval = if val.id.is_none() {
        if val.holiday_date.is_none() {
            let ret: web::Json<ApiResult<ChimesHolidayInfo>> =
                web::Json(ApiResult::error(5010, &"holiday_date is null".to_string()));
            return Ok(HttpResponse::Ok().json(ret));
        } else {
            match val.find_holiday_info(rb).await {
                Ok(ts) => {
                    if ts.is_none() {
                        val.clone()
                    } else {
                        let mut xts = ts.unwrap();
                        xts.date_type = val.date_type;
                        xts.remark = val.remark.clone();
                        xts
                    }
                }
                Err(err) => {
                    let ret: web::Json<ApiResult<ChimesHolidayInfo>> =
                        web::Json(ApiResult::error(5010, &err.to_string()));
                    return Ok(HttpResponse::Ok().json(ret));
                }
            }
        }
    } else {
        val.clone()
    };
    if tval.create_date.is_none() {
        tval.create_date = Some(DateTimeNative::now());
    }

    tval.update_date = Some(DateTimeNative::now());

    if tval.id.is_none() {
        match tval.save(rb).await {
            Ok(_st) => {
                let ret: web::Json<ApiResult<ChimesHolidayInfo>> = web::Json(ApiResult::ok(val));
                Ok(HttpResponse::Ok().json(ret))
            }
            Err(err) => {
                let ret: web::Json<ApiResult<ChimesHolidayInfo>> =
                    web::Json(ApiResult::error(5010, &err.to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        }
    } else {
        match tval.update_selective(rb).await {
            Ok(_st) => {
                let ret: web::Json<ApiResult<ChimesHolidayInfo>> = web::Json(ApiResult::ok(val));
                Ok(HttpResponse::Ok().json(ret))
            }
            Err(err) => {
                let ret: web::Json<ApiResult<ChimesHolidayInfo>> =
                    web::Json(ApiResult::error(5010, &err.to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        }
    }
}

#[post("/api/v1/holiday/delete")]
pub async fn holiday_delete(req: web::Json<ChimesHolidayInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    match val.remove(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesHolidayInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesHolidayInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/holiday/delete_ids")]
pub async fn holiday_delete_ids(req: web::Json<Vec<i64>>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let ids = req.as_slice();
    match ChimesHolidayInfo::remove_ids(rb, ids).await {
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

#[post("/api/v1/holiday/delete_physical_year")]
pub async fn holiday_delete_phy_year(req: web::Json<i64>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let fy = req.to_owned();
    match ChimesHolidayInfo::remove_physical_year(rb, fy).await {
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

#[post("/api/v1/holiday/search")]
pub async fn holiday_search(req: web::Json<ChimesHolidayInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match val.query_list(rb).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Vec<ChimesHolidayInfo>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesHolidayInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/holiday/paged/{current}/{size}")]
pub async fn holiday_paged(
    req: web::Json<ChimesHolidayInfo>,
    path_param: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let (current, size) = path_param.into_inner();
    match val.query_paged(rb, current, size).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Page<ChimesHolidayInfo>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Page<ChimesHolidayInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/holiday/get/{id}")]
pub async fn holiday_get(dept_id_req: web::Path<i64>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let dept_id = dept_id_req.to_owned();
    match ChimesHolidayInfo::from_id(rb, &dept_id).await {
        Ok(st) => match st {
            Some(tv) => {
                let ret: web::Json<ApiResult<ChimesHolidayInfo>> = web::Json(ApiResult::ok(tv));
                Ok(HttpResponse::Ok().json(ret))
            }
            None => {
                let ret: web::Json<ApiResult<ChimesHolidayInfo>> =
                    web::Json(ApiResult::error(5040, &"Not-Found".to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesHolidayInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}
