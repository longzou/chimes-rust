use crate::entity::{BaseAreaInfo, BaseAreaInfoValue};
use actix_web::HttpRequest;
use actix_web::{web, HttpResponse, Result};
use chimes_auth::ApiResult;
use chimes_utils::get_rbatis;
/**
 * Generate the file for base_area_info.rs,
 */
use rbatis::Page;

#[post("/api/v1/base_area/save")]
pub async fn base_area_save(req: web::Json<BaseAreaInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.create_ms = Some(rbatis::DateTimeNative::now().and_utc().timestamp());
    val.update_ms = Some(rbatis::DateTimeNative::now().and_utc().timestamp());
    match val.save(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<BaseAreaInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<BaseAreaInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/base_area/update")]
async fn base_area_update(req: web::Json<BaseAreaInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.update_ms = Some(rbatis::DateTimeNative::now().and_utc().timestamp());
    match val.update_selective(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<BaseAreaInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<BaseAreaInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/base_area/delete")]
pub async fn base_area_delete(req: web::Json<BaseAreaInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    match val.remove(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<BaseAreaInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<BaseAreaInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/base_area/delete_ids")]
pub async fn base_area_delete_ids(req: web::Json<Vec<String>>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let ids = req.as_slice();
    match BaseAreaInfo::remove_ids(rb, ids).await {
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

#[post("/api/v1/base_area/search")]
pub async fn base_area_search(req: web::Json<BaseAreaInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match val.query_list(rb).await {
        Ok(st) => {
            let mtts: Vec<BaseAreaInfoValue> = st
                .into_iter()
                .map(|f| BaseAreaInfoValue::from_entity_with(&f, true, &[]))
                .collect();
            let ret: web::Json<ApiResult<Vec<BaseAreaInfoValue>>> = web::Json(ApiResult::ok(mtts));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<BaseAreaInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/base_area/paged/{current}/{size}")]
pub async fn base_area_paged(
    req: web::Json<BaseAreaInfo>,
    path_param: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let (current, size) = path_param.into_inner();
    match val.query_paged(rb, current, size).await {
        Ok(st) => {
            let mtts: Vec<BaseAreaInfoValue> = st
                .records
                .into_iter()
                .map(|f| BaseAreaInfoValue::from_entity_with(&f, true, &[]))
                .collect();
            let mut newpage = Page::new_total(st.page_no, st.page_size, st.total);
            newpage.records = mtts;
            let ret: web::Json<ApiResult<Page<BaseAreaInfoValue>>> =
                web::Json(ApiResult::ok(newpage));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Page<BaseAreaInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/base_area/get/{id}")]
pub async fn base_area_get(id_req: web::Path<String>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let id = id_req.to_owned();
    match BaseAreaInfo::from_id(rb, &id).await {
        Ok(st) => match st {
            Some(tv) => {
                let ret: web::Json<ApiResult<BaseAreaInfo>> = web::Json(ApiResult::ok(tv));
                Ok(HttpResponse::Ok().json(ret))
            }
            None => {
                let ret: web::Json<ApiResult<BaseAreaInfo>> =
                    web::Json(ApiResult::error(5040, &"Not-Found".to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<BaseAreaInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/base_area/tree")]
pub async fn base_area_tree(req: HttpRequest) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let query = req.query_string();
    let dic = crate::utils::parse_query(query);
    let val = crate::utils::get_hash_value(&dic, "pid");
    let valopt = if val.is_empty() { None } else { Some(val) };
    match BaseAreaInfo::query_tree(rb, &valopt).await {
        Ok(st) => {
            let mtts: Vec<BaseAreaInfoValue> = st
                .into_iter()
                .map(|f| BaseAreaInfoValue::from_entity_with(&f, true, &[]))
                .collect();
            let ret: web::Json<ApiResult<Vec<BaseAreaInfoValue>>> = web::Json(ApiResult::ok(mtts));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<BaseAreaInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

/**
 * 根据指定的parent id，来获得其上级目录
 */
#[post("/api/v1/base_area/superior")]
pub async fn base_area_superior(pids: web::Json<Vec<String>>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    // let query = req.query_string();
    // let dic = crate::utils::parse_query(query);
    // let val = crate::utils::get_hash_value(&dic, "pid");
    let valopt = Some(pids[0].clone());
    // let mut depts = vec![];
    let ch = BaseAreaInfo {
        pcode: valopt,
        ..Default::default()
    };
    match ch.query_pcode(rb).await {
        Ok(st) => {
            let mut valst = st
                .into_iter()
                .map(|f| BaseAreaInfoValue::from_entity(&f))
                .collect();
            let mtts: Vec<BaseAreaInfoValue> =
                BaseAreaInfoValue::build_parent_tree(&mut valst, rb).await;
            let ret: web::Json<ApiResult<Vec<BaseAreaInfoValue>>> = web::Json(ApiResult::ok(mtts));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<BaseAreaInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

/**
 * 根据指定的根ID，构建查询指定的级数
 */
#[get("/api/v1/base_area/normaltree/{root}/{level}")]
pub async fn base_area_normaltree(path_param: web::Path<(String, i32)>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    // let query = req.query_string();
    // let dic = crate::utils::parse_query(query);
    // let val = crate::utils::get_hash_value(&dic, "pid");
    let (root_id, level) = path_param.to_owned();

    let _valopt = Some(root_id);
    // let mut depts = vec![];
    match BaseAreaInfo::query_by_level(rb, &Some(level)).await {
        Ok(st) => {
            let mut valst = st
                .into_iter()
                .map(|f| BaseAreaInfoValue::from_entity(&f))
                .collect::<Vec<BaseAreaInfoValue>>();
            let mtts: Vec<BaseAreaInfoValue> =
                BaseAreaInfoValue::build_normal_tree(&mut valst, level);
            let ret: web::Json<ApiResult<Vec<BaseAreaInfoValue>>> = web::Json(ApiResult::ok(mtts));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<BaseAreaInfoValue>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}
