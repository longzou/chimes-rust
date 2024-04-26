use crate::entity::ChimesCompanyAccountInfo;
use crate::entity::ChimesDeptInfo;
use crate::entity::ChimesJobInfo;
use crate::entity::ChimesRoleInfo;
use crate::entity::ChimesUserInfo;
use crate::entity::ChimesUserJobInfo;
use crate::entity::ChimesUserRoleInfo;
use chimes_utils::{bool_from_str, i64_from_str};
use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_user_detail_info.rs,
 */
use std::fmt::Debug;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesUserDetailInfo {
    pub user_id: Option<i64>,
    pub dept_id: Option<i64>,
    pub username: Option<String>,
    pub nick_name: Option<String>,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub area_id: Option<String>,
    pub avatar_name: Option<String>,
    pub avatar_path: Option<String>,
    pub password: Option<String>,
    pub open_id: Option<String>,
    pub union_id: Option<String>,

    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub is_admin: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "bool_from_str")]
    pub enabled: Option<bool>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub data_scope: Option<String>,
    pub pwd_reset_time: Option<rbatis::DateTimeNative>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
    pub dept: Option<ChimesDeptInfo>,
    #[serde(default)]
    pub roles: Vec<ChimesRoleInfo>,
    #[serde(default)]
    pub jobs: Vec<ChimesJobInfo>,
    pub company_code: Option<String>,

    #[serde(default)]
    #[serde(deserialize_with = "i64_from_str")]
    pub company_id: Option<i64>,
}

impl ChimesUserDetailInfo {
    fn to_roles_vec(roles: &Option<String>) -> Vec<ChimesRoleInfo> {
        if roles.is_none() {
            vec![]
        } else {
            let role_text = roles.clone().unwrap_or_default();
            let rolevec = role_text
                .split(',')
                .map(|f| ChimesRoleInfo {
                    role_code: Some(f.to_string()),
                    ..Default::default()
                })
                .collect::<Vec<ChimesRoleInfo>>();
            rolevec
        }
    }

    #[allow(dead_code)]
    pub fn from_user(param: &ChimesUserInfo) -> Self {
        ChimesUserDetailInfo {
            user_id: param.user_id,
            dept_id: param.dept_id,
            username: param.username.clone(),
            nick_name: param.nick_name.clone(),
            gender: param.gender.clone(),
            phone: param.phone.clone(),
            email: param.email.clone(),
            area_id: param.area_id.clone(),
            avatar_name: param.avatar_name.clone(),
            avatar_path: param.avatar_path.clone(),
            password: param.password.clone(),
            open_id: param.open_id.clone(),
            union_id: param.union_id.clone(),
            is_admin: param.is_admin,
            enabled: param.enabled,
            create_by: param.create_by.clone(),
            update_by: param.update_by.clone(),
            pwd_reset_time: param.pwd_reset_time,
            create_time: param.create_time,
            update_time: param.update_time,
            company_code: param.company_code.clone(),
            company_id: param.company_id,
            data_scope: param.data_scope.clone(),
            dept: None,
            roles: Self::to_roles_vec(&param.simulate_roles.clone()),
            jobs: vec![],
        }
    }

    pub fn from_account(param: &ChimesCompanyAccountInfo) -> Self {
        ChimesUserDetailInfo {
            user_id: param.account_id,
            dept_id: None,
            username: param.username.clone(),
            nick_name: param.real_name.clone(),
            gender: param.gender.clone(),
            phone: param.telephone.clone(),
            email: param.email.clone(),
            area_id: None,
            avatar_name: None, // param.avatar_name.clone(),
            avatar_path: None, // param.avatar_path.clone(),
            password: param.sign_password.clone(),
            open_id: param.wechat_open_id.clone(),
            union_id: param.wechat_union_id.clone(),
            is_admin: Some(false),
            enabled: param.enabled,
            create_by: None,
            update_by: None,
            pwd_reset_time: None,
            data_scope: None,
            create_time: param.create_time,
            update_time: param.update_time,
            company_code: param.company_code.clone(),
            company_id: param.company_id,
            dept: None,
            roles: Self::to_roles_vec(&param.role_id.clone()),
            jobs: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn to_user(&self) -> ChimesUserInfo {
        let self_dept_id = match self.dept.clone() {
            Some(np) => np.dept_id,
            None => self.dept_id,
        };
        let roles = self
            .roles
            .clone()
            .into_iter()
            .map(|f| f.role_code.unwrap_or_default())
            .collect::<Vec<String>>()
            .join(",");
        ChimesUserInfo {
            user_id: self.user_id,
            dept_id: self_dept_id,
            username: self.username.clone(),
            nick_name: self.nick_name.clone(),
            gender: self.gender.clone(),
            phone: self.phone.clone(),
            email: self.email.clone(),
            area_id: self.area_id.clone(),
            avatar_name: self.avatar_name.clone(),
            avatar_path: self.avatar_path.clone(),
            password: self.password.clone(),
            open_id: self.open_id.clone(),
            union_id: self.union_id.clone(),
            is_admin: self.is_admin,
            enabled: self.enabled,
            create_by: self.create_by.clone(),
            update_by: self.update_by.clone(),
            data_scope: self.data_scope.clone(),
            pwd_reset_time: self.pwd_reset_time,
            create_time: self.create_time,
            update_time: self.update_time,
            company_id: self.company_id,
            company_code: self.company_code.clone(),
            simulate_roles: if roles.is_empty() { None } else { Some(roles) },
        }
    }

    #[allow(dead_code)]
    pub async fn load(rb: &Rbatis, user_id: &i64) -> Result<Option<Self>, Error> {
        match ChimesUserInfo::from_id(rb, user_id).await {
            Ok(ts) => {
                match ts {
                    Some(mp) => {
                        let mut selfmp = Self::from_user(&mp);
                        selfmp.dept =
                            match ChimesDeptInfo::from_id(rb, &selfmp.dept_id.unwrap_or_default())
                                .await
                            {
                                Ok(lst) => lst,
                                Err(_) => None,
                            };
                        let mut rb_args = vec![];
                        let sql_role = "SELECT tp.* FROM chimes_role tp INNER JOIN chimes_users_roles mt ON tp.role_id = mt.role_id WHERE mt.user_id = ?";
                        rb_args.push(
                            rbson::to_bson(selfmp.user_id.unwrap_or_default()).unwrap_or_default(),
                        );
                        selfmp.roles = match rb.fetch(sql_role, rb_args).await {
                            Ok(lst) => lst,
                            Err(_) => {
                                vec![]
                            }
                        };
                        let mut rb_args = vec![];
                        let sql_job = "SELECT tp.* FROM chimes_job tp INNER JOIN chimes_users_jobs mt ON tp.job_id = mt.job_id WHERE mt.user_id = ?";
                        rb_args.push(
                            rbson::to_bson(selfmp.user_id.unwrap_or_default()).unwrap_or_default(),
                        );
                        selfmp.jobs = match rb.fetch(sql_job, rb_args).await {
                            Ok(lst) => lst,
                            Err(_) => {
                                vec![]
                            }
                        };
                        Ok(Some(selfmp))
                    }
                    None => Ok(None),
                }
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn load_openid(rb: &Rbatis, open_id: &str) -> Result<Option<Self>, Error> {
        match ChimesUserInfo::load_openid(rb, open_id).await {
            Ok(ts) => {
                match ts {
                    Some(mp) => {
                        let mut selfmp = Self::from_user(&mp);
                        selfmp.dept =
                            match ChimesDeptInfo::from_id(rb, &selfmp.dept_id.unwrap_or_default())
                                .await
                            {
                                Ok(lst) => lst,
                                Err(_) => None,
                            };
                        let mut rb_args = vec![];
                        let sql_role = "SELECT tp.* FROM chimes_role tp INNER JOIN chimes_users_roles mt ON tp.role_id = mt.role_id WHERE mt.user_id = ?";
                        rb_args.push(
                            rbson::to_bson(selfmp.user_id.unwrap_or_default()).unwrap_or_default(),
                        );
                        selfmp.roles = match rb.fetch(sql_role, rb_args).await {
                            Ok(lst) => lst,
                            Err(_) => {
                                vec![]
                            }
                        };
                        let mut rb_args = vec![];
                        let sql_job = "SELECT tp.* FROM chimes_job tp INNER JOIN chimes_users_jobs mt ON tp.job_id = mt.job_id WHERE mt.user_id = ?";
                        rb_args.push(
                            rbson::to_bson(selfmp.user_id.unwrap_or_default()).unwrap_or_default(),
                        );
                        selfmp.jobs = match rb.fetch(sql_job, rb_args).await {
                            Ok(lst) => lst,
                            Err(_) => {
                                vec![]
                            }
                        };
                        Ok(Some(selfmp))
                    }
                    None => {
                        match ChimesCompanyAccountInfo::load_openid(rb, open_id).await {
                            Ok(ts) => {
                                match ts {
                                    Some(mt) => {
                                        let mut cds = Self::from_account(&mt);
                                        let mut codes = vec![];

                                        codes.push("ROLE_COMMONUSER".to_string());
                                        if mt.role_id.is_some() {
                                            // Should we have multi role code in this field
                                            codes.push(mt.role_id.unwrap_or_default());
                                        }

                                        match ChimesRoleInfo::query_multicode(rb, &codes).await {
                                            Ok(mlst) => {
                                                cds.roles = mlst;
                                            }
                                            Err(err) => {
                                                log::warn!("Could not fetch the roles {}", err);
                                            }
                                        }

                                        Ok(Some(cds))
                                    }
                                    None => Ok(None),
                                }
                            }
                            Err(err) => Err(err),
                        }
                    }
                }
            }
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub async fn load_username(
        rb: &Rbatis,
        username: &String,
        company_code: &Option<String>,
    ) -> Result<Option<Self>, Error> {
        if company_code.is_some() {
            // load the account info from chimes_company_account
            match ChimesCompanyAccountInfo::find_code(
                rb,
                &company_code.clone().unwrap_or_default(),
                username,
            )
            .await
            {
                Ok(ts) => {
                    match ts {
                        Some(mt) => {
                            let mut cds = Self::from_account(&mt);
                            let mut codes = vec![];

                            codes.push("ROLE_COMMONUSER".to_string());
                            if mt.role_id.is_some() {
                                // Should we have multi role code in this field
                                codes.push(mt.role_id.unwrap_or_default());
                            }

                            match ChimesRoleInfo::query_multicode(rb, &codes).await {
                                Ok(mlst) => {
                                    cds.roles = mlst;
                                }
                                Err(err) => {
                                    log::warn!("Could not fetch the roles {}", err);
                                }
                            }

                            Ok(Some(cds))
                        }
                        None => Ok(None),
                    }
                }
                Err(err) => Err(err),
            }
        } else {
            match ChimesUserInfo::load_username(rb, username).await {
                Ok(ts) => match ts {
                    Some(mp) => {
                        let mut selfmp = Self::from_user(&mp);
                        selfmp.dept =
                            match ChimesDeptInfo::from_id(rb, &selfmp.dept_id.unwrap_or_default())
                                .await
                            {
                                Ok(lst) => lst,
                                Err(_) => None,
                            };
                        let mut rb_args = vec![];
                        let sql_role = "SELECT tp.* FROM chimes_role tp INNER JOIN chimes_users_roles mt ON tp.role_id = mt.role_id WHERE mt.user_id = ?";
                        rb_args.push(
                            rbson::to_bson(selfmp.user_id.unwrap_or_default()).unwrap_or_default(),
                        );
                        selfmp.roles = match rb.fetch(sql_role, rb_args).await {
                            Ok(lst) => lst,
                            Err(_) => {
                                vec![]
                            }
                        };
                        let mut rb_args = vec![];
                        let sql_job = "SELECT tp.* FROM chimes_job tp INNER JOIN chimes_users_jobs mt ON tp.job_id = mt.job_id WHERE mt.user_id = ?";
                        rb_args.push(
                            rbson::to_bson(selfmp.user_id.unwrap_or_default()).unwrap_or_default(),
                        );
                        selfmp.jobs = match rb.fetch(sql_job, rb_args).await {
                            Ok(lst) => lst,
                            Err(_) => {
                                vec![]
                            }
                        };
                        Ok(Some(selfmp))
                    }
                    None => Ok(None),
                },
                Err(err) => Err(err),
            }
        }
    }

    #[allow(dead_code)]
    pub async fn save(&self, rb: &Rbatis) -> Result<bool, Error> {
        let mut ret: Option<Error>;
        let mut self_user = self.to_user();
        if self_user.user_id.is_none() {
            ret = match self_user.save(rb).await {
                Ok(_rs) => None,
                Err(err) => {
                    log::warn!("Save user occurred an error {}", err);
                    Some(err)
                }
            }
        } else {
            ret = match self_user.update_selective(rb).await {
                Ok(_rs) => None,
                Err(err) => {
                    log::warn!("Update user occurred an error {}", err);
                    Some(err)
                }
            }
        }
        // remove batch for ChimesUserRoleInfo.
        if ret.is_none() {
            let rm_user_role_info = ChimesUserRoleInfo {
                user_id: self_user.user_id,
                ..Default::default()
            };
            ret = match rm_user_role_info.remove_batch(rb).await {
                Ok(_) => None,
                Err(err) => {
                    log::warn!("Remove user_role_info occurred an error {}", err);
                    Some(err)
                }
            };
        }
        for row in self.roles.clone() {
            log::info!("Insert new roles: {}", row.role_id.unwrap_or_default());
            let mut svrow_user_role_info = ChimesUserRoleInfo {
                user_id: self_user.user_id,
                role_id: row.role_id,
            };
            ret = match svrow_user_role_info.save(rb).await {
                Ok(_) => None,
                Err(err) => {
                    log::warn!("Save user_role_info occurred an error {}", err);
                    Some(err)
                }
            };
        }
        // remove batch for ChimesUserJobInfo.
        if ret.is_none() {
            let rm_user_job_info = ChimesUserJobInfo {
                user_id: self_user.user_id,
                ..Default::default()
            };

            ret = match rm_user_job_info.remove_batch(rb).await {
                Ok(_) => None,
                Err(err) => {
                    log::warn!("Remove user_job_info occurred an error {}", err);
                    Some(err)
                }
            };
        }
        for row in self.jobs.clone() {
            let mut svrow_user_job_info = ChimesUserJobInfo {
                user_id: self_user.user_id,
                job_id: row.job_id,
            };
            ret = match svrow_user_job_info.save(rb).await {
                Ok(_) => None,
                Err(err) => {
                    log::warn!("Save user_job_info occurred an error {}", err);
                    Some(err)
                }
            };
        }
        match ret {
            Some(err) => Err(err),
            None => Ok(true),
        }
    }

    #[allow(dead_code)]
    pub async fn remove(&self, rb: &Rbatis) -> Result<bool, Error> {
        let mut ret: Option<Error> = None;
        // remove batch for ChimesUserRoleInfo.
        if ret.is_none() {
            let rm_user_role_info = ChimesUserRoleInfo {
                user_id: self.user_id,
                ..Default::default()
            };
            ret = match rm_user_role_info.remove_batch(rb).await {
                Ok(_rtremove) => None,
                Err(err) => {
                    log::warn!("Remove user_role_info occurred an error {}", err);
                    Some(err)
                }
            };
        }
        // remove batch for ChimesUserJobInfo.
        if ret.is_none() {
            let rm_user_job_info = ChimesUserJobInfo {
                user_id: self.user_id,
                ..Default::default()
            };
            ret = match rm_user_job_info.remove_batch(rb).await {
                Ok(_rtremove) => None,
                Err(err) => {
                    log::warn!("Remove user_job_info occurred an error {}", err);
                    Some(err)
                }
            };
        }

        if let Some(ret) = ret {
            Err(ret)
        } else {
            match self.to_user().remove(rb).await {
                Ok(_rs) => Ok(true),
                Err(err) => {
                    log::warn!("Remove user occurred an error {}", err);
                    Err(err)
                }
            }
        }
    }
}
