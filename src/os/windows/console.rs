use std::{ptr, slice};

use winapi::{
    um::{
        winbase::{
            STD_OUTPUT_HANDLE,
            STD_INPUT_HANDLE,
            FORMAT_MESSAGE_ALLOCATE_BUFFER,
            FORMAT_MESSAGE_FROM_SYSTEM,
            FORMAT_MESSAGE_IGNORE_INSERTS,
            FormatMessageA,
        },
        processenv::GetStdHandle,
        errhandlingapi::GetLastError,
        winnt::{
            HANDLE,
            MAKELANGID,
            LANG_NEUTRAL,
            SUBLANG_DEFAULT,
        },
        handleapi::INVALID_HANDLE_VALUE,
    },
    shared::ntdef::NULL,
};

pub fn get_last_error() -> Option<String> {
    let error = unsafe {
        GetLastError()
    };

    if error == 0 {
        return None;
    }

    let mut raw_buffer: *const u8 = ptr::null();
    let size = unsafe {
        FormatMessageA(FORMAT_MESSAGE_ALLOCATE_BUFFER | FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
            NULL, error, MAKELANGID(LANG_NEUTRAL, SUBLANG_DEFAULT) as u32, &mut raw_buffer as *mut _ as *mut _, 0, ptr::null_mut())
    };

    let buffer = unsafe {
        slice::from_raw_parts(raw_buffer, size as usize)
    };
    let message = String::from_utf8_lossy(buffer).into_owned();

    Some(message)
}

pub fn die_if_win32_error() {
    if let Some(message) = get_last_error() {
        panic!("Win32 error: {}", message);
    }
}

pub fn get_stdout_handle() -> HANDLE {
    let handle = unsafe {
        GetStdHandle(STD_OUTPUT_HANDLE)
    };

    if handle == INVALID_HANDLE_VALUE {
        die_if_win32_error();
    }

    if handle == NULL {
        panic!("No stdout available!");
    }

    handle
}

pub fn get_stdin_handle() -> HANDLE {
    let handle = unsafe {
        GetStdHandle(STD_INPUT_HANDLE)
    };

    if handle == INVALID_HANDLE_VALUE {
        die_if_win32_error();
    }

    if handle == NULL {
        panic!("No stdout available!");
    }

    handle
}