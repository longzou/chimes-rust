use crate::entity::{
    ChimesAttachmentInfo, ChimesUserDetailInfo, ChimesUserInfo, UserResetEmailRequest,
    UserUpdateEmailRequest, UserUpdateInfoRequest, UserUpdatePasswordRequest,
};
use crate::utils::SystemUser;
use actix_web::http::header::{
    Charset, ContentDisposition, DispositionParam, DispositionType, ExtendedValue,
};
use actix_web::{web, HttpResponse, Result};
use awmp::Error;
use chimes_auth::{get_local_timestamp, ApiResult, ChimesAuthUser};
use chimes_utils::get_rbatis;
use chimes_utils::{
    generate_rand_string, get_email_queue, rsa_decrypt_by_private_key, set_file_permission,
    AppConfig, EmailBody,
};
use chimes_utils::{global_app_data_get, global_app_data_insert, global_app_data_remove};
use rbatis::{snowflake, Page};
use rsa::pkcs8::der::Encodable;
use std::fs::OpenOptions;
use std::io::Read;
use std::path::Path;

#[post("/api/v1/user/save")]
pub async fn user_save(req: web::Json<ChimesUserDetailInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.create_time = Some(rbatis::DateTimeNative::now());
    val.update_time = Some(rbatis::DateTimeNative::now());
    match val.save(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesUserDetailInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesUserDetailInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/user/update")]
async fn user_update(req: web::Json<ChimesUserDetailInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    val.update_time = Some(rbatis::DateTimeNative::now());
    match val.save(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesUserDetailInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesUserDetailInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/user/delete")]
pub async fn user_delete(req: web::Json<ChimesUserInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let mut val = req.to_owned();
    match val.remove(rb).await {
        Ok(_st) => {
            let ret: web::Json<ApiResult<ChimesUserInfo>> = web::Json(ApiResult::ok(val));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesUserInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/user/delete_ids")]
pub async fn user_delete_ids(req: web::Json<Vec<i64>>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let ids = req.as_slice();
    match ChimesUserInfo::remove_ids(rb, ids).await {
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

#[post("/api/v1/user/search")]
pub async fn user_search(req: web::Json<ChimesUserInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    match val.query_list(rb).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Vec<ChimesUserInfo>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Vec<ChimesUserInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/user/paged/{current}/{size}")]
pub async fn user_paged(
    req: web::Json<ChimesUserInfo>,
    path_param: web::Path<(u64, u64)>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let (current, size) = path_param.into_inner();
    match val.query_paged(rb, current, size).await {
        Ok(st) => {
            let ret: web::Json<ApiResult<Page<ChimesUserInfo>>> = web::Json(ApiResult::ok(st));
            Ok(HttpResponse::Ok().json(ret))
        }
        Err(err) => {
            let ret: web::Json<ApiResult<Page<ChimesUserInfo>>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[get("/api/v1/user/get/{id}")]
pub async fn user_get(user_id_req: web::Path<i64>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let user_id = user_id_req.to_owned();
    log::info!("Try to load the user's detail by {}", user_id.clone());
    match ChimesUserDetailInfo::load(rb, &user_id).await {
        Ok(st) => match st {
            Some(tv) => {
                let ret: web::Json<ApiResult<ChimesUserDetailInfo>> = web::Json(ApiResult::ok(tv));
                Ok(HttpResponse::Ok().json(ret))
            }
            None => {
                let ret: web::Json<ApiResult<ChimesUserDetailInfo>> =
                    web::Json(ApiResult::error(5040, &"Not-Found".to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesUserDetailInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/user/center/update/info")]
async fn user_center_update(
    su: SystemUser<ChimesUserInfo>,
    req: web::Json<UserUpdateInfoRequest>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let username = su.get_user_name();
    match ChimesUserInfo::load_username(rb, &username).await {
        Ok(st) => match st {
            Some(sval) => {
                let mut valmut = sval.clone();
                valmut.nick_name = Some(val.nick_name);
                valmut.gender = Some(val.gender);
                valmut.phone = Some(val.phone);
                valmut.update_time = Some(rbatis::DateTimeNative::now());
                match valmut.update_selective(rb).await {
                    Ok(_) => {
                        let ret: web::Json<ApiResult<ChimesUserInfo>> =
                            web::Json(ApiResult::ok(valmut));
                        Ok(HttpResponse::Ok().json(ret))
                    }
                    Err(err) => {
                        let ret: web::Json<ApiResult<ChimesUserInfo>> =
                            web::Json(ApiResult::error(5010, &err.to_string()));
                        Ok(HttpResponse::Ok().json(ret))
                    }
                }
            }
            None => {
                let ret: web::Json<ApiResult<ChimesUserInfo>> =
                    web::Json(ApiResult::error(5404, &"NOT-Found".to_string()));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesUserInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

/**
 * Given the user's information, and provider a new email-address
 * The system will send an email into the email-address
 * In this email, a email verify-code was beloned into.
 */
#[post("/api/v1/user/center/reset/email")]
pub async fn user_reset_email(
    _su: SystemUser<ChimesUserInfo>,
    req: web::Json<UserResetEmailRequest>,
) -> Result<HttpResponse> {
    let keyid = generate_rand_string(18);
    let code = generate_rand_string(6);
    global_app_data_insert(&keyid.clone(), &code.clone());

    let emb = EmailBody {
        html_email: true,
        subject: "重置用户帐户邮箱".to_string(),
        content: format!(
            "<div>请在修改邮箱的界面中提供下如下代码：</div><div><span>{}</span></div>",
            code.clone()
        )
        .to_string(),
        mine_email: "system@qq.com".to_string(),
        email_receiver: req.email.clone(),
    };
    get_email_queue().queue_send(&emb).await;
    let dat = UserResetEmailRequest {
        email: req.email.clone(),
        codekey: keyid.clone(),
    };
    let ret: web::Json<ApiResult<UserResetEmailRequest>> = web::Json(ApiResult::ok(dat));
    Ok(HttpResponse::Ok().json(ret))
}

#[post("/api/v1/user/center/update/email")]
async fn user_center_update_email(
    su: SystemUser<ChimesUserInfo>,
    req: web::Json<UserUpdateEmailRequest>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let username = su.get_user_name();
    match ChimesUserInfo::load_username(rb, &username).await {
        Ok(st) => {
            match st {
                Some(sval) => {
                    let mut valmut = sval.clone();
                    valmut.email = Some(val.email);
                    // should check the code and the password

                    let stcode = global_app_data_get(&val.codekey);

                    let codeeq = if let Some(stcode) = stcode {
                        val.code == stcode
                    } else {
                        false
                    };

                    global_app_data_remove(&val.codekey);

                    if codeeq {
                        // check the password
                        let decodepassword = rsa_decrypt_by_private_key(&val.password);
                        let mctdcdpasswd =
                            rsa_decrypt_by_private_key(&sval.password.clone().unwrap_or_default());
                        if decodepassword == mctdcdpasswd {
                            match valmut.update_selective(rb).await {
                                Ok(_) => {
                                    let ret: web::Json<ApiResult<ChimesUserInfo>> =
                                        web::Json(ApiResult::ok(valmut));
                                    Ok(HttpResponse::Ok().json(ret))
                                }
                                Err(err) => {
                                    let ret: web::Json<ApiResult<ChimesUserInfo>> =
                                        web::Json(ApiResult::error(5010, &err.to_string()));
                                    Ok(HttpResponse::Ok().json(ret))
                                }
                            }
                        } else {
                            let ret: web::Json<ApiResult<ChimesUserInfo>> =
                                web::Json(ApiResult::error(5051, &"Password-Error".to_string()));
                            Ok(HttpResponse::Ok().json(ret))
                        }
                    } else {
                        let ret: web::Json<ApiResult<ChimesUserInfo>> =
                            web::Json(ApiResult::error(5051, &"Code-Error".to_string()));
                        Ok(HttpResponse::Ok().json(ret))
                    }
                }
                None => {
                    let ret: web::Json<ApiResult<ChimesUserInfo>> =
                        web::Json(ApiResult::error(5404, &"NOT-Found".to_string()));
                    Ok(HttpResponse::Ok().json(ret))
                }
            }
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesUserInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/user/center/update/pwd")]
async fn user_center_update_pwd(
    su: SystemUser<ChimesUserInfo>,
    req: web::Json<UserUpdatePasswordRequest>,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let val = req.to_owned();
    let username = su.get_user_name();
    match ChimesUserInfo::load_username(rb, &username).await {
        Ok(st) => {
            match st {
                Some(sval) => {
                    let decodepassword = rsa_decrypt_by_private_key(&val.old_pwd);
                    let mctdcdpasswd =
                        rsa_decrypt_by_private_key(&sval.password.clone().unwrap_or_default());

                    if decodepassword == mctdcdpasswd {
                        let mut valmut = sval.clone();
                        valmut.password = Some(val.new_pwd);
                        valmut.update_time = Some(rbatis::DateTimeNative::now());
                        // should check the code and the password
                        match valmut.update_selective(rb).await {
                            Ok(_) => {
                                let ret: web::Json<ApiResult<ChimesUserInfo>> =
                                    web::Json(ApiResult::ok(valmut));
                                Ok(HttpResponse::Ok().json(ret))
                            }
                            Err(err) => {
                                let ret: web::Json<ApiResult<ChimesUserInfo>> =
                                    web::Json(ApiResult::error(5010, &err.to_string()));
                                Ok(HttpResponse::Ok().json(ret))
                            }
                        }
                    } else {
                        let ret: web::Json<ApiResult<ChimesUserInfo>> =
                            web::Json(ApiResult::error(5051, &"Old password error".to_string()));
                        Ok(HttpResponse::Ok().json(ret))
                    }
                }
                None => {
                    let ret: web::Json<ApiResult<ChimesUserInfo>> =
                        web::Json(ApiResult::error(5404, &"NOT-Found".to_string()));
                    Ok(HttpResponse::Ok().json(ret))
                }
            }
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesUserInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/user/center/update/avatar")]
async fn user_center_update_avatar(
    su: SystemUser<ChimesUserInfo>,
    mut parts: awmp::Parts,
) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let qs = parts.texts.to_query_string();
    let username = su.get_user_name();
    let store_path = AppConfig::get()
        .lock()
        .unwrap()
        .webserver_conf
        .upload_store_path
        .clone();

    log::info!(
        "The request {},, {},  stored in: {}",
        qs,
        username.clone(),
        store_path.clone()
    );

    let file_parts = parts
        .files
        .take("avatar")
        .pop()
        .and_then(|f| {
            let stamp = get_local_timestamp();
            let fext = Path::new(f.sanitized_file_name())
                .extension()
                .unwrap()
                .to_str()
                .unwrap_or_default();
            let fcp = format!(
                "{}/{:x}_{}.{}",
                store_path,
                stamp,
                snowflake::new_snowflake_id(),
                fext
            );

            f.into_inner()
                .persist(fcp.clone())
                .map(|_| {
                    set_file_permission(&fcp, 0o644);
                    fcp.clone()
                })
                .map_err(|op| {
                    log::warn!("With Error {}", op.error);
                    Error::TempFilePersistError
                })
                .ok()
        })
        .map(|f| f.to_string())
        .unwrap_or_default();

    log::info!("Stored file: {}", file_parts.clone());

    let filepathfull = file_parts.clone();
    let fextion = Path::new(&filepathfull)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap_or_default();

    match ChimesUserInfo::load_username(rb, &username).await {
        Ok(st) => {
            match st {
                Some(sval) => {
                    let mut valmut = sval.clone();
                    valmut.avatar_path = Some(file_parts.clone());
                    valmut.avatar_name = Some(fextion.to_string());
                    valmut.update_time = Some(rbatis::DateTimeNative::now());
                    // should check the code and the password
                    match valmut.update_selective(rb).await {
                        Ok(_) => {
                            let ret: web::Json<ApiResult<ChimesUserInfo>> =
                                web::Json(ApiResult::ok(valmut));
                            Ok(HttpResponse::Ok().json(ret))
                        }
                        Err(err) => {
                            let ret: web::Json<ApiResult<ChimesUserInfo>> =
                                web::Json(ApiResult::error(5010, &err.to_string()));
                            Ok(HttpResponse::Ok().json(ret))
                        }
                    }
                }
                None => {
                    let ret: web::Json<ApiResult<ChimesUserInfo>> =
                        web::Json(ApiResult::error(5404, &"NOT-Found".to_string()));
                    Ok(HttpResponse::Ok().json(ret))
                }
            }
        }
        Err(err) => {
            let ret: web::Json<ApiResult<ChimesUserInfo>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

/**
 * Upload the Avatar and generated a new Avatar Url
 * But the avatar url was not saved into the user's information directly.
 */
#[post("/api/v1/user/common/avatar/upload")]
async fn user_common_update_avatar(
    su: SystemUser<ChimesUserInfo>,
    mut parts: awmp::Parts,
) -> Result<HttpResponse> {
    let _rb = get_rbatis();
    let qs = parts.texts.to_query_string();
    let username = su.get_user_name();
    let store_path = AppConfig::get()
        .lock()
        .unwrap()
        .webserver_conf
        .upload_store_path
        .clone();

    log::info!(
        "The request {},, {},  stored in: {}",
        qs,
        username.clone(),
        store_path.clone()
    );

    let file_parts = parts
        .files
        .take("avatar")
        .pop()
        .and_then(|f| {
            let stamp = get_local_timestamp();
            let fext = Path::new(f.sanitized_file_name())
                .extension()
                .unwrap()
                .to_str()
                .unwrap_or_default();
            let mfcp = format!(
                "{}/{:x}_{}.{}",
                store_path,
                stamp,
                snowflake::new_snowflake_id(),
                fext
            );
            let fcp = format!(
                "{}/t_{:x}_{}.{}",
                store_path,
                stamp,
                snowflake::new_snowflake_id(),
                fext
            );
            log::info!("Stamp: {}, fext: {}, fcp: {}", stamp, fext, fcp.clone());

            f.persist_at(mfcp.clone())
                .map(|_f| {
                    set_file_permission(&mfcp, 0o644);
                    mfcp.clone()
                })
                .map_err(|op| {
                    log::warn!("With Error {}", op);
                    Error::TempFilePersistError
                })
                .ok()
        })
        .map(|f| f.to_string())
        .unwrap_or_default();

    log::info!("Stored file: {}", file_parts.clone());

    let filepathfull = file_parts.clone();
    let fextion = Path::new(&filepathfull)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap_or_default();
    let val = ChimesUserInfo {
        avatar_path: Some(file_parts.clone()),
        avatar_name: Some(fextion.to_string()),
        update_time: Some(rbatis::DateTimeNative::now()),
        ..Default::default()
    };
    // should check the code and the password
    let ret: web::Json<ApiResult<ChimesUserInfo>> = web::Json(ApiResult::ok(val));
    Ok(HttpResponse::Ok().json(ret))
}

#[get("/api/v1/avatar/{filename}")]
async fn show_avatar(filename: web::Path<String>) -> Result<HttpResponse> {
    // let rb = get_rbatis();
    let qs = filename.clone().to_string();
    let store_path = AppConfig::get()
        .lock()
        .unwrap()
        .webserver_conf
        .upload_store_path
        .clone();

    let fullpath = format!("{}/{}", store_path, qs);

    let _filepath = Path::new(&fullpath.clone());
    log::info!("Filepath: {}", fullpath.clone());

    match OpenOptions::new().read(true).open(fullpath) {
        Ok(fl) => {
            let mut mutfile = fl;
            let cd1 = ContentDisposition {
                disposition: DispositionType::Inline,
                parameters: vec![DispositionParam::FilenameExt(ExtendedValue {
                    charset: Charset::Iso_8859_1, // The character set for the bytes of the filename
                    language_tag: None, // The optional language tag (see `language-tag` crate)
                    value: qs.to_vec().ok().unwrap(), // the actual bytes of the filename
                })],
            };
            let mut buf = vec![];
            match mutfile.read_to_end(&mut buf) {
                Ok(_us) => Ok(HttpResponse::Ok()
                    .insert_header((actix_web::http::header::CONTENT_DISPOSITION, cd1))
                    .body(buf)),
                Err(_err) => Ok(HttpResponse::NotFound().body("Not found")),
            }
        }
        Err(_) => Ok(HttpResponse::NotFound().body("Not found")),
    }
}

#[get("/api/v1/avatar/{type}/{filename}")]
async fn show_type_avatar(filename: web::Path<(String, String)>) -> Result<HttpResponse> {
    // let rb = get_rbatis();
    let (typ, qs) = filename.clone();
    //let qs = filename.clone().to_string();
    let store_path = AppConfig::get()
        .lock()
        .unwrap()
        .webserver_conf
        .upload_store_path
        .clone();

    let fullpath = format!("{}/{}/{}", store_path, typ, qs);

    let _filepath = Path::new(&fullpath.clone());
    log::info!("Filepath: {}", fullpath.clone());

    match OpenOptions::new().read(true).open(fullpath) {
        Ok(fl) => {
            let mut mutfile = fl;
            let cd1 = ContentDisposition {
                disposition: DispositionType::Inline,
                parameters: vec![DispositionParam::FilenameExt(ExtendedValue {
                    charset: Charset::Iso_8859_1, // The character set for the bytes of the filename
                    language_tag: None, // The optional language tag (see `language-tag` crate)
                    value: qs.to_vec().ok().unwrap(), // the actual bytes of the filename
                })],
            };
            let mut buf = vec![];
            match mutfile.read_to_end(&mut buf) {
                Ok(_us) => Ok(HttpResponse::Ok()
                    .insert_header((actix_web::http::header::CONTENT_DISPOSITION, cd1))
                    .body(buf)),
                Err(_err) => Ok(HttpResponse::NotFound().body("Not found")),
            }
        }
        Err(_) => Ok(HttpResponse::NotFound().body("Not found")),
    }
}

#[get("/api/v1/original/{type}/{file_id}")]
async fn show_org_file_content(filename: web::Path<(String, String)>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let (typ, qs) = filename.clone();
    //let qs = filename.clone().to_string();
    let store_path = AppConfig::get()
        .lock()
        .unwrap()
        .webserver_conf
        .upload_store_path
        .clone();

    // let _filepath = Path::new(&fullpath.clone());
    //log::info!("Filepath: {}", fullpath.clone());

    let filech = match ChimesAttachmentInfo::from_attachment_name(rb, &qs.clone()).await {
        Ok(ch) => {
            if let Some(ch) = ch {
                ch
            } else {
                let mcid = match qs.parse::<i64>() {
                    Ok(id) => id,
                    Err(_err) => {
                        return Ok(HttpResponse::NotFound().body("Not-Found3"));
                    }
                };
                match ChimesAttachmentInfo::from_id(rb, &mcid).await {
                    Ok(cht) => {
                        if let Some(cht) = cht {
                            cht
                        } else {
                            return Ok(HttpResponse::NotFound().body("Not-Found4"));
                        }
                    }
                    Err(_err) => {
                        return Ok(HttpResponse::InternalServerError()
                            .body("Internal Error to find the resources"));
                    }
                }
            }
        }
        Err(_err) => {
            return Ok(
                HttpResponse::InternalServerError().body("Internal Error to find the resources")
            );
        }
    };

    let fullpath = match filech.storage_path {
        Some(t) => {
            if t.starts_with('/') || t.find(':') == Some(1usize) {
                t
            } else {
                format!("{}/{}", store_path, t)
            }
        }
        None => {
            format!("{}/{}/{}", store_path, typ, qs)
        }
    };

    log::info!("Filepath: {}", fullpath.clone());

    match OpenOptions::new().read(true).open(fullpath) {
        Ok(fl) => {
            let mut mutfile = fl;
            let cd1 = ContentDisposition {
                disposition: DispositionType::Inline,
                parameters: vec![DispositionParam::FilenameExt(ExtendedValue {
                    charset: Charset::Iso_8859_1, // The character set for the bytes of the filename
                    language_tag: None, // The optional language tag (see `language-tag` crate)
                    value: qs.to_vec().ok().unwrap(), // the actual bytes of the filename
                })],
            };
            let mut buf = vec![];
            match mutfile.read_to_end(&mut buf) {
                Ok(_us) => Ok(HttpResponse::Ok()
                    .insert_header((actix_web::http::header::CONTENT_DISPOSITION, cd1))
                    .body(buf)),
                Err(_err) => Ok(HttpResponse::NotFound().body("Not found5")),
            }
        }
        Err(_) => Ok(HttpResponse::NotFound().body("Not found6")),
    }
}

/**
 * Upload the Image and generated a new Avatar Url
 * But the avatar url was not saved into the user's information directly.
 */
#[post("/api/v1/resource/upload/image")]
async fn resource_common_upload_image(
    su: SystemUser<ChimesUserInfo>,
    mut parts: awmp::Parts,
) -> Result<HttpResponse> {
    let _rb = get_rbatis();
    let qs = parts.texts.to_query_string();
    let username = su.get_user_name();
    let store_path = AppConfig::get()
        .lock()
        .unwrap()
        .webserver_conf
        .upload_store_path
        .clone();

    log::info!(
        "The request {},, {},  stored in: {}",
        qs,
        username.clone(),
        store_path.clone()
    );

    let file_parts = parts
        .files
        .take("file")
        .pop()
        .and_then(|f| {
            let stamp = get_local_timestamp();
            let fext = Path::new(f.sanitized_file_name())
                .extension()
                .unwrap()
                .to_str()
                .unwrap_or_default();
            let mfcp = format!(
                "{}/{:x}_{}.{}",
                store_path,
                stamp,
                snowflake::new_snowflake_id(),
                fext
            );
            let fcp = format!(
                "{}/t_{:x}_{}.{}",
                store_path,
                stamp,
                snowflake::new_snowflake_id(),
                fext
            );
            log::info!("Stamp: {}, fext: {}, fcp: {}", stamp, fext, fcp.clone());

            f.persist_at(mfcp.clone())
                .map(|_f| {
                    set_file_permission(&mfcp, 0o644);
                    mfcp.clone()
                })
                .map_err(|op| {
                    log::warn!("With Error {}", op);
                    Error::TempFilePersistError
                })
                .ok()
        })
        .map(|f| f.to_string())
        .unwrap_or_default();

    log::info!("Stored file: {}", file_parts.clone());

    let filepathfull = file_parts.clone();
    let fextion = Path::new(&filepathfull)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap_or_default();
    let valmut = ChimesUserInfo {
        avatar_path: Some(file_parts.clone()),
        avatar_name: Some(fextion.to_string()),
        ..Default::default()
    };
    // should check the code and the password
    let ret: web::Json<ApiResult<ChimesUserInfo>> = web::Json(ApiResult::ok(valmut));
    Ok(HttpResponse::Ok().json(ret))
}
