use std::string::ToString;

use lazy_static::lazy_static;

use crate::windows_service_manage::WindowsServiceManage;

lazy_static! {
    static ref ZEROTIER_SERVICE_NAME: String = {
        String::from("ZeroTierOneService")
    };
    static ref ZEROTIER_SERVICE_MANAGE: WindowsServiceManage = {
      WindowsServiceManage::new(ZEROTIER_SERVICE_NAME.to_string())
    };
}

#[tauri::command]
pub(crate) fn get_zerotier_services() -> String {
    return ZEROTIER_SERVICE_MANAGE.get_service_info();
}

#[tauri::command]
pub(crate) fn get_zerotier_start_type() -> String {
    return format!("{:?}", ZEROTIER_SERVICE_MANAGE.get_start_type());
}

#[tauri::command]
pub(crate) fn set_zerotier_start_type(start_type: String) {
    ZEROTIER_SERVICE_MANAGE.set_start_type(start_type);
}

#[tauri::command]
pub(crate) fn start_zerotier() {
    ZEROTIER_SERVICE_MANAGE.start();
}

#[tauri::command]
pub(crate) fn stop_zerotier() {
    ZEROTIER_SERVICE_MANAGE.stop();
}

#[tauri::command]
pub(crate) fn get_zerotier_state() -> String {
    return format!("{:?}", ZEROTIER_SERVICE_MANAGE.get_state());
}


#[cfg(test)]
mod tests {
    use log::info;
    use log::LevelFilter::Debug;

    use crate::logger::init_logger_with_level;
    use crate::zerotier_manage::*;

    fn setup() {
        init_logger_with_level(Debug)
    }

    #[test]
    fn test_get_zerotier_services() {
        setup();
        info!("test_get_zerotier_services:{:?}", get_zerotier_services())
    }

    #[test]
    fn test_get_zerotier_state() {
        setup();
        info!("test_get_zerotier_state:{:?}", get_zerotier_state())
    }

    #[test]
    fn test_get_zerotier_start_type() {
        setup();
        info!("test_get_zerotier_start_type:{:?}", get_zerotier_start_type())
    }

    #[test]
    fn test_start_zerotier() {
        setup();
        info!("test_start_zerotier:{:?}", start_zerotier());
        let state = get_zerotier_state();
        assert_eq!(state, "Running")
    }

    #[test]
    fn test_stop_zerotier() {
        setup();
        info!("test_start_zerotier:{:?}", stop_zerotier());
        let state = get_zerotier_state();
        assert_eq!(state, "Stopped")
    }
}
