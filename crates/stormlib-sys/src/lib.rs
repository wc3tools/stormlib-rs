#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

include!("./bindings.rs");

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
