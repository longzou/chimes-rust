use actix_web::dev::{ServiceFactory, ServiceRequest};
use actix_web::{error::Error, App};

pub trait AppEntryCollect {
    fn show_all_entry(self) -> Self;

    fn register_handlers(self) -> Self;
}

impl<T> AppEntryCollect for App<T>
where
    T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
{
    fn show_all_entry(self) -> Self {
        self
    }

    fn register_handlers(self) -> Self {
        self.service(crate::handler::query_user_permission_paged)
            .service(crate::handler::performance_get)
            .service(crate::handler::healthcheck)
            .service(crate::handler::query_user_permission_query)
            .service(crate::handler::user_save)
            .service(crate::handler::user_update)
            .service(crate::handler::user_delete)
            .service(crate::handler::user_search)
            .service(crate::handler::query_user_permission_paged)
            .service(crate::handler::query_user_permission_query)
            .service(crate::handler::user_save)
            .service(crate::handler::user_update)
            .service(crate::handler::user_delete_ids)
            .service(crate::handler::user_search)
            .service(crate::handler::user_paged)
            .service(crate::handler::user_get)
            .service(crate::handler::menu_save)
            .service(crate::handler::menu_update)
            .service(crate::handler::menu_delete)
            .service(crate::handler::menu_delete_ids)
            .service(crate::handler::menu_search)
            .service(crate::handler::menu_children)
            .service(crate::handler::menu_paged)
            .service(crate::handler::menu_get)
            .service(crate::handler::menu_superior)
            .service(crate::handler::permission_save)
            .service(crate::handler::permission_update)
            .service(crate::handler::permission_delete)
            .service(crate::handler::permission_delete_ids)
            .service(crate::handler::permission_search)
            .service(crate::handler::permission_paged)
            .service(crate::handler::permission_get)
            .service(crate::handler::permission_children)
            .service(crate::handler::role_save)
            .service(crate::handler::role_update)
            .service(crate::handler::role_delete)
            .service(crate::handler::role_delete_ids)
            .service(crate::handler::role_search)
            .service(crate::handler::role_paged)
            .service(crate::handler::role_get)
            .service(crate::handler::role_level)
            .service(crate::handler::role_menus_rel_save)
            .service(crate::handler::role_menus_rel_remove_ids)
            .service(crate::handler::dept_save)
            .service(crate::handler::dept_update)
            .service(crate::handler::dept_delete)
            .service(crate::handler::dept_delete_ids)
            .service(crate::handler::dept_search)
            .service(crate::handler::dept_paged)
            .service(crate::handler::dept_get)
            .service(crate::handler::dept_tree)
            .service(crate::handler::dept_superior)
            .service(crate::handler::job_save)
            .service(crate::handler::job_update)
            .service(crate::handler::job_delete)
            .service(crate::handler::job_delete_ids)
            .service(crate::handler::job_search)
            .service(crate::handler::job_paged)
            .service(crate::handler::job_get)
            .service(crate::handler::datascope_save)
            .service(crate::handler::datascope_update)
            .service(crate::handler::datascope_delete)
            .service(crate::handler::datascope_delete_ids)
            .service(crate::handler::datascope_search)
            .service(crate::handler::datascope_paged)
            .service(crate::handler::datascope_get)
            .service(crate::handler::datasource_save)
            .service(crate::handler::datasource_update)
            .service(crate::handler::datasource_delete)
            .service(crate::handler::datasource_delete_ids)
            .service(crate::handler::datasource_search)
            .service(crate::handler::datasource_paged)
            .service(crate::handler::datasource_get)
            .service(crate::handler::apiserver_save)
            .service(crate::handler::apiserver_update)
            .service(crate::handler::apiserver_delete)
            .service(crate::handler::apiserver_delete_ids)
            .service(crate::handler::apiserver_search)
            .service(crate::handler::apiserver_paged)
            .service(crate::handler::apiserver_get)
            .service(crate::handler::dict_save)
            .service(crate::handler::dict_update)
            .service(crate::handler::dict_delete)
            .service(crate::handler::dict_delete_ids)
            .service(crate::handler::dict_search)
            .service(crate::handler::dict_paged)
            .service(crate::handler::dict_get)
            .service(crate::handler::dict_detail_save)
            .service(crate::handler::dict_detail_update)
            .service(crate::handler::dict_detail_delete)
            .service(crate::handler::dict_detail_delete_ids)
            .service(crate::handler::dict_detail_search)
            .service(crate::handler::dict_detail_paged)
            .service(crate::handler::dict_detail_get)
            .service(crate::handler::base_area_save)
            .service(crate::handler::base_area_update)
            .service(crate::handler::base_area_delete)
            .service(crate::handler::base_area_delete_ids)
            .service(crate::handler::base_area_search)
            .service(crate::handler::base_area_paged)
            .service(crate::handler::base_area_get)
            .service(crate::handler::base_area_tree)
            .service(crate::handler::base_area_superior)
            .service(crate::handler::base_area_normaltree)
            .service(crate::handler::query_dict_detail_query)
            .service(crate::handler::query_dict_detail_paged)
            .service(crate::handler::logs_save)
            .service(crate::handler::logs_update)
            .service(crate::handler::logs_delete)
            .service(crate::handler::logs_delete_ids)
            .service(crate::handler::logs_search)
            .service(crate::handler::logs_paged)
            .service(crate::handler::logs_get)
            .service(crate::handler::user_roles_rel_load)
            .service(crate::handler::user_roles_rel_remove)
            .service(crate::handler::user_roles_rel_save)
            .service(crate::handler::auth_code)
            .service(crate::handler::auth_login)
            .service(crate::handler::auth_info)
            .service(crate::handler::auth_logout)
            .service(crate::handler::menu_build)
            .service(crate::handler::user_center_update)
            .service(crate::handler::user_center_update_email)
            .service(crate::handler::user_center_update_pwd)
            .service(crate::handler::user_center_update_avatar)
            .service(crate::handler::user_common_update_avatar)
            .service(crate::handler::resource_common_upload_image)
            .service(crate::handler::user_reset_email)
            .service(crate::handler::company_save)
            .service(crate::handler::company_update)
            .service(crate::handler::company_delete)
            .service(crate::handler::company_delete_ids)
            .service(crate::handler::company_search)
            .service(crate::handler::company_paged)
            .service(crate::handler::company_get)
            .service(crate::handler::company_unioncode)
            .service(crate::handler::company_register)
            .service(crate::handler::account_save)
            .service(crate::handler::account_update)
            .service(crate::handler::account_delete)
            .service(crate::handler::account_delete_ids)
            .service(crate::handler::account_search)
            .service(crate::handler::account_paged)
            .service(crate::handler::account_get)
            .service(crate::handler::attachment_save)
            .service(crate::handler::attachment_update)
            .service(crate::handler::attachment_delete)
            .service(crate::handler::attachment_delete_ids)
            .service(crate::handler::attachment_search)
            .service(crate::handler::attachment_paged)
            .service(crate::handler::attachment_get)
            .service(crate::handler::attachref_save)
            .service(crate::handler::attachref_update)
            .service(crate::handler::attachref_delete)
            .service(crate::handler::attachref_delete_ids)
            .service(crate::handler::attachref_search)
            .service(crate::handler::attachref_paged)
            .service(crate::handler::attachref_get)
            .service(crate::handler::attachment_query)
            .service(crate::handler::resource_common_upload_file)
            .service(crate::handler::resource_common_upload_multi_files)
            .service(crate::handler::show_avatar)
            .service(crate::handler::show_type_avatar)
            .service(crate::handler::show_org_file_content)
            .service(crate::handler::holiday_save)
            .service(crate::handler::holiday_update)
            .service(crate::handler::holiday_paged)
            .service(crate::handler::holiday_search)
            .service(crate::handler::holiday_delete_phy_year)
            .service(crate::handler::holiday_delete)
            .service(crate::handler::holiday_delete_ids)
    }
}
