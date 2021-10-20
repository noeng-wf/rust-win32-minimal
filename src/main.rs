//
// Must be compiled with: cargo rustc -- -C link-args="/ENTRY:_start"
//

#![no_std]
#![no_main]
#![windows_subsystem = "console"]

use winapi::ctypes::c_void;
use winapi::um::consoleapi::WriteConsoleA;
use winapi::um::processenv::GetStdHandle;
use winapi::um::processthreadsapi::ExitProcess;
use winapi::um::winbase::STD_OUTPUT_HANDLE;

#[no_mangle]
pub extern "C" fn _start() -> () {
    const HELLO: &'static str = "Hello, world!\r\n"; // Length 15
    let bytes_to_write: u32 = HELLO.len() as u32;
    let mut bytes_written: u32 = 0;

    unsafe {
        WriteConsoleA(
            GetStdHandle(STD_OUTPUT_HANDLE),
            HELLO.as_ptr() as *const c_void,
            bytes_to_write,
            &mut bytes_written,
            0 as *mut c_void,
        );
    }
}

#[panic_handler]
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    unsafe {
        ExitProcess(1);
    }
    loop {} // should not happen
}
