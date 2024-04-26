use actix_web::{web, HttpResponse, Result};
use chimes_auth::ApiResult;
use chimes_utils::ChimesPerformanceInfo;

#[get("/api/v1/performance/get")]
pub async fn performance_get() -> Result<HttpResponse> {
    match ChimesPerformanceInfo::get_performance_info() {
        Ok(st) => {
            let ret: web::Json<ApiResult<ChimesPerformanceInfo>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesPerformanceInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}
