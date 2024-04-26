use rbatis::error::Error;
use rbatis::rbatis::Rbatis;
use serde_derive::{Deserialize, Serialize};
/**
 * Generate the file for chimes_user_roles.rs,
 */
use std::fmt::Debug;

use crate::entity::ChimeProfileInfo;
use crate::entity::ChimesRoleInfo;
use crate::entity::ChimesUserInfo;
use crate::entity::ChimesUserRoleInfo;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChimesUserRoles {
    pub user_id: Option<i64>,
    pub dept_id: Option<i64>,
    pub company_id: Option<i64>,
    pub company_code: Option<String>,
    pub username: Option<String>,
    pub nick_name: Option<String>,
    pub gender: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub area_id: Option<String>,
    pub avatar_name: Option<String>,
    pub avatar_path: Option<String>,
    pub password: Option<String>,
    pub is_admin: Option<bool>,
    pub open_id: Option<String>,
    pub union_id: Option<String>,
    pub enabled: Option<bool>,
    pub create_by: Option<String>,
    pub update_by: Option<String>,
    pub data_scope: Option<String>,
    pub pwd_reset_time: Option<rbatis::DateTimeNative>,
    pub create_time: Option<rbatis::DateTimeNative>,
    pub update_time: Option<rbatis::DateTimeNative>,
    #[serde(rename(deserialize = "user_id"))]
    pub profile: Option<ChimeProfileInfo>,
    #[serde(rename(deserialize = "role_id"))]
    pub roles: Vec<ChimesRoleInfo>,
}

impl ChimesUserRoles {
    #[allow(dead_code)]
    pub fn from_user(param: &ChimesUserInfo) -> Self {
        ChimesUserRoles {
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
            is_admin: param.is_admin,
            enabled: param.enabled,
            open_id: param.open_id.clone(),
            union_id: param.union_id.clone(),
            create_by: param.create_by.clone(),
            update_by: param.update_by.clone(),
            data_scope: param.data_scope.clone(),
            pwd_reset_time: param.pwd_reset_time,
            create_time: param.create_time,
            update_time: param.update_time,
            company_code: param.company_code.clone(),
            company_id: param.company_id,
            profile: None,
            roles: vec![],
        }
    }

    #[allow(dead_code)]
    pub fn to_user(&self) -> ChimesUserInfo {
        let roles = self
            .roles
            .clone()
            .into_iter()
            .map(|f| f.role_code.unwrap_or_default())
            .collect::<Vec<String>>()
            .join(",");
        ChimesUserInfo {
            user_id: self.user_id,
            dept_id: self.dept_id,
            username: self.username.clone(),
            nick_name: self.nick_name.clone(),
            gender: self.gender.clone(),
            phone: self.phone.clone(),
            email: self.email.clone(),
            area_id: self.area_id.clone(),
            avatar_name: self.avatar_name.clone(),
            avatar_path: self.avatar_path.clone(),
            password: self.password.clone(),
            is_admin: self.is_admin,
            open_id: self.open_id.clone(),
            union_id: self.union_id.clone(),
            data_scope: self.data_scope.clone(),
            company_code: self.company_code.clone(),
            company_id: self.company_id,
            enabled: self.enabled,
            create_by: self.create_by.clone(),
            update_by: self.update_by.clone(),
            pwd_reset_time: self.pwd_reset_time,
            create_time: self.create_time,
            update_time: self.update_time,
            simulate_roles: if roles.is_empty() { None } else { Some(roles) },
        }
    }

    #[allow(dead_code)]
    pub async fn find_user_by_role(
        rb: &Rbatis,
        role_name: &str,
    ) -> Result<Vec<ChimesUserInfo>, Error> {
        let mut sql = "select cu.* from chimes_user cu inner join chimes_users_roles cur on cur.user_id = cu.user_id inner join chimes_role cr on cr.role_id = cur.role_id ".to_string();
        let mut rb_args = vec![];
        if role_name.contains(',') {
            let role_names = role_name
                .split(',')
                .map(|f| f.trim().to_string())
                .collect::<Vec<String>>();
            sql.push_str(" where cr.role_code in (");
            for rn in role_names {
                sql.push_str("?,");
                rb_args.push(rbson::to_bson(&rn.clone()).unwrap_or_default());
            }
            if sql.ends_with(',') {
                sql.remove(sql.len() - 1);
            }
            sql.push(')');
        } else {
            sql.push_str(" where cr.role_code = ?");
            rb_args.push(rbson::to_bson(role_name.to_owned()).unwrap_or_default());
        }
        rb.fetch::<Vec<ChimesUserInfo>>(&sql, rb_args).await
    }

    #[allow(dead_code)]
    pub async fn load(rb: &Rbatis, user_id: &i64) -> Result<Option<Self>, Error> {
        match ChimesUserInfo::from_id(rb, user_id).await {
            Ok(ts) => match ts {
                Some(mp) => {
                    let mut selfmp = Self::from_user(&mp);
                    let tmp_profile = ChimeProfileInfo {
                        user_id: selfmp.user_id,
                        ..Default::default()
                    };
                    selfmp.profile = match tmp_profile.query_list(rb).await {
                        Ok(lst) => {
                            if !lst.is_empty() {
                                Some(lst[0].clone())
                            } else {
                                None
                            }
                        }
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
                    Ok(Some(selfmp))
                }
                None => Ok(None),
            },
            Err(err) => Err(err),
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
            ret = match self_user.update(rb).await {
                Ok(_rs) => None,
                Err(err) => {
                    log::warn!("Update user occurred an error {}", err);
                    Some(err)
                }
            }
        }
        if ret.is_none() {
            ret = match self.profile.clone() {
                Some(tp) => {
                    let mut mtp = tp.clone();
                    mtp.user_id = self_user.user_id;
                    if mtp.user_id.is_none() {
                        match mtp.save(rb).await {
                            Ok(_mtpsave) => None,
                            Err(err) => {
                                log::warn!("Save profile occurred an error {}", err);
                                Some(err)
                            }
                        }
                    } else {
                        match mtp.update(rb).await {
                            Ok(_mtpsave) => None,
                            Err(err) => {
                                log::warn!("Save profile occurred an error {}", err);
                                Some(err)
                            }
                        }
                    }
                }
                None => None,
            };
        }
        // remove batch for ChimesUserRoleInfo.
        if ret.is_none() {
            let rm_user_role_info = ChimesUserRoleInfo {
                role_id: self.user_id,
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
            let mut svrow_user_role_info = ChimesUserRoleInfo {
                user_id: self.user_id,
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
        match ret {
            Some(err) => Err(err),
            None => Ok(true),
        }
    }

    #[allow(dead_code)]
    pub async fn remove(&self, rb: &Rbatis) -> Result<bool, Error> {
        let mut ret: Option<Error> = None;
        if ret.is_none() {
            ret = match self.profile.clone() {
                Some(tp) => {
                    let mut mtp = tp.clone();
                    match mtp.remove(rb).await {
                        Ok(_rtremove) => None,
                        Err(err) => {
                            log::warn!("Remove profile occurred an error {}", err);
                            Some(err)
                        }
                    }
                }
                None => None,
            };
        }
        // remove batch for ChimesUserRoleInfo.
        if ret.is_none() {
            let rm_user_role_info = ChimesUserRoleInfo {
                role_id: self.user_id,
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
