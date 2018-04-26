use std::mem;

use winapi::shared::winerror::ERROR_SUCCESS;
use winapi::um::xinput::{self, XINPUT_CAPABILITIES as XCapabilities, XINPUT_STATE as XState,
                         XINPUT_FLAG_GAMEPAD, XINPUT_GAMEPAD_A, XINPUT_GAMEPAD_B,
                         XINPUT_GAMEPAD_BACK, XINPUT_GAMEPAD_DPAD_DOWN, XINPUT_GAMEPAD_DPAD_LEFT,
                         XINPUT_GAMEPAD_DPAD_RIGHT, XINPUT_GAMEPAD_DPAD_UP,
                         XINPUT_GAMEPAD_LEFT_SHOULDER, XINPUT_GAMEPAD_LEFT_THUMB,
                         XINPUT_GAMEPAD_RIGHT_SHOULDER, XINPUT_GAMEPAD_RIGHT_THUMB,
                         XINPUT_GAMEPAD_START, XINPUT_GAMEPAD_X, XINPUT_GAMEPAD_Y};

use super::super::{ControllerContextInterface, ControllerInfo, ControllerState, ControllerStatus,
                   DEFAULT_CONTROLLER_INFO, DEFAULT_CONTROLLER_STATE, MAX_DEVICES, MAX_DIGITAL};

pub struct ControllerContext {
    info: Vec<ControllerInfo>,
    state: Vec<ControllerState>,
    buttons: Vec<[u16; MAX_DIGITAL]>,
}

impl ControllerContextInterface for ControllerContext {
    fn get_controller_count(&mut self) -> usize {
        let mut count = 0;
        let mut state = unsafe { mem::zeroed::<XState>() };
        for id in 0..4 {
            let val = unsafe { xinput::XInputGetState(id as u32, &mut state) };
            if val == ERROR_SUCCESS {
                count += 1;
                self.state[id].status = ControllerStatus::Connected;
            } else {
                self.state[id].status = ControllerStatus::Disconnected;
            }
        }
        count
    }
    fn borrow_controller_info(&mut self, controller_num: usize) -> &ControllerInfo {
        let mut capabilities = unsafe { mem::zeroed::<XCapabilities>() };
        if unsafe {
            xinput::XInputGetCapabilities(
                controller_num as u32,
                XINPUT_FLAG_GAMEPAD,
                &mut capabilities,
            )
        } == ERROR_SUCCESS
        {
            self.update_info(controller_num, &capabilities);
            &self.info[controller_num]
        } else {
            &*DEFAULT_CONTROLLER_INFO
        }
    }
    fn borrow_controller_state(&mut self, controller_num: usize) -> &ControllerState {
        let mut state = unsafe { mem::zeroed::<XState>() };
        let val = unsafe { xinput::XInputGetState(controller_num as u32, &mut state) };
        if val == ERROR_SUCCESS {
            self.update_state(controller_num, &state);
            &self.state[controller_num]
        } else {
            &DEFAULT_CONTROLLER_STATE
        }
    }
}

impl ControllerContext {
    pub fn new() -> Self {
        let mut info = Vec::new();
        let mut state = Vec::new();
        let mut buttons = Vec::new();
        unsafe { xinput::XInputEnable(1) };
        for _ in 0..MAX_DEVICES {
            info.push(ControllerInfo::new());
            state.push(ControllerState::new());
            buttons.push([0; MAX_DIGITAL]);
        }
        Self {
            info,
            state,
            buttons,
        }
    }
    fn update_state(&mut self, id: usize, state: &XState) {
        if state.dwPacketNumber as usize == self.state[id].sequence {
            // no change in state
            return;
        }
        self.state[id].sequence = state.dwPacketNumber as usize;
        if self.info[id].digital_count == 0 {
            self.borrow_controller_info(id);
        }
        for i in 0..self.info[id].digital_count {
            self.state[id].digital_state[i] = state.Gamepad.wButtons & self.buttons[id][i] != 0;
        }
        self.state[id].analog_state[0] =
            (state.Gamepad.sThumbLX as i32 + 32768) as f32 / 65535.0 * 2.0 - 1.0;
        self.state[id].analog_state[1] =
            (state.Gamepad.sThumbLY as i32 + 32768) as f32 / 65535.0 * 2.0 - 1.0;
        self.state[id].analog_state[2] = state.Gamepad.bLeftTrigger as f32 / 255.0 * 2.0 - 1.0;
        self.state[id].analog_state[3] = state.Gamepad.bRightTrigger as f32 / 255.0 * 2.0 - 1.0;
        self.state[id].analog_state[4] =
            (state.Gamepad.sThumbRX as i32 + 32768) as f32 / 65535.0 * 2.0 - 1.0;
        self.state[id].analog_state[5] =
            (state.Gamepad.sThumbRY as i32 + 32768) as f32 / 65535.0 * 2.0 - 1.0;
    }
    fn update_info(&mut self, id: usize, capabilities: &XCapabilities) {
        let mut name = String::from("XBOX360");
        match capabilities.SubType {
            xinput::XINPUT_DEVSUBTYPE_GAMEPAD => name.push_str(" gamepad"),
            xinput::XINPUT_DEVSUBTYPE_WHEEL => name.push_str(" wheel"),
            xinput::XINPUT_DEVSUBTYPE_ARCADE_STICK => name.push_str(" arcade stick"),
            xinput::XINPUT_DEVSUBTYPE_FLIGHT_SICK => name.push_str(" flight stick"),
            xinput::XINPUT_DEVSUBTYPE_DANCE_PAD => name.push_str(" dance pad"),
            xinput::XINPUT_DEVSUBTYPE_GUITAR => name.push_str(" guitar"),
            xinput::XINPUT_DEVSUBTYPE_DRUM_KIT => name.push_str(" drum"),
            _ => (),
        };
        name.push_str(" controller");
        self.info[id].name = name;
        let mut buttons = 0;
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_A != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_A;
            buttons += 1;
        }
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_B != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_B;
            buttons += 1;
        }
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_X != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_X;
            buttons += 1;
        }
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_Y != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_Y;
            buttons += 1;
        }
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_UP != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_DPAD_UP;
            buttons += 1;
        }
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_DOWN != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_DPAD_DOWN;
            buttons += 1;
        }
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_LEFT != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_DPAD_LEFT;
            buttons += 1;
        }
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_RIGHT != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_DPAD_RIGHT;
            buttons += 1;
        }
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_START != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_START;
            buttons += 1;
        }
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_BACK != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_BACK;
            buttons += 1;
        }
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_LEFT_THUMB != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_LEFT_THUMB;
            buttons += 1;
        }
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_RIGHT_THUMB != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_RIGHT_THUMB;
            buttons += 1;
        }
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_LEFT_SHOULDER != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_LEFT_SHOULDER;
            buttons += 1;
        }
        if capabilities.Gamepad.wButtons & XINPUT_GAMEPAD_RIGHT_SHOULDER != 0 {
            self.buttons[id][buttons] = XINPUT_GAMEPAD_RIGHT_SHOULDER;
            buttons += 1;
        }
        self.info[id].digital_count = buttons;
        let mut axis = 0;
        if capabilities.Gamepad.bLeftTrigger != 0 {
            axis += 1;
        }
        if capabilities.Gamepad.bRightTrigger != 0 {
            axis += 1;
        }
        if capabilities.Gamepad.sThumbLX != 0 {
            axis += 1;
        }
        if capabilities.Gamepad.sThumbLY != 0 {
            axis += 1;
        }
        if capabilities.Gamepad.sThumbRX != 0 {
            axis += 1;
        }
        if capabilities.Gamepad.sThumbRY != 0 {
            axis += 1;
        }
        self.info[id].analog_count = axis;
    }
}
