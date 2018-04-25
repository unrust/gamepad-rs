mod udev;

use libc;
use std::ffi::CStr;
use std::os::raw::c_char;

use self::udev::*;
use super::super::{ControllerContextInterface, ControllerInfo, ControllerState,
                   DEFAULT_CONTROLLER_INFO, DEFAULT_CONTROLLER_STATE, MAX_DEVICES};

pub struct ControllerContext {
    udev: Option<Udev>,
    info: Vec<ControllerInfo>,
    state: Vec<ControllerState>,
}

impl ControllerContextInterface for ControllerContext {
    fn get_controller_count(&self) -> usize {
        if let Some(ref udev) = self.udev {
            if let Some(ref en) = udev.enumerate() {
                unsafe {
                    en.add_match_property(cstr_new(b"ID_INPUT_JOYSTICK\0"), cstr_new(b"1\0"));
                }
                en.scan_devices();
                let count = 0;
                for dev in en.iter() {
                    if let Some(dev) = Device::from_syspath(&udev, &dev) {
                        if let Some(path) = dev.devnode() {
                            if unsafe {
                                !libc::strstr(path.as_ptr(), b"js\0".as_ptr() as *const c_char)
                                    .is_null()
                            } {
                                continue;
                            }
                        }
                        if let Some(gamepad) = Gamepad::open(&dev) {
                            gamepads
                                .push(MainGamepad::from_inner_status(gamepad, Status::Connected));
                            additional_events.push_back(RawEvent::new(
                                gamepads.len() - 1,
                                RawEventType::Connected,
                            ));
                        }
                    }
                }
            }
        }
        0
    }
    fn get_controller_info(&self, controller_num: usize) -> &ControllerInfo {
        &*DEFAULT_CONTROLLER_INFO
    }
    fn borrow_controller_state(&self, controller_num: usize) -> &ControllerState {
        &DEFAULT_CONTROLLER_STATE
    }
}

impl ControllerContext {
    pub fn new() -> Self {
        let mut info = Vec::new();
        let mut state = Vec::new();
        for _ in 0..MAX_DEVICES {
            info.push(ControllerInfo::new());
            state.push(ControllerState::new());
        }
        Self {
            udev: Udev::new(),
            info,
            state,
        }
    }
}

unsafe fn cstr_new(bytes: &[u8]) -> &CStr {
    CStr::from_bytes_with_nul_unchecked(bytes)
}
