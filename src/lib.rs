#![feature(lang_items)]
#![feature(panic_implementation)]
#![no_std]

use core::slice;

#[no_mangle]
pub extern "C" fn add_u8(a: u8, b: u8) -> u8 {
   a + b
}

#[cfg(target_os = "none")]
#[panic_implementation]
#[no_mangle]
pub extern "C" fn my_panic(panic_info: &core::panic::PanicInfo) -> ! {
    if let Some(location) = panic_info.location() {
        //println!("panic occurred in file '{}' at line {}", location.file(),
        let _line = location.line();
    } else {
        //jprintln!("panic occurred but can't get location information...");
    }
    loop {}
}

#[cfg(target_os = "none")]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[cfg(target_os = "linux")]
mod test {
    use add_u8;

    #[test]
    fn simple_add() {
        assert_eq!(add_u8(2,3), 5);
    }

}
