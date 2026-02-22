use std::io;

type DWORD = u32;
type BOOL = i32;
type HANDLE = isize;

const STD_OUTPUT_HANDLE: DWORD = 0xFFFF_FFF5;
const ENABLE_VIRTUAL_TERMINAL_PROCESSING: DWORD = 0x0004;

extern "system" {
    fn GetStdHandle(nStdHandle: DWORD) -> HANDLE;
    fn GetConsoleMode(hConsoleHandle: HANDLE, lpMode: *mut DWORD) -> BOOL;
    fn SetConsoleMode(hConsoleHandle: HANDLE, dwMode: DWORD) -> BOOL;
}

pub fn enable_ansi() -> io::Result<()> {
    unsafe {
        let h = GetStdHandle(STD_OUTPUT_HANDLE);

        let mut mode: DWORD = 0;
        if GetConsoleMode(h, &mut mode as *mut DWORD) == 0 {
            return Err(io::Error::last_os_error());
        }

        let new_mode = mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
        if SetConsoleMode(h, new_mode) == 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}
