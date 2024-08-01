#![feature(proc_macro_hygiene)]


use smash::app::{self, lua_bind::*};
use rand::Rng;
pub mod ext;
use ext::Controller;

static mut SHOULD_END_RESULT_SCREEN : bool = false;
pub static mut FIGHTER_MANAGER_ADDR: usize = 0;

#[skyline::hook(offset = 0x3664CC0)]
unsafe fn process_inputs_handheld(controller: &mut Controller) {
    let mgr = *(FIGHTER_MANAGER_ADDR as *mut *mut app::FighterManager);
    let entry_count = FighterManager::entry_count(mgr);
    if FighterManager::is_result_mode(mgr) && entry_count > 0 {
        if ninput::any::is_press(ninput::Buttons::PLUS) {
            SHOULD_END_RESULT_SCREEN = true;
        }
        if ninput::any::is_press(ninput::Buttons::B) {
            SHOULD_END_RESULT_SCREEN = false;
        }
        if SHOULD_END_RESULT_SCREEN {
            let mut rng = rand::thread_rng();
            // Need to space apart A-presses so it does not seem like we are holding the button.
            let n: u32 = rng.gen_range(0..3);
            if n == 1 {
                controller.current_buttons.set_a(true);
                controller.just_down.set_a(true);
            }
        }
    }
    if entry_count == 0 {
        SHOULD_END_RESULT_SCREEN = false;
    }
    call_original!(controller);
}

#[skyline::main(name = "results-screen")]
pub fn main() {
    unsafe {
        skyline::nn::ro::LookupSymbol(
            &mut FIGHTER_MANAGER_ADDR,
            "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E\u{0}".as_bytes().as_ptr(),
        );
    }
    skyline::install_hooks!(
        process_inputs_handheld
    );
}
