use std::pin::Pin;

use crate::entity::ChimesUserDetailInfo;
use crate::query::{QueryUserRole, QueryUserRoleParams};
use actix_web::{HttpRequest, Result};
use futures::Future;
use serde::Deserialize;

use crate::entity::ChimesUserInfo;
use crate::query::{QueryUserPermission, QueryUserPermissionParams};
use actix_utils::future::{err, ok};
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpMessage};
use chimes_auth::{ChimesAuthService, ChimesAuthUser};
use chimes_utils::get_rbatis;

use super::UserClaims;

#[derive(Clone, Default, Deserialize)]
pub struct SystemUser<ChimesUserInfo> {
    pub user: ChimesUserInfo,
    pub roles: Vec<String>,
    pub open_id: Option<String>,
    pub union_id: Option<String>,
}

impl SystemUser<ChimesUserInfo> {
    #[allow(dead_code)]
    pub fn has_role(&self, role: &String) -> bool {
        self.roles.contains(role)
    }
}

impl ChimesAuthUser<SystemUser<ChimesUserInfo>> for SystemUser<ChimesUserInfo> {
    #[allow(dead_code)]
    fn get_user_name(&self) -> String {
        self.user.username.clone().unwrap_or_default()
    }

    #[allow(dead_code)]
    fn get_creditial(&self) -> String {
        self.user.phone.clone().unwrap_or_default()
    }

    #[allow(dead_code)]
    fn to_detail(self) -> Self {
        self
    }
}

impl FromRequest for SystemUser<ChimesUserInfo> {
    type Error = actix_web::Error;
    type Future = actix_utils::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let header = req.headers();

        match req.extensions().get::<SystemUser<ChimesUserInfo>>() {
            Some(user) => {
                let mut us = user.clone();
                if header.contains_key("AttachedOpenId") {
                    us.open_id = header
                        .get("AttachedOpenId")
                        .map(|f| f.to_str().ok().map(|g| g.to_string()).unwrap_or_default());
                    us.union_id = header
                        .get("AttachedUnionId")
                        .map(|f| f.to_str().ok().map(|g| g.to_string()).unwrap_or_default());
                }
                ok(us)
            }
            None => {
                log::info!("Not found the SystemUser");
                err(actix_web::error::ErrorBadRequest("ups..."))
            }
        }
    }
}

#[derive(Clone)]
pub struct ChimesUserAuthService<SystemUser> {
    #[allow(unused)]
    pub system_user: Option<SystemUser>,
}

impl ChimesAuthService<SystemUser<ChimesUserInfo>>
    for ChimesUserAuthService<SystemUser<ChimesUserInfo>>
{
    type Future = Pin<Box<dyn Future<Output = Option<SystemUser<ChimesUserInfo>>>>>;

    fn permit(
        &self,
        ust: &Option<SystemUser<ChimesUserInfo>>,
        req_method: &String,
        url_pattern: &String,
    ) -> Self::Future {
        let up = url_pattern.clone();
        let mth = req_method.clone();
        let mtusr = ust.clone();
        let username = match ust {
            Some(st) => st.get_user_name(),
            None => "ANNOYMOUS".to_string(),
        };

        Box::pin(async move {
            let param = QueryUserPermissionParams {
                api_method: mth.clone(),
                api_pattern: up.clone(),
                username: username.clone(),
            };

            let rb = get_rbatis();
            match QueryUserPermission::query(rb, &param).await {
                Ok(rs) => {
                    if !rs.is_empty() {
                        let rp = rs[0].clone();
                        if rp.api_bypass == Some("ANONYMOUS".to_string())
                            || rp.api_bypass == Some("anonymous".to_string())
                        {
                            // log::info!("Found Permission {}, the api_pass is {}", rp.api_pattern.clone().unwrap_or_default(), rp.api_bypass.clone().unwrap_or_default());
                            Some(SystemUser::default())
                        } else if rp.api_bypass == Some("USER".to_string())
                            || rp.api_bypass == Some("user".to_string())
                        {
                            if mtusr.is_none() {
                                return None;
                            } else {
                                return mtusr;
                            }
                        } else {
                            let mut find_username = None;
                            for rx in rs.clone() {
                                if rx.username.is_some() {
                                    find_username = rx.username.clone();
                                    break;
                                }
                            }
                            if find_username.is_none() {
                                return None;
                            } else {
                                return mtusr;
                            }
                        }
                    } else if up.is_empty() {
                        mtusr
                    } else {
                        None
                    }
                }
                Err(err) => {
                    log::warn!(
                        "Query the permission for user with an error: {}",
                        err.to_string()
                    );
                    None
                }
            }
        })
    }

    fn authenticate(&self, token: &String) -> Self::Future {
        let rb = get_rbatis();
        let tk = token.clone();
        Box::pin(async move {
            match UserClaims::decode(&tk) {
                Some(uc) => match ChimesUserInfo::load_username(rb, &uc.aud).await {
                    Ok(r) => match r {
                        Some(u) => {
                            let param = QueryUserRoleParams {
                                user_id: u.user_id,
                                username: u.username.clone(),
                                company_code: u.company_code.clone(),
                                role_codes: u.parse_simulate_roles(),
                            };

                            let roles: Vec<String> = match QueryUserRole::query(rb, &param).await {
                                Ok(rs) => rs
                                    .into_iter()
                                    .map(|f| f.role_code.unwrap_or_default())
                                    .collect(),
                                Err(_) => {
                                    vec![]
                                }
                            };
                            Some(SystemUser {
                                user: u,
                                roles,
                                open_id: None,
                                union_id: None,
                            })
                        }
                        None => None,
                    },
                    Err(_) => None,
                },
                None => None,
            }
        })
    }

    fn nojwt_authenticate(&self, token: &String) -> Self::Future {
        let rb = get_rbatis();
        let tk = token.clone();
        Box::pin(async move {
            match ChimesUserInfo::load_username(rb, &tk.clone()).await {
                Ok(r) => match r {
                    Some(u) => {
                        let param = QueryUserRoleParams {
                            user_id: u.user_id,
                            username: u.username.clone(),
                            company_code: u.company_code.clone(),
                            role_codes: u.parse_simulate_roles(),
                        };

                        let roles = match QueryUserRole::query(rb, &param).await {
                            Ok(rs) => rs
                                .into_iter()
                                .map(|f| f.role_code.unwrap_or_default())
                                .collect(),
                            Err(_) => {
                                vec![]
                            }
                        };
                        Some(SystemUser {
                            user: u,
                            roles,
                            open_id: None,
                            union_id: None,
                        })
                    }
                    None => {
                        log::warn!("For NoJWT mode, to load by openid with: {}, ", tk.clone());
                        match ChimesUserDetailInfo::load_openid(rb, &tk.clone()).await {
                            Ok(cdinfo) => {
                                if cdinfo.is_none() {
                                    None
                                } else {
                                    let cdu = cdinfo.unwrap();
                                    let mu = cdu.to_user();
                                    let roles = cdu
                                        .roles
                                        .into_iter()
                                        .map(|f| f.role_code.unwrap_or_default())
                                        .collect::<Vec<String>>();
                                    Some(SystemUser {
                                        user: mu.clone(),
                                        roles,
                                        open_id: mu.open_id.clone(),
                                        union_id: mu.union_id.clone(),
                                    })
                                }
                            }
                            Err(_err) => None,
                        }
                    }
                },
                Err(_) => None,
            }
        })
    }
}
