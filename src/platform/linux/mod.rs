mod udev;

use libc;
use std::ffi::CStr;
use std::os::raw::c_char;

use self::udev::*;
use crate::{ControllerInfo, ControllerState, ControllerStatus, DEFAULT_CONTROLLER_INFO,
                   DEFAULT_CONTROLLER_STATE, MAX_DEVICES};

pub struct ControllerContext {
    udev: Option<Udev>,
    info: Vec<ControllerInfo>,
    state: Vec<ControllerState>,
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
    pub fn scan_controllers(&self) -> usize {
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
                        // if let Some(gamepad) = Gamepad::open(&dev) {
                        //     gamepads
                        //         .push(MainGamepad::from_inner_status(gamepad, ControllerStatus::Connected));
                        //     additional_events.push_back(RawEvent::new(
                        //         gamepads.len() - 1,
                        //         RawEventType::Connected,
                        //     ));
                        // }
                    }
                }
            }
        }
        0
    }
    /// Update controller state by index
    pub fn update(&mut self, _index: usize) {}
    /// Get current information of Controller
    pub fn info(&mut self, _index: usize) -> &ControllerInfo {
        &*DEFAULT_CONTROLLER_INFO
    }
    /// Get current state of Controller
    pub fn state(&mut self, _index: usize) -> &ControllerState {
        &DEFAULT_CONTROLLER_STATE
    }
}

unsafe fn cstr_new(bytes: &[u8]) -> &CStr {
    CStr::from_bytes_with_nul_unchecked(bytes)
}
