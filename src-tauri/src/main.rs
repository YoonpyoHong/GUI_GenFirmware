#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    gui_gen_firmware_lib::run();
}
