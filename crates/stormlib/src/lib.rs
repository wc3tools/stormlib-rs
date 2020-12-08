use std::ffi::*;
use std::path::Path;
use std::ptr;
use stormlib_sys::*;

#[macro_use]
mod util;

mod constants;
pub use constants::*;

pub mod error;
use error::*;

/// MPQ archive
#[derive(Debug)]
pub struct Archive {
  handle: HANDLE,
}

impl Archive {
  /// Opens a MPQ archive
  pub fn open<P: AsRef<Path>>(path: P, flags: OpenArchiveFlags) -> Result<Self> {
    let pathstr = path.as_ref().to_str().ok_or_else(|| StormError::NonUtf8)?;
    let cpath = CString::new(pathstr)?;
    let mut handle: HANDLE = ptr::null_mut();
    unsafe {
      unsafe_try_call!(SFileOpenArchive(
        cpath.as_ptr(),
        0,
        flags.bits(),
        &mut handle as *mut HANDLE,
      ));
      Ok(Archive { handle })
    }
  }

  /// Quick check if the file exists within MPQ archive, without opening it
  pub fn has_file(&mut self, path: &str) -> Result<bool> {
    let cpath = CString::new(path)?;
    unsafe {
      let r = SFileHasFile(self.handle, cpath.as_ptr());
      let err = GetLastError();
      if !r && err != ERROR_FILE_NOT_FOUND {
        return Err(From::from(err));
      }
      Ok(r)
    }
  }

  /// Opens a file from MPQ archive
  pub fn open_file<'a>(&'a mut self, path: &str) -> Result<File<'a>> {
    let mut file_handle: HANDLE = ptr::null_mut();
    let cpath = CString::new(path)?;
    unsafe_try_call!(SFileOpenFileEx(
      self.handle,
      cpath.as_ptr(),
      0,
      &mut file_handle as *mut HANDLE
    ));
    Ok(File {
      archive: self,
      file_handle,
      size: None,
      need_reset: false,
    })
  }
}

impl std::ops::Drop for Archive {
  fn drop(&mut self) {
    unsafe {
      SFileCloseArchive(self.handle);
    }
  }
}

/// Opened file
#[derive(Debug)]
pub struct File<'a> {
  archive: &'a Archive,
  file_handle: HANDLE,
  size: Option<u64>,
  need_reset: bool,
}

impl<'a> File<'a> {
  /// Retrieves a size of the file within archive
  pub fn get_size(&mut self) -> Result<u64> {
    if let Some(size) = self.size.clone() {
      Ok(size)
    } else {
      let mut high: DWORD = 0;
      let low = unsafe { SFileGetFileSize(self.file_handle, &mut high as *mut DWORD) };
      if low == SFILE_INVALID_SIZE {
        return Err(From::from(unsafe { GetLastError() }));
      }
      let high = (high as u64) << 32;
      let size = high | (low as u64);
      self.size = Some(size);
      return Ok(size);
    }
  }

  /// Reads all data from the file
  pub fn read_all(&mut self) -> Result<Vec<u8>> {
    if self.need_reset {
      unsafe {
        if SFileSetFilePointer(self.file_handle, 0, ptr::null_mut(), 0) == SFILE_INVALID_SIZE {
          return Err(From::from(GetLastError()));
        }
      }
    }

    let size = self.get_size()?;
    let mut buf = Vec::<u8>::with_capacity(size as usize);
    buf.resize(buf.capacity(), 0);
    let mut read: DWORD = 0;
    self.need_reset = true;
    unsafe_try_call!(SFileReadFile(
      self.file_handle,
      std::mem::transmute(buf.as_mut_ptr()),
      size as u32,
      &mut read as *mut DWORD,
      ptr::null_mut(),
    ));
    if (read as u64) < size {
      buf.truncate(read as usize);
    }
    Ok(buf)
  }
}

impl<'a> std::ops::Drop for File<'a> {
  fn drop(&mut self) {
    unsafe {
      SFileCloseFile(self.file_handle);
    }
  }
}

#[test]
fn test_read() {
  let mut archive = Archive::open(
    "../../samples/test_tft.w3x",
    OpenArchiveFlags::MPQ_OPEN_NO_LISTFILE | OpenArchiveFlags::MPQ_OPEN_NO_ATTRIBUTES,
  )
  .unwrap();

  assert_eq!(archive.has_file("invalid").unwrap(), false);
  assert_eq!(archive.has_file("war3map.j").unwrap(), true);
  let mut f = archive.open_file("war3map.j").unwrap();
  assert_eq!(f.get_size().unwrap(), 14115);
  assert_eq!(
    f.read_all().unwrap(),
    std::fs::read("../../samples/war3map.j").unwrap()
  );
}

#[cfg(target_os = "windows")]
#[test]
fn test_read_unicode() {
  use widestring::U16String;
  let mut archive = Archive::open(
    U16String::from_str("../../samples/中文.w3x").to_os_string(),
    OpenArchiveFlags::MPQ_OPEN_NO_LISTFILE | OpenArchiveFlags::MPQ_OPEN_NO_ATTRIBUTES,
  )
    .unwrap();
  let mut f = archive.open_file("war3map.j").unwrap();
  assert_eq!(
    f.read_all().unwrap(),
    std::fs::read("../../samples/war3map.j").unwrap()
  );
}

#[cfg(target_os = "macos")]
#[test]
fn test_read_utf8() {
  let mut archive = Archive::open(
    "../../samples/中文.w3x",
    OpenArchiveFlags::MPQ_OPEN_NO_LISTFILE | OpenArchiveFlags::MPQ_OPEN_NO_ATTRIBUTES,
  )
    .unwrap();
  let mut f = archive.open_file("war3map.j").unwrap();
  assert_eq!(
    f.read_all().unwrap(),
    std::fs::read("../../samples/war3map.j").unwrap()
  );
}