#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

#[cfg(windows)]
mod error_codes {
  pub type ErrorCode = u32;
  pub use winapi::shared::winerror::{
    ERROR_ACCESS_DENIED, ERROR_ALREADY_EXISTS, ERROR_BAD_FORMAT, ERROR_CAN_NOT_COMPLETE,
    ERROR_DISK_FULL, ERROR_FILE_CORRUPT, ERROR_FILE_NOT_FOUND, ERROR_HANDLE_EOF,
    ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_HANDLE, ERROR_INVALID_PARAMETER, ERROR_NEGATIVE_SEEK,
    ERROR_NOT_ENOUGH_MEMORY, ERROR_NOT_SUPPORTED, ERROR_NO_MORE_FILES, ERROR_SUCCESS,
  };
}

#[cfg(unix)]
mod error_codes {
  pub type ErrorCode = i32;
  pub const ERROR_SUCCESS: i32 = 0;
  pub const ERROR_FILE_NOT_FOUND: i32 = libc::ENOENT;
  pub const ERROR_ACCESS_DENIED: i32 = libc::EPERM;
  pub const ERROR_INVALID_HANDLE: i32 = libc::EBADF;
  pub const ERROR_NOT_ENOUGH_MEMORY: i32 = libc::ENOMEM;
  pub const ERROR_NOT_SUPPORTED: i32 = libc::ENOTSUP;
  pub const ERROR_INVALID_PARAMETER: i32 = libc::EINVAL;
  pub const ERROR_NEGATIVE_SEEK: i32 = libc::EINVAL;
  pub const ERROR_DISK_FULL: i32 = libc::ENOSPC;
  pub const ERROR_ALREADY_EXISTS: i32 = libc::EEXIST;
  pub const ERROR_INSUFFICIENT_BUFFER: i32 = libc::ENOBUFS;
  pub const ERROR_BAD_FORMAT: i32 = 1000;
  pub const ERROR_NO_MORE_FILES: i32 = 1001;
  pub const ERROR_HANDLE_EOF: i32 = 1002;
  pub const ERROR_CAN_NOT_COMPLETE: i32 = 1003;
  pub const ERROR_FILE_CORRUPT: i32 = 1004;
}

pub use error_codes::*;

#[cfg(target_os = "windows")]
include!("./bindings_windows.rs");
#[cfg(target_os = "linux")]
include!("./bindings_linux.rs");
#[cfg(target_os = "macos")]
include!("./bindings_macos.rs");

extern "C" {
  pub fn SetLastError(dwErrCode: ErrorCode);
}
extern "C" {
  pub fn GetLastError() -> ErrorCode;
}

#[test]
fn test_w3x() {
  use std::ffi::*;
  use std::ptr;

  let path = CString::new("../../samples/test_tft.w3x").unwrap();
  let file = CString::new("war3map.j").unwrap();
  unsafe {
    let mut handle: HANDLE = ptr::null_mut();
    let ok = SFileOpenArchive(
      path.as_ptr(),
      0,
      MPQ_OPEN_NO_LISTFILE | MPQ_OPEN_NO_ATTRIBUTES,
      &mut handle as *mut HANDLE,
    );
    assert!(ok);

    let mut file_handle: HANDLE = ptr::null_mut();
    let ok = SFileOpenFileEx(handle, file.as_ptr(), 0, &mut file_handle as *mut HANDLE);
    assert!(ok);

    let mut size_high: DWORD = 0;
    let size = SFileGetFileSize(file_handle, &mut size_high as *mut DWORD);
    assert!(ok);

    println!("file size = {}", size);

    let mut buf = Vec::<u8>::with_capacity(size as usize);
    buf.resize(buf.capacity(), 0);

    let mut read: DWORD = 0;
    let ok = SFileReadFile(
      file_handle,
      std::mem::transmute(buf.as_mut_ptr()),
      size,
      &mut read as *mut DWORD,
      ptr::null_mut(),
    );
    assert!(ok);

    println!("read size = {}", read);

    assert_eq!(buf, std::fs::read("../../samples/war3map.j").unwrap());

    assert!(SFileCloseFile(file_handle));

    assert!(SFileCloseArchive(handle));
  }
}
