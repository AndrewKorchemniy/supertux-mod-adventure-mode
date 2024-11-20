#![allow(non_snake_case)]

use std::ptr;
use winapi::um::memoryapi::VirtualProtect;
use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::um::winnt::PAGE_EXECUTE_READWRITE;
use winapi::ctypes::c_void;

static mut BASE: u32 = 0;

fn write_byte(location: *mut u8, value: u8) {
    unsafe {
        let mut old_protect = 0;
        VirtualProtect(location as *mut c_void, 1, PAGE_EXECUTE_READWRITE, &mut old_protect);
        ptr::write(location, value);
        VirtualProtect(location as *mut c_void, 1, old_protect, &mut old_protect);
    }
}

#[no_mangle]
pub extern "stdcall" fn DllMain(
    _hinst_dll: *mut c_void,
    fdw_reason: u32,
    _lpv_reserved: *mut c_void
) -> i32 {
    match fdw_reason {
        winapi::um::winnt::DLL_PROCESS_ATTACH => {
            unsafe {
                BASE = GetModuleHandleA(std::ptr::null()) as u32;

                // JMP over taking damage.
                write_byte((BASE + 0x00c41d3) as *mut u8, 0xeb);
            }
        }
        _ => {}
    }
    1 // TRUE
}
