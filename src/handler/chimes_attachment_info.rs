use crate::entity::ChimesAttachmentRefInfo;
use crate::entity::{ChimesAttachmentInfo, ChimesUserInfo};
use crate::utils::SystemUser;
use actix_web::{web, HttpResponse, Result};
use awmp::Error;
use chimes_auth::{ApiResult, ChimesAuthUser};
use chimes_utils::{
    calc_file_hash, file_size_format, get_local_timestamp, get_rbatis, set_file_permission,
    AppConfig,
};
/**
 * Generate the file for chimes_attachment_info.rs,
 */
use rbatis::{snowflake, Page};
use std::fs::create_dir_all;
use std::path::Path;

#[post("/api/v1/attachment/create")]
pub async fn attachment_save(
    su: SystemUser<ChimesUserInfo>,
    req: web::Json<ChimesAttachmentInfo>,
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
                    let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                        web::Json(ApiResult::ok(val));
                    Ok(HttpResponse::Ok().json(ret))
                }
                Err(err) => {
                    let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                        web::Json(ApiResult::error(5010, &err.to_string()));
                    Ok(HttpResponse::Ok().json(ret))
                }
            },
            Err(err) => {
                let _ = tx.rollback().await.is_ok();
                let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                    web::Json(ApiResult::error(5010, &err.to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/attachment/update")]
async fn attachment_update(
    su: SystemUser<ChimesUserInfo>,
    req: web::Json<ChimesAttachmentInfo>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.modify_by = su.user.username.clone();
    val.update_time = Some(rbatis::DateTimeNative::now());
    match rb.acquire_begin().await {
        Ok(mut tx) => match val.update_selective(&mut tx).await {
            Ok(_st) => match tx.commit().await {
                Ok(_) => {
                    let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                        web::Json(ApiResult::ok(val));
                    Ok(HttpResponse::Ok().json(ret))
                }
                Err(err) => {
                    let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                        web::Json(ApiResult::error(5010, &err.to_string()));
                    Ok(HttpResponse::Ok().json(ret))
                }
            },
            Err(err) => {
                let _ = tx.rollback().await.is_ok();
                let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                    web::Json(ApiResult::error(5010, &err.to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/attachment/delete")]
pub async fn attachment_delete(
    su: SystemUser<ChimesUserInfo>,
    req: web::Json<ChimesAttachmentInfo>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.modify_by = su.user.username.clone();
    val.update_time = Some(rbatis::DateTimeNative::now());
    match rb.acquire_begin().await {
        Ok(mut tx) => match val.remove(&mut tx).await {
            Ok(_st) => {
                let ids = [val.attachment_id.unwrap_or_default()];
                match ChimesAttachmentRefInfo::remove_attachment_ids(&mut tx, &ids).await {
                    Ok(_) => match tx.commit().await {
                        Ok(_) => {
                            let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                                web::Json(ApiResult::ok(val));
                            Ok(HttpResponse::Ok().json(ret))
                        }
                        Err(err) => {
                            let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                                web::Json(ApiResult::error(5010, &err.to_string()));
                            Ok(HttpResponse::Ok().json(ret))
                        }
                    },
                    Err(err) => {
                        let _ = tx.rollback().await.is_ok();
                        let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                            web::Json(ApiResult::error(5010, &err.to_string()));
                        Ok(HttpResponse::Ok().json(ret))
                    }
                }
            }
            Err(err) => {
                let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                    web::Json(ApiResult::error(5010, &err.to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/attachment/delete_ids")]
pub async fn attachment_delete_ids(
    _su: SystemUser<ChimesUserInfo>,
    req: web::Json<Vec<i64>>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let ids = req.as_slice();
    match rb.acquire_begin().await {
        Ok(mut tx) => match ChimesAttachmentInfo::remove_ids(&mut tx, ids).await {
            Ok(st) => match ChimesAttachmentRefInfo::remove_attachment_ids(&mut tx, ids).await {
                Ok(_) => match tx.commit().await {
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

#[post("/api/v1/attachment/search")]
pub async fn attachment_search(
    _su: SystemUser<ChimesUserInfo>,
    req: web::Json<ChimesAttachmentInfo>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match val.query_list(rb).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Vec<ChimesAttachmentInfo>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesAttachmentInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/attachment/query")]
pub async fn attachment_query(
    _su: SystemUser<ChimesUserInfo>,
    req: web::Json<ChimesAttachmentRefInfo>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match ChimesAttachmentInfo::find_attachments(
        rb,
        &val.business_name.clone().unwrap_or_default(),
        &val.business_id.clone(),
    )
    .await
    {
        Ok(st) => {
            let ret: web::Json<ApiResult<Vec<ChimesAttachmentInfo>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesAttachmentInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/attachment/paged/{current}/{size}")]
pub async fn attachment_paged(
    _su: SystemUser<ChimesUserInfo>,
    req: web::Json<ChimesAttachmentInfo>,
    path_param: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let (current, size) = path_param.into_inner();
    match val.query_paged(rb, current, size).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Page<ChimesAttachmentInfo>>> =
                web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Page<ChimesAttachmentInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/attachment/get/{id}")]
pub async fn attachment_get(
    _su: SystemUser<ChimesUserInfo>,
    attachment_id_req: web::Path<i64>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let attachment_id = attachment_id_req.to_owned();
    match ChimesAttachmentInfo::from_id(rb, &attachment_id).await {
        Ok(st) => match st {
            Some(tv) => {
                let ret: web::Json<ApiResult<ChimesAttachmentInfo>> = web::Json(ApiResult::ok(tv));
                Ok(HttpResponse::Ok().json(ret))
            }
            None => {
                let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                    web::Json(ApiResult::error(5040, &"Not-Found".to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

/**
 * Upload the Image and generated a new Avatar Url
 * But the avatar url was not saved into the user's information directly.
 */
#[post("/api/v1/resource/common/upload/{type}")]
async fn resource_common_upload_file(
    su: SystemUser<ChimesUserInfo>,
    pathreq: web::Path<String>,
    mut parts: awmp::Parts,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let qs = parts.texts.to_query_string();
    let username = su.get_user_name();
    let pathspc = pathreq.to_string();
    let store_path = AppConfig::get()
        .lock()
        .unwrap()
        .webserver_conf
        .upload_store_path
        .clone();
    let base_url = AppConfig::get()
        .lock()
        .unwrap()
        .webserver_conf
        .access_url_prefix
        .clone();

    log::info!(
        "The request {},, {},  stored in: {}",
        qs,
        username.clone(),
        store_path.clone()
    );
    let milpath = format!("{}/{}/", store_path, pathspc.clone());
    let _ = create_dir_all(milpath).is_ok();

    let (file_org, file_parts, relative_path, file_ext, len) = parts
        .files
        .take("file")
        .pop()
        .and_then(|f| {
            let stamp = get_local_timestamp();
            let forg = f.sanitized_file_name().to_string();
            let fext = Path::new(f.sanitized_file_name())
                .extension()
                .unwrap()
                .to_str()
                .unwrap_or_default()
                .to_string();

            let filename_part = format!("{:x}_{}.{}", stamp, snowflake::new_snowflake_id(), fext);
            let mfcp = format!("{}/{}/{}", store_path, pathspc, filename_part.clone());
            let rel_fcp = format!("{}/{}", pathspc, filename_part.clone());
            //let fcp = format!("{}/t_{:x}_{}.{}", store_path, stamp, snowflake::new_snowflake_id(), fext);
            //log::info!("Stamp: {}, fext: {}, fcp: {}", stamp, fext.clone(), fcp.clone());

            f.persist_at(mfcp.clone())
                .map(|fx| {
                    set_file_permission(&mfcp, 0o644);
                    let len = fx.metadata().unwrap().len();
                    (
                        forg.clone(),
                        mfcp.clone(),
                        rel_fcp.clone(),
                        fext.clone(),
                        len,
                    )
                })
                .map_err(|op| {
                    log::warn!("With Error {}", op);
                    Error::TempFilePersistError
                })
                .ok()
        })
        .unwrap_or_default();

    log::info!("Stored file: {}", file_parts.clone());

    let filepathfull = file_parts.clone();
    let fextion = Path::new(&filepathfull)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap_or_default();
    let mut valmut = ChimesAttachmentInfo {
        storage_path: Some(relative_path.clone()),
        attachment_name: Some(fextion.to_string()),
        // valmut. Some(file_org.to_string());
        // valmut.attach_hash = Some(fextion.to_string());
        original_name: Some(file_org.clone()),
        file_ext: Some(file_ext),
        file_size: Some(len as i64),
        file_size_display: Some(file_size_format(len as usize)),
        content_type: Some(pathspc.clone()),
        // valmut.access_url;
        attach_hash: calc_file_hash(file_parts.clone().as_str()),
        access_url: Some(format!("{}/{}/{}", base_url.clone(), pathspc, fextion)),
        modify_by: su.user.username.clone(),
        create_time: Some(rbatis::DateTimeNative::now()),
        update_time: Some(rbatis::DateTimeNative::now()),
        ..Default::default()
    };

    match rb.acquire_begin().await {
        Ok(mut tx) => match valmut.save(&mut tx).await {
            Ok(_) => {
                let _ = tx.commit().await.is_ok();
                let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                    web::Json(ApiResult::ok(valmut));
                Ok(HttpResponse::Ok().json(ret))
            }
            Err(err) => {
                let _ = tx.rollback().await.is_ok();
                let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                    web::Json(ApiResult::error(5100, &err.to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesAttachmentInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/resource/common/batchupload/{type}")]
async fn resource_common_upload_multi_files(
    su: SystemUser<ChimesUserInfo>,
    pathreq: web::Path<String>,
    mut parts: awmp::Parts,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let qs = parts.texts.to_query_string();
    let username = su.get_user_name();
    let pathspc = pathreq.to_string();
    let store_path = AppConfig::get()
        .lock()
        .unwrap()
        .webserver_conf
        .upload_store_path
        .clone();
    let base_url = AppConfig::get()
        .lock()
        .unwrap()
        .webserver_conf
        .access_url_prefix
        .clone();

    log::info!(
        "The request {},, {},  stored in: {}",
        qs,
        username.clone(),
        store_path.clone()
    );
    let milpath = format!("{}/{}/", store_path, pathspc.clone());
    let _ = create_dir_all(milpath).is_ok();

    let attaches = parts
        .files
        .take("file")
        .into_iter()
        .map(|f| {
            let stamp = get_local_timestamp();
            let forg = f.sanitized_file_name().to_string();
            let fext = Path::new(f.sanitized_file_name())
                .extension()
                .unwrap()
                .to_str()
                .unwrap_or_default()
                .to_string();

            let filename_part = format!("{:x}_{}.{}", stamp, snowflake::new_snowflake_id(), fext);
            let mfcp = format!("{}/{}/{}", store_path, pathspc, filename_part.clone());
            let rel_fcp = format!("{}/{}", pathspc, filename_part.clone());

            f.persist_at(mfcp.clone())
                .map(|fx| {
                    set_file_permission(&mfcp, 0o644);
                    let len = fx.metadata().unwrap().len();
                    let mut valmut = ChimesAttachmentInfo::default();

                    let fextion = filename_part.clone();

                    valmut.storage_path = Some(rel_fcp.clone());
                    valmut.attachment_name = Some(fextion.clone());

                    // valmut. Some(file_org.to_string());
                    // valmut.attach_hash = Some(fextion.to_string());
                    valmut.original_name = Some(forg.clone());
                    valmut.file_ext = Some(fext.clone());
                    valmut.file_size = Some(len as i64);
                    valmut.file_size_display = Some(file_size_format(len as usize));

                    valmut.content_type = Some(pathspc.clone());
                    // valmut.access_url;
                    valmut.attach_hash = calc_file_hash(mfcp.clone().as_str());
                    valmut.access_url = Some(format!(
                        "{}/{}/{}",
                        base_url.clone(),
                        pathspc,
                        fextion.clone()
                    ));

                    valmut.modify_by = su.user.username.clone();
                    valmut.create_time = Some(rbatis::DateTimeNative::now());
                    valmut.update_time = Some(rbatis::DateTimeNative::now());

                    // (forg.clone(), mfcp.clone(), rel_fcp.clone(), fext.clone(), len)
                    valmut
                })
                .map_err(|op| {
                    log::warn!("With Error {}", op);
                    Error::TempFilePersistError
                })
                .ok()
        })
        .collect::<Vec<Option<ChimesAttachmentInfo>>>();

    // log::info!("Stored file: {}", file_parts.clone());

    // let filepathfull = file_parts.clone();
    // let fextion = Path::new(&filepathfull).file_name().unwrap().to_str().unwrap_or_default();

    match rb.acquire_begin().await {
        Ok(mut tx) => {
            let mut upload_files = vec![];
            for mut val in attaches.into_iter().flatten() {
                match val.save(&mut tx).await {
                    Ok(_) => {
                        upload_files.push(val.clone());
                    }
                    Err(err) => {
                        log::info!("save the attachmenent with error {}", err);
                    }
                }
            }
            let _ = tx.commit().await.is_ok();

            let ret: web::Json<ApiResult<Vec<ChimesAttachmentInfo>>> =
                web::Json(ApiResult::ok(upload_files));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesAttachmentInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}
