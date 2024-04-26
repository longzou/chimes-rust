use std::collections::HashMap;

use crate::query::QueryUserMenusParams;
/**
 * Generate the file for query_user.rs,
 */
use crate::{
    entity::{ChimesUserDetailInfo, ChimesUserInfo},
    query::QueryUserMenus,
    utils::{generate_rand_string, get_local_timestamp, SystemUser, UserClaims},
};
use actix_web::{web, HttpRequest, HttpResponse, Result};
use base64::{engine::general_purpose, Engine as _};
use chimes_auth::{ApiResult, ChimesAuthUser};
use chimes_utils::{
    get_rbatis, global_app_data_get, global_app_data_insert, global_app_data_remove,
    rsa_decrypt_by_private_key, rsa_encrypt_by_public_key,
};

use serde::{Deserialize, Serialize};

#[get("/api/v1/auth/code")]
pub async fn auth_code(_req: HttpRequest) -> Result<HttpResponse> {
    let png = captcha::Captcha::new()
        .add_chars(5)
        .apply_filter(captcha::filters::Noise::new(0.4))
        .view(180, 80)
        .as_tuple();
    match png {
        Some(st) => {
            let basestr = general_purpose::STANDARD_NO_PAD.encode(st.1);
            let keyid = generate_rand_string(18);
            global_app_data_insert(&keyid.clone(), &st.0);
            let ret: web::Json<ApiResult<String>> =
                web::Json(ApiResult::new(200, &keyid, basestr, get_local_timestamp()));
            Ok(HttpResponse::Ok().json(ret))
        }
        None => {
            let ret: web::Json<ApiResult<String>> =
                web::Json(ApiResult::error(5010, &"FAILED".to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserAuth {
    pub company_code: Option<String>,
    pub username: String,
    pub password: String,
    pub verify_code_key: String,
    pub verify_code: String,
    pub open_id: Option<String>,
    pub force_change: Option<bool>,
    pub remember: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserResponse {
    pub token: String,
    pub username: String,
    pub company_code: Option<String>,
    pub authorities: Vec<HashMap<String, String>>,
    pub detail: Option<ChimesUserDetailInfo>,
    pub roles: Vec<String>,
}

#[post("/api/v1/auth/login")]
pub async fn auth_login(req: web::Json<UserAuth>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let auth = req.to_owned();

    let codeval = global_app_data_get(&auth.verify_code_key);
    if codeval.is_none() {
        let ret: web::Json<ApiResult<String>> =
            web::Json(ApiResult::error(5301, &"VERIFY-CODE-KEY".to_string()));
        return Ok(HttpResponse::Ok().json(ret));
    }

    global_app_data_remove(&auth.verify_code_key);

    log::info!(
        "CompanyCode: {}, Username: {}",
        auth.company_code.clone().unwrap_or_default(),
        auth.username.clone()
    );

    if codeval.unwrap().to_lowercase() != auth.verify_code.to_lowercase() {
        let ret: web::Json<ApiResult<String>> =
            web::Json(ApiResult::error(5301, &"VERIFY-CODE".to_string()));
        return Ok(HttpResponse::Ok().json(ret));
    }

    match ChimesUserDetailInfo::load_username(rb, &auth.username, &auth.company_code).await {
        Ok(st) => {
            match st {
                Some(us) => {
                    let decodepassword = rsa_decrypt_by_private_key(&auth.password);
                    if decodepassword.is_some() {
                        let pwd =
                            rsa_encrypt_by_public_key(&decodepassword.clone().unwrap_or_default());
                        log::warn!("Re-encrypt password: {}", pwd.unwrap_or_default());
                    }
                    let md5password = match us.password.clone() {
                        Some(t) => {
                            // let digist = md5::compute(t);
                            rsa_decrypt_by_private_key(&t)
                            // Some(format!("{:x}", digist))
                        }
                        None => None,
                    };

                    if decodepassword != md5password {
                        // may need to encode the password
                        let ret: web::Json<ApiResult<String>> =
                            web::Json(ApiResult::error(5302, &"PASSWORD".to_string()));
                        Ok(HttpResponse::Ok().json(ret))
                    } else {
                        if auth.open_id.is_some() {
                            if let Ok(mt) = ChimesUserDetailInfo::load_openid(
                                rb,
                                &auth.open_id.clone().unwrap_or_default(),
                            )
                            .await
                            {
                                if let Some(ut) = mt {
                                    if ut.user_id != us.user_id && auth.force_change != Some(true) {
                                        let ret: web::Json<ApiResult<String>> = web::Json(
                                            ApiResult::error(5601, &"NEED-CONFIRM".to_string()),
                                        );
                                        return Ok(HttpResponse::Ok().json(ret));
                                    }
                                    if auth.force_change == Some(true)
                                        && auth.remember == Some(true)
                                    {
                                        let mut cusinfo = ut.to_user();
                                        cusinfo.open_id = None;
                                        // cusinfo.union_id = None;
                                        if auth.company_code.is_some() {
                                            let cusacc = cusinfo.to_account();
                                            let _ = cusacc.update_openid(rb).await.is_ok();
                                        } else {
                                            let _ = cusinfo.update(rb).await.is_ok();
                                        }
                                        let mut xusinfo = us.to_user();
                                        xusinfo.open_id = auth.open_id.clone();
                                        // xusinfo.union_id = auth.union_id.clone();
                                        if auth.company_code.is_some() {
                                            let xusacc = xusinfo.to_account();
                                            let _ = xusacc.update_openid(rb).await.is_ok();
                                        } else {
                                            let _ = xusinfo.update(rb).await.is_ok();
                                        }
                                    }
                                } else if auth.remember == Some(true) {
                                    let mut xusinfo = us.to_user();
                                    xusinfo.open_id = auth.open_id.clone();
                                    // xusinfo.union_id = auth.union_id.clone();
                                    if auth.company_code.is_some() {
                                        let xusacc = xusinfo.to_account();
                                        let _ = xusacc.update_openid(rb).await.is_ok();
                                    } else {
                                        let _ = xusinfo.update(rb).await.is_ok();
                                    }
                                }
                            }
                        }

                        let usn = if auth.company_code.is_some() {
                            auth.company_code.clone().unwrap_or_default()
                                + "$$"
                                + us.username.clone().unwrap().as_str()
                        } else {
                            us.username.clone().unwrap()
                        };

                        let claim = UserClaims {
                            aud: usn,
                            sub: format!("{}", us.user_id.unwrap_or_default()),
                            exp: get_local_timestamp() as usize,
                        };
                        match claim.encode() {
                            Some(token) => {
                                let mut usc = us.clone();
                                usc.password = None;
                                let username = usc.username.clone().unwrap();
                                let mut hs = HashMap::new();
                                hs.insert("authority".to_string(), username.clone());
                                let mut roles = vec![];
                                roles.push(username.clone());
                                for rl in usc.roles.clone() {
                                    roles.push(rl.role_code.unwrap_or_default());
                                }
                                let umparam = QueryUserMenusParams {
                                    username: username.clone(),
                                    role_codes: roles.clone(),
                                };

                                if let Ok(rls) = QueryUserMenus::query(rb, &umparam).await {
                                    rls.into_iter().for_each(|f| {
                                        if f.permission.is_some() {
                                            roles.push(f.permission.unwrap_or_default());
                                        }
                                    });
                                };
                                let up = UserResponse {
                                    username: username.clone(),
                                    company_code: auth.company_code.clone(),
                                    authorities: vec![hs],
                                    token,
                                    roles,
                                    detail: Some(usc),
                                };

                                let ret: web::Json<ApiResult<UserResponse>> =
                                    web::Json(ApiResult::ok(up));
                                Ok(HttpResponse::Ok().json(ret))
                            }
                            None => {
                                let ret: web::Json<ApiResult<UserResponse>> = web::Json(
                                    ApiResult::error(5404, &"TOKEN was not generated.".to_string()),
                                );
                                Ok(HttpResponse::Ok().json(ret))
                            }
                        }
                    }
                }
                None => {
                    let ret: web::Json<ApiResult<UserResponse>> = web::Json(ApiResult::error(
                        5404,
                        &format!("{} is not found.", auth.username),
                    ));
                    Ok(HttpResponse::Ok().json(ret))
                }
            }
        }
        Err(err) => {
            let ret: web::Json<ApiResult<UserResponse>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}

#[post("/api/v1/auth/logout")]
pub async fn auth_logout() -> Result<HttpResponse> {
    let ret: web::Json<ApiResult<UserResponse>> =
        web::Json(ApiResult::error(200, &"OK".to_string()));
    Ok(HttpResponse::Ok().json(ret))
}

#[get("/api/v1/healthcheck")]
pub async fn healthcheck() -> Result<HttpResponse> {
    let ret: web::Json<ApiResult<String>> = web::Json(ApiResult::ok("Iamliving".to_string()));
    Ok(HttpResponse::Ok().json(ret))
}

#[get("/api/v1/auth/info")]
pub async fn auth_info(su: SystemUser<ChimesUserInfo>) -> Result<HttpResponse> {
    let rb = get_rbatis();
    let param = su.get_user_name();
    let company_code = su.user.company_code.clone();

    match ChimesUserDetailInfo::load_username(rb, &param, &company_code.clone()).await {
        Ok(st) => match st {
            Some(us) => {
                let mut mutus = us;
                mutus.password = None;

                let claim = UserClaims {
                    aud: mutus.username.clone().unwrap(),
                    sub: format!("{}", mutus.user_id.unwrap_or_default()),
                    exp: get_local_timestamp() as usize,
                };
                match claim.encode() {
                    Some(token) => {
                        let username = mutus.username.clone().unwrap();
                        let mut hs = HashMap::new();
                        hs.insert("authority".to_string(), username.clone());

                        let mut roles = vec![];
                        roles.push(username.clone());
                        for rl in mutus.roles.clone() {
                            roles.push(rl.role_code.unwrap_or_default());
                        }
                        let umparam = QueryUserMenusParams {
                            username: username.clone(),
                            role_codes: roles.clone(),
                        };

                        if let Ok(rls) = QueryUserMenus::query(rb, &umparam).await {
                            rls.into_iter().for_each(|f| {
                                if f.permission.is_some() {
                                    roles.push(f.permission.unwrap_or_default());
                                }
                            });
                        };

                        let up = UserResponse {
                            username: username.clone(),
                            company_code: company_code.clone(),
                            authorities: vec![hs],
                            token,
                            roles,
                            detail: Some(mutus),
                        };
                        let ret: web::Json<ApiResult<UserResponse>> = web::Json(ApiResult::ok(up));
                        Ok(HttpResponse::Ok().json(ret))
                    }
                    None => {
                        let ret: web::Json<ApiResult<UserResponse>> = web::Json(ApiResult::error(
                            5404,
                            &"TOKEN was not generated.".to_string(),
                        ));
                        Ok(HttpResponse::Ok().json(ret))
                    }
                }
            }
            None => {
                let ret: web::Json<ApiResult<UserResponse>> =
                    web::Json(ApiResult::error(5404, &format!("{} is not found.", param)));
                Ok(HttpResponse::Ok().json(ret))
            }
        },
        Err(err) => {
            let ret: web::Json<ApiResult<UserResponse>> =
                web::Json(ApiResult::error(5010, &err.to_string()));
            Ok(HttpResponse::Ok().json(ret))
        }
    }
}
