/* automatically generated by rust-bindgen 0.56.0 */

pub const ERROR_SUCCESS: u32 = 0;
pub const ERROR_FILE_NOT_FOUND: u32 = 2;
pub const ERROR_ACCESS_DENIED: u32 = 1;
pub const ERROR_INVALID_HANDLE: u32 = 9;
pub const ERROR_NOT_ENOUGH_MEMORY: u32 = 12;
pub const ERROR_NOT_SUPPORTED: u32 = 95;
pub const ERROR_INVALID_PARAMETER: u32 = 22;
pub const ERROR_DISK_FULL: u32 = 28;
pub const ERROR_ALREADY_EXISTS: u32 = 17;
pub const ERROR_INSUFFICIENT_BUFFER: u32 = 105;
pub const ERROR_BAD_FORMAT: u32 = 1000;
pub const ERROR_NO_MORE_FILES: u32 = 1001;
pub const ERROR_HANDLE_EOF: u32 = 1002;
pub const ERROR_CAN_NOT_COMPLETE: u32 = 1003;
pub const ERROR_FILE_CORRUPT: u32 = 1004;
pub const STORMLIB_VERSION: u32 = 2326;
pub const STORMLIB_VERSION_STRING: &'static [u8; 5usize] = b"9.22\0";
pub const ID_MPQ: u32 = 441536589;
pub const ID_MPQ_USERDATA: u32 = 458313805;
pub const ID_MPK: u32 = 441143373;
pub const ERROR_AVI_FILE: u32 = 10000;
pub const ERROR_UNKNOWN_FILE_KEY: u32 = 10001;
pub const ERROR_CHECKSUM_ERROR: u32 = 10002;
pub const ERROR_INTERNAL_FILE: u32 = 10003;
pub const ERROR_BASE_FILE_MISSING: u32 = 10004;
pub const ERROR_MARKED_FOR_DELETE: u32 = 10005;
pub const ERROR_FILE_INCOMPLETE: u32 = 10006;
pub const ERROR_UNKNOWN_FILE_NAMES: u32 = 10007;
pub const ERROR_CANT_FIND_PATCH_PREFIX: u32 = 10008;
pub const HASH_TABLE_SIZE_MIN: u32 = 4;
pub const HASH_TABLE_SIZE_DEFAULT: u32 = 4096;
pub const HASH_TABLE_SIZE_MAX: u32 = 524288;
pub const HASH_ENTRY_DELETED: u32 = 4294967294;
pub const HASH_ENTRY_FREE: u32 = 4294967295;
pub const HET_ENTRY_DELETED: u32 = 128;
pub const HET_ENTRY_FREE: u32 = 0;
pub const HASH_STATE_SIZE: u32 = 96;
pub const SFILE_OPEN_HARD_DISK_FILE: u32 = 2;
pub const SFILE_OPEN_CDROM_FILE: u32 = 3;
pub const SFILE_OPEN_FROM_MPQ: u32 = 0;
pub const SFILE_OPEN_CHECK_EXISTS: u32 = 4294967292;
pub const SFILE_OPEN_BASE_FILE: u32 = 4294967293;
pub const SFILE_OPEN_ANY_LOCALE: u32 = 4294967294;
pub const SFILE_OPEN_LOCAL_FILE: u32 = 4294967295;
pub const MPQ_FLAG_READ_ONLY: u32 = 1;
pub const MPQ_FLAG_CHANGED: u32 = 2;
pub const MPQ_FLAG_MALFORMED: u32 = 4;
pub const MPQ_FLAG_HASH_TABLE_CUT: u32 = 8;
pub const MPQ_FLAG_BLOCK_TABLE_CUT: u32 = 16;
pub const MPQ_FLAG_CHECK_SECTOR_CRC: u32 = 32;
pub const MPQ_FLAG_SAVING_TABLES: u32 = 64;
pub const MPQ_FLAG_PATCH: u32 = 128;
pub const MPQ_FLAG_WAR3_MAP: u32 = 256;
pub const MPQ_FLAG_LISTFILE_NONE: u32 = 512;
pub const MPQ_FLAG_LISTFILE_NEW: u32 = 1024;
pub const MPQ_FLAG_ATTRIBUTES_NONE: u32 = 2048;
pub const MPQ_FLAG_ATTRIBUTES_NEW: u32 = 4096;
pub const MPQ_FLAG_SIGNATURE_NONE: u32 = 8192;
pub const MPQ_FLAG_SIGNATURE_NEW: u32 = 16384;
pub const MPQ_SUBTYPE_MPQ: u32 = 0;
pub const MPQ_SUBTYPE_SQP: u32 = 1;
pub const MPQ_SUBTYPE_MPK: u32 = 2;
pub const SFILE_INVALID_SIZE: u32 = 4294967295;
pub const SFILE_INVALID_POS: u32 = 4294967295;
pub const SFILE_INVALID_ATTRIBUTES: u32 = 4294967295;
pub const MPQ_FILE_IMPLODE: u32 = 256;
pub const MPQ_FILE_COMPRESS: u32 = 512;
pub const MPQ_FILE_ENCRYPTED: u32 = 65536;
pub const MPQ_FILE_FIX_KEY: u32 = 131072;
pub const MPQ_FILE_PATCH_FILE: u32 = 1048576;
pub const MPQ_FILE_SINGLE_UNIT: u32 = 16777216;
pub const MPQ_FILE_DELETE_MARKER: u32 = 33554432;
pub const MPQ_FILE_SECTOR_CRC: u32 = 67108864;
pub const MPQ_FILE_SIGNATURE: u32 = 268435456;
pub const MPQ_FILE_EXISTS: u32 = 2147483648;
pub const MPQ_FILE_REPLACEEXISTING: u32 = 2147483648;
pub const MPQ_FILE_COMPRESS_MASK: u32 = 65280;
pub const MPQ_FILE_DEFAULT_INTERNAL: u32 = 4294967295;
pub const MPQ_FILE_VALID_FLAGS: u32 = 2534605568;
pub const MPQ_FILE_VALID_FLAGS_W3X: u32 = 2516779776;
pub const BLOCK_INDEX_MASK: u32 = 268435455;
pub const MPQ_COMPRESSION_HUFFMANN: u32 = 1;
pub const MPQ_COMPRESSION_ZLIB: u32 = 2;
pub const MPQ_COMPRESSION_PKWARE: u32 = 8;
pub const MPQ_COMPRESSION_BZIP2: u32 = 16;
pub const MPQ_COMPRESSION_SPARSE: u32 = 32;
pub const MPQ_COMPRESSION_ADPCM_MONO: u32 = 64;
pub const MPQ_COMPRESSION_ADPCM_STEREO: u32 = 128;
pub const MPQ_COMPRESSION_LZMA: u32 = 18;
pub const MPQ_COMPRESSION_NEXT_SAME: u32 = 4294967295;
pub const MPQ_WAVE_QUALITY_HIGH: u32 = 0;
pub const MPQ_WAVE_QUALITY_MEDIUM: u32 = 1;
pub const MPQ_WAVE_QUALITY_LOW: u32 = 2;
pub const HET_TABLE_SIGNATURE: u32 = 441730376;
pub const BET_TABLE_SIGNATURE: u32 = 441730370;
pub const MPQ_KEY_HASH_TABLE: u32 = 3283040112;
pub const MPQ_KEY_BLOCK_TABLE: u32 = 3968054179;
pub const LISTFILE_NAME: &'static [u8; 11usize] = b"(listfile)\0";
pub const SIGNATURE_NAME: &'static [u8; 12usize] = b"(signature)\0";
pub const ATTRIBUTES_NAME: &'static [u8; 13usize] = b"(attributes)\0";
pub const PATCH_METADATA_NAME: &'static [u8; 17usize] = b"(patch_metadata)\0";
pub const MPQ_FORMAT_VERSION_1: u32 = 0;
pub const MPQ_FORMAT_VERSION_2: u32 = 1;
pub const MPQ_FORMAT_VERSION_3: u32 = 2;
pub const MPQ_FORMAT_VERSION_4: u32 = 3;
pub const MPQ_ATTRIBUTE_CRC32: u32 = 1;
pub const MPQ_ATTRIBUTE_FILETIME: u32 = 2;
pub const MPQ_ATTRIBUTE_MD5: u32 = 4;
pub const MPQ_ATTRIBUTE_PATCH_BIT: u32 = 8;
pub const MPQ_ATTRIBUTE_ALL: u32 = 15;
pub const MPQ_ATTRIBUTES_V1: u32 = 100;
pub const BASE_PROVIDER_FILE: u32 = 0;
pub const BASE_PROVIDER_MAP: u32 = 1;
pub const BASE_PROVIDER_HTTP: u32 = 2;
pub const BASE_PROVIDER_MASK: u32 = 15;
pub const STREAM_PROVIDER_FLAT: u32 = 0;
pub const STREAM_PROVIDER_PARTIAL: u32 = 16;
pub const STREAM_PROVIDER_MPQE: u32 = 32;
pub const STREAM_PROVIDER_BLOCK4: u32 = 48;
pub const STREAM_PROVIDER_MASK: u32 = 240;
pub const STREAM_FLAG_READ_ONLY: u32 = 256;
pub const STREAM_FLAG_WRITE_SHARE: u32 = 512;
pub const STREAM_FLAG_USE_BITMAP: u32 = 1024;
pub const STREAM_OPTIONS_MASK: u32 = 65280;
pub const STREAM_PROVIDERS_MASK: u32 = 255;
pub const STREAM_FLAGS_MASK: u32 = 65535;
pub const MPQ_OPEN_NO_LISTFILE: u32 = 65536;
pub const MPQ_OPEN_NO_ATTRIBUTES: u32 = 131072;
pub const MPQ_OPEN_NO_HEADER_SEARCH: u32 = 262144;
pub const MPQ_OPEN_FORCE_MPQ_V1: u32 = 524288;
pub const MPQ_OPEN_CHECK_SECTOR_CRC: u32 = 1048576;
pub const MPQ_OPEN_PATCH: u32 = 2097152;
pub const MPQ_OPEN_READ_ONLY: u32 = 256;
pub const MPQ_CREATE_LISTFILE: u32 = 1048576;
pub const MPQ_CREATE_ATTRIBUTES: u32 = 2097152;
pub const MPQ_CREATE_SIGNATURE: u32 = 4194304;
pub const MPQ_CREATE_ARCHIVE_V1: u32 = 0;
pub const MPQ_CREATE_ARCHIVE_V2: u32 = 16777216;
pub const MPQ_CREATE_ARCHIVE_V3: u32 = 33554432;
pub const MPQ_CREATE_ARCHIVE_V4: u32 = 50331648;
pub const MPQ_CREATE_ARCHIVE_VMASK: u32 = 251658240;
pub const FLAGS_TO_FORMAT_SHIFT: u32 = 24;
pub const SFILE_VERIFY_SECTOR_CRC: u32 = 1;
pub const SFILE_VERIFY_FILE_CRC: u32 = 2;
pub const SFILE_VERIFY_FILE_MD5: u32 = 4;
pub const SFILE_VERIFY_RAW_MD5: u32 = 8;
pub const SFILE_VERIFY_ALL: u32 = 15;
pub const VERIFY_OPEN_ERROR: u32 = 1;
pub const VERIFY_READ_ERROR: u32 = 2;
pub const VERIFY_FILE_HAS_SECTOR_CRC: u32 = 4;
pub const VERIFY_FILE_SECTOR_CRC_ERROR: u32 = 8;
pub const VERIFY_FILE_HAS_CHECKSUM: u32 = 16;
pub const VERIFY_FILE_CHECKSUM_ERROR: u32 = 32;
pub const VERIFY_FILE_HAS_MD5: u32 = 64;
pub const VERIFY_FILE_MD5_ERROR: u32 = 128;
pub const VERIFY_FILE_HAS_RAW_MD5: u32 = 256;
pub const VERIFY_FILE_RAW_MD5_ERROR: u32 = 512;
pub const VERIFY_FILE_ERROR_MASK: u32 = 683;
pub const SFILE_VERIFY_MPQ_HEADER: u32 = 1;
pub const SFILE_VERIFY_HET_TABLE: u32 = 2;
pub const SFILE_VERIFY_BET_TABLE: u32 = 3;
pub const SFILE_VERIFY_HASH_TABLE: u32 = 4;
pub const SFILE_VERIFY_BLOCK_TABLE: u32 = 5;
pub const SFILE_VERIFY_HIBLOCK_TABLE: u32 = 6;
pub const SFILE_VERIFY_FILE: u32 = 7;
pub const SIGNATURE_TYPE_NONE: u32 = 0;
pub const SIGNATURE_TYPE_WEAK: u32 = 1;
pub const SIGNATURE_TYPE_STRONG: u32 = 2;
pub const ERROR_NO_SIGNATURE: u32 = 0;
pub const ERROR_VERIFY_FAILED: u32 = 1;
pub const ERROR_WEAK_SIGNATURE_OK: u32 = 2;
pub const ERROR_WEAK_SIGNATURE_ERROR: u32 = 3;
pub const ERROR_STRONG_SIGNATURE_OK: u32 = 4;
pub const ERROR_STRONG_SIGNATURE_ERROR: u32 = 5;
pub const MD5_DIGEST_SIZE: u32 = 16;
pub const SHA1_DIGEST_SIZE: u32 = 20;
pub const LANG_NEUTRAL: u32 = 0;
pub const CCB_CHECKING_FILES: u32 = 1;
pub const CCB_CHECKING_HASH_TABLE: u32 = 2;
pub const CCB_COPYING_NON_MPQ_DATA: u32 = 3;
pub const CCB_COMPACTING_FILES: u32 = 4;
pub const CCB_CLOSING_ARCHIVE: u32 = 5;
pub const MPQ_HEADER_SIZE_V1: u32 = 32;
pub const MPQ_HEADER_SIZE_V2: u32 = 44;
pub const MPQ_HEADER_SIZE_V3: u32 = 68;
pub const MPQ_HEADER_SIZE_V4: u32 = 208;
pub const MPQ_HEADER_DWORDS: u32 = 52;
pub type LONG = ::std::os::raw::c_int;
pub type DWORD = ::std::os::raw::c_uint;
pub type ULONGLONG = ::std::os::raw::c_ulonglong;
pub type HANDLE = *mut ::std::os::raw::c_void;
pub type LPOVERLAPPED = *mut ::std::os::raw::c_void;
pub type TCHAR = ::std::os::raw::c_char;
pub type LCID = ::std::os::raw::c_uint;
pub type LPDWORD = *mut DWORD;
pub const _SFileInfoClass_SFileMpqFileName: _SFileInfoClass = 0;
pub const _SFileInfoClass_SFileMpqStreamBitmap: _SFileInfoClass = 1;
pub const _SFileInfoClass_SFileMpqUserDataOffset: _SFileInfoClass = 2;
pub const _SFileInfoClass_SFileMpqUserDataHeader: _SFileInfoClass = 3;
pub const _SFileInfoClass_SFileMpqUserData: _SFileInfoClass = 4;
pub const _SFileInfoClass_SFileMpqHeaderOffset: _SFileInfoClass = 5;
pub const _SFileInfoClass_SFileMpqHeaderSize: _SFileInfoClass = 6;
pub const _SFileInfoClass_SFileMpqHeader: _SFileInfoClass = 7;
pub const _SFileInfoClass_SFileMpqHetTableOffset: _SFileInfoClass = 8;
pub const _SFileInfoClass_SFileMpqHetTableSize: _SFileInfoClass = 9;
pub const _SFileInfoClass_SFileMpqHetHeader: _SFileInfoClass = 10;
pub const _SFileInfoClass_SFileMpqHetTable: _SFileInfoClass = 11;
pub const _SFileInfoClass_SFileMpqBetTableOffset: _SFileInfoClass = 12;
pub const _SFileInfoClass_SFileMpqBetTableSize: _SFileInfoClass = 13;
pub const _SFileInfoClass_SFileMpqBetHeader: _SFileInfoClass = 14;
pub const _SFileInfoClass_SFileMpqBetTable: _SFileInfoClass = 15;
pub const _SFileInfoClass_SFileMpqHashTableOffset: _SFileInfoClass = 16;
pub const _SFileInfoClass_SFileMpqHashTableSize64: _SFileInfoClass = 17;
pub const _SFileInfoClass_SFileMpqHashTableSize: _SFileInfoClass = 18;
pub const _SFileInfoClass_SFileMpqHashTable: _SFileInfoClass = 19;
pub const _SFileInfoClass_SFileMpqBlockTableOffset: _SFileInfoClass = 20;
pub const _SFileInfoClass_SFileMpqBlockTableSize64: _SFileInfoClass = 21;
pub const _SFileInfoClass_SFileMpqBlockTableSize: _SFileInfoClass = 22;
pub const _SFileInfoClass_SFileMpqBlockTable: _SFileInfoClass = 23;
pub const _SFileInfoClass_SFileMpqHiBlockTableOffset: _SFileInfoClass = 24;
pub const _SFileInfoClass_SFileMpqHiBlockTableSize64: _SFileInfoClass = 25;
pub const _SFileInfoClass_SFileMpqHiBlockTable: _SFileInfoClass = 26;
pub const _SFileInfoClass_SFileMpqSignatures: _SFileInfoClass = 27;
pub const _SFileInfoClass_SFileMpqStrongSignatureOffset: _SFileInfoClass = 28;
pub const _SFileInfoClass_SFileMpqStrongSignatureSize: _SFileInfoClass = 29;
pub const _SFileInfoClass_SFileMpqStrongSignature: _SFileInfoClass = 30;
pub const _SFileInfoClass_SFileMpqArchiveSize64: _SFileInfoClass = 31;
pub const _SFileInfoClass_SFileMpqArchiveSize: _SFileInfoClass = 32;
pub const _SFileInfoClass_SFileMpqMaxFileCount: _SFileInfoClass = 33;
pub const _SFileInfoClass_SFileMpqFileTableSize: _SFileInfoClass = 34;
pub const _SFileInfoClass_SFileMpqSectorSize: _SFileInfoClass = 35;
pub const _SFileInfoClass_SFileMpqNumberOfFiles: _SFileInfoClass = 36;
pub const _SFileInfoClass_SFileMpqRawChunkSize: _SFileInfoClass = 37;
pub const _SFileInfoClass_SFileMpqStreamFlags: _SFileInfoClass = 38;
pub const _SFileInfoClass_SFileMpqFlags: _SFileInfoClass = 39;
pub const _SFileInfoClass_SFileInfoPatchChain: _SFileInfoClass = 40;
pub const _SFileInfoClass_SFileInfoFileEntry: _SFileInfoClass = 41;
pub const _SFileInfoClass_SFileInfoHashEntry: _SFileInfoClass = 42;
pub const _SFileInfoClass_SFileInfoHashIndex: _SFileInfoClass = 43;
pub const _SFileInfoClass_SFileInfoNameHash1: _SFileInfoClass = 44;
pub const _SFileInfoClass_SFileInfoNameHash2: _SFileInfoClass = 45;
pub const _SFileInfoClass_SFileInfoNameHash3: _SFileInfoClass = 46;
pub const _SFileInfoClass_SFileInfoLocale: _SFileInfoClass = 47;
pub const _SFileInfoClass_SFileInfoFileIndex: _SFileInfoClass = 48;
pub const _SFileInfoClass_SFileInfoByteOffset: _SFileInfoClass = 49;
pub const _SFileInfoClass_SFileInfoFileTime: _SFileInfoClass = 50;
pub const _SFileInfoClass_SFileInfoFileSize: _SFileInfoClass = 51;
pub const _SFileInfoClass_SFileInfoCompressedSize: _SFileInfoClass = 52;
pub const _SFileInfoClass_SFileInfoFlags: _SFileInfoClass = 53;
pub const _SFileInfoClass_SFileInfoEncryptionKey: _SFileInfoClass = 54;
pub const _SFileInfoClass_SFileInfoEncryptionKeyRaw: _SFileInfoClass = 55;
pub const _SFileInfoClass_SFileInfoCRC32: _SFileInfoClass = 56;
pub type _SFileInfoClass = ::std::os::raw::c_uint;
pub use self::_SFileInfoClass as SFileInfoClass;
pub type SFILE_DOWNLOAD_CALLBACK = ::std::option::Option<
  unsafe extern "C" fn(
    pvUserData: *mut ::std::os::raw::c_void,
    ByteOffset: ULONGLONG,
    dwTotalBytes: DWORD,
  ),
>;
pub type SFILE_ADDFILE_CALLBACK = ::std::option::Option<
  unsafe extern "C" fn(
    pvUserData: *mut ::std::os::raw::c_void,
    dwBytesWritten: DWORD,
    dwTotalBytes: DWORD,
    bFinalCall: bool,
  ),
>;
pub type SFILE_COMPACT_CALLBACK = ::std::option::Option<
  unsafe extern "C" fn(
    pvUserData: *mut ::std::os::raw::c_void,
    dwWorkType: DWORD,
    BytesProcessed: ULONGLONG,
    TotalBytes: ULONGLONG,
  ),
>;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _SFILE_FIND_DATA {
  pub cFileName: [::std::os::raw::c_char; 1024usize],
  pub szPlainName: *mut ::std::os::raw::c_char,
  pub dwHashIndex: DWORD,
  pub dwBlockIndex: DWORD,
  pub dwFileSize: DWORD,
  pub dwFileFlags: DWORD,
  pub dwCompSize: DWORD,
  pub dwFileTimeLo: DWORD,
  pub dwFileTimeHi: DWORD,
  pub lcLocale: LCID,
}
#[test]
fn bindgen_test_layout__SFILE_FIND_DATA() {
  assert_eq!(
    ::std::mem::size_of::<_SFILE_FIND_DATA>(),
    1064usize,
    concat!("Size of: ", stringify!(_SFILE_FIND_DATA))
  );
  assert_eq!(
    ::std::mem::align_of::<_SFILE_FIND_DATA>(),
    8usize,
    concat!("Alignment of ", stringify!(_SFILE_FIND_DATA))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_FIND_DATA>())).cFileName as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_FIND_DATA),
      "::",
      stringify!(cFileName)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_FIND_DATA>())).szPlainName as *const _ as usize },
    1024usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_FIND_DATA),
      "::",
      stringify!(szPlainName)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_FIND_DATA>())).dwHashIndex as *const _ as usize },
    1032usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_FIND_DATA),
      "::",
      stringify!(dwHashIndex)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_FIND_DATA>())).dwBlockIndex as *const _ as usize },
    1036usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_FIND_DATA),
      "::",
      stringify!(dwBlockIndex)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_FIND_DATA>())).dwFileSize as *const _ as usize },
    1040usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_FIND_DATA),
      "::",
      stringify!(dwFileSize)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_FIND_DATA>())).dwFileFlags as *const _ as usize },
    1044usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_FIND_DATA),
      "::",
      stringify!(dwFileFlags)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_FIND_DATA>())).dwCompSize as *const _ as usize },
    1048usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_FIND_DATA),
      "::",
      stringify!(dwCompSize)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_FIND_DATA>())).dwFileTimeLo as *const _ as usize },
    1052usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_FIND_DATA),
      "::",
      stringify!(dwFileTimeLo)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_FIND_DATA>())).dwFileTimeHi as *const _ as usize },
    1056usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_FIND_DATA),
      "::",
      stringify!(dwFileTimeHi)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_FIND_DATA>())).lcLocale as *const _ as usize },
    1060usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_FIND_DATA),
      "::",
      stringify!(lcLocale)
    )
  );
}
pub type SFILE_FIND_DATA = _SFILE_FIND_DATA;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _SFILE_CREATE_MPQ {
  pub cbSize: DWORD,
  pub dwMpqVersion: DWORD,
  pub pvUserData: *mut ::std::os::raw::c_void,
  pub cbUserData: DWORD,
  pub dwStreamFlags: DWORD,
  pub dwFileFlags1: DWORD,
  pub dwFileFlags2: DWORD,
  pub dwFileFlags3: DWORD,
  pub dwAttrFlags: DWORD,
  pub dwSectorSize: DWORD,
  pub dwRawChunkSize: DWORD,
  pub dwMaxFileCount: DWORD,
}
#[test]
fn bindgen_test_layout__SFILE_CREATE_MPQ() {
  assert_eq!(
    ::std::mem::size_of::<_SFILE_CREATE_MPQ>(),
    56usize,
    concat!("Size of: ", stringify!(_SFILE_CREATE_MPQ))
  );
  assert_eq!(
    ::std::mem::align_of::<_SFILE_CREATE_MPQ>(),
    8usize,
    concat!("Alignment of ", stringify!(_SFILE_CREATE_MPQ))
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_CREATE_MPQ>())).cbSize as *const _ as usize },
    0usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_CREATE_MPQ),
      "::",
      stringify!(cbSize)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_CREATE_MPQ>())).dwMpqVersion as *const _ as usize },
    4usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_CREATE_MPQ),
      "::",
      stringify!(dwMpqVersion)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_CREATE_MPQ>())).pvUserData as *const _ as usize },
    8usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_CREATE_MPQ),
      "::",
      stringify!(pvUserData)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_CREATE_MPQ>())).cbUserData as *const _ as usize },
    16usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_CREATE_MPQ),
      "::",
      stringify!(cbUserData)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_CREATE_MPQ>())).dwStreamFlags as *const _ as usize },
    20usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_CREATE_MPQ),
      "::",
      stringify!(dwStreamFlags)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_CREATE_MPQ>())).dwFileFlags1 as *const _ as usize },
    24usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_CREATE_MPQ),
      "::",
      stringify!(dwFileFlags1)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_CREATE_MPQ>())).dwFileFlags2 as *const _ as usize },
    28usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_CREATE_MPQ),
      "::",
      stringify!(dwFileFlags2)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_CREATE_MPQ>())).dwFileFlags3 as *const _ as usize },
    32usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_CREATE_MPQ),
      "::",
      stringify!(dwFileFlags3)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_CREATE_MPQ>())).dwAttrFlags as *const _ as usize },
    36usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_CREATE_MPQ),
      "::",
      stringify!(dwAttrFlags)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_CREATE_MPQ>())).dwSectorSize as *const _ as usize },
    40usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_CREATE_MPQ),
      "::",
      stringify!(dwSectorSize)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_CREATE_MPQ>())).dwRawChunkSize as *const _ as usize },
    44usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_CREATE_MPQ),
      "::",
      stringify!(dwRawChunkSize)
    )
  );
  assert_eq!(
    unsafe { &(*(::std::ptr::null::<_SFILE_CREATE_MPQ>())).dwMaxFileCount as *const _ as usize },
    48usize,
    concat!(
      "Offset of field: ",
      stringify!(_SFILE_CREATE_MPQ),
      "::",
      stringify!(dwMaxFileCount)
    )
  );
}
pub type SFILE_CREATE_MPQ = _SFILE_CREATE_MPQ;
pub type PSFILE_CREATE_MPQ = *mut _SFILE_CREATE_MPQ;
pub type SFILESETLOCALE = ::std::option::Option<unsafe extern "C" fn(arg1: LCID) -> LCID>;
pub type SFILEOPENARCHIVE = ::std::option::Option<
  unsafe extern "C" fn(
    arg1: *const ::std::os::raw::c_char,
    arg2: DWORD,
    arg3: DWORD,
    arg4: *mut HANDLE,
  ) -> bool,
>;
pub type SFILECLOSEARCHIVE = ::std::option::Option<unsafe extern "C" fn(arg1: HANDLE) -> bool>;
pub type SFILEOPENFILEEX = ::std::option::Option<
  unsafe extern "C" fn(
    arg1: HANDLE,
    arg2: *const ::std::os::raw::c_char,
    arg3: DWORD,
    arg4: *mut HANDLE,
  ) -> bool,
>;
pub type SFILECLOSEFILE = ::std::option::Option<unsafe extern "C" fn(arg1: HANDLE) -> bool>;
pub type SFILEGETFILESIZE =
  ::std::option::Option<unsafe extern "C" fn(arg1: HANDLE, arg2: LPDWORD) -> DWORD>;
pub type SFILESETFILEPOINTER = ::std::option::Option<
  unsafe extern "C" fn(arg1: HANDLE, arg2: LONG, arg3: *mut LONG, arg4: DWORD) -> DWORD,
>;
pub type SFILEREADFILE = ::std::option::Option<
  unsafe extern "C" fn(
    arg1: HANDLE,
    arg2: *mut ::std::os::raw::c_void,
    arg3: DWORD,
    arg4: LPDWORD,
    arg5: LPOVERLAPPED,
  ) -> bool,
>;
extern "C" {
  pub fn SFileGetLocale() -> LCID;
}
extern "C" {
  pub fn SFileSetLocale(lcNewLocale: LCID) -> LCID;
}
extern "C" {
  pub fn SFileOpenArchive(
    szMpqName: *const TCHAR,
    dwPriority: DWORD,
    dwFlags: DWORD,
    phMpq: *mut HANDLE,
  ) -> bool;
}
extern "C" {
  pub fn SFileCreateArchive(
    szMpqName: *const TCHAR,
    dwCreateFlags: DWORD,
    dwMaxFileCount: DWORD,
    phMpq: *mut HANDLE,
  ) -> bool;
}
extern "C" {
  pub fn SFileCreateArchive2(
    szMpqName: *const TCHAR,
    pCreateInfo: PSFILE_CREATE_MPQ,
    phMpq: *mut HANDLE,
  ) -> bool;
}
extern "C" {
  pub fn SFileSetDownloadCallback(
    hMpq: HANDLE,
    DownloadCB: SFILE_DOWNLOAD_CALLBACK,
    pvUserData: *mut ::std::os::raw::c_void,
  ) -> bool;
}
extern "C" {
  pub fn SFileFlushArchive(hMpq: HANDLE) -> bool;
}
extern "C" {
  pub fn SFileCloseArchive(hMpq: HANDLE) -> bool;
}
extern "C" {
  pub fn SFileAddListFile(hMpq: HANDLE, szListFile: *const TCHAR) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn SFileSetCompactCallback(
    hMpq: HANDLE,
    CompactCB: SFILE_COMPACT_CALLBACK,
    pvUserData: *mut ::std::os::raw::c_void,
  ) -> bool;
}
extern "C" {
  pub fn SFileCompactArchive(hMpq: HANDLE, szListFile: *const TCHAR, bReserved: bool) -> bool;
}
extern "C" {
  pub fn SFileGetMaxFileCount(hMpq: HANDLE) -> DWORD;
}
extern "C" {
  pub fn SFileSetMaxFileCount(hMpq: HANDLE, dwMaxFileCount: DWORD) -> bool;
}
extern "C" {
  pub fn SFileGetAttributes(hMpq: HANDLE) -> DWORD;
}
extern "C" {
  pub fn SFileSetAttributes(hMpq: HANDLE, dwFlags: DWORD) -> bool;
}
extern "C" {
  pub fn SFileUpdateFileAttributes(hMpq: HANDLE, szFileName: *const ::std::os::raw::c_char)
    -> bool;
}
extern "C" {
  pub fn SFileOpenPatchArchive(
    hMpq: HANDLE,
    szPatchMpqName: *const TCHAR,
    szPatchPathPrefix: *const ::std::os::raw::c_char,
    dwFlags: DWORD,
  ) -> bool;
}
extern "C" {
  pub fn SFileIsPatchedArchive(hMpq: HANDLE) -> bool;
}
extern "C" {
  pub fn SFileHasFile(hMpq: HANDLE, szFileName: *const ::std::os::raw::c_char) -> bool;
}
extern "C" {
  pub fn SFileOpenFileEx(
    hMpq: HANDLE,
    szFileName: *const ::std::os::raw::c_char,
    dwSearchScope: DWORD,
    phFile: *mut HANDLE,
  ) -> bool;
}
extern "C" {
  pub fn SFileGetFileSize(hFile: HANDLE, pdwFileSizeHigh: LPDWORD) -> DWORD;
}
extern "C" {
  pub fn SFileSetFilePointer(
    hFile: HANDLE,
    lFilePos: LONG,
    plFilePosHigh: *mut LONG,
    dwMoveMethod: DWORD,
  ) -> DWORD;
}
extern "C" {
  pub fn SFileReadFile(
    hFile: HANDLE,
    lpBuffer: *mut ::std::os::raw::c_void,
    dwToRead: DWORD,
    pdwRead: LPDWORD,
    lpOverlapped: LPOVERLAPPED,
  ) -> bool;
}
extern "C" {
  pub fn SFileCloseFile(hFile: HANDLE) -> bool;
}
extern "C" {
  pub fn SFileGetFileInfo(
    hMpqOrFile: HANDLE,
    InfoClass: SFileInfoClass,
    pvFileInfo: *mut ::std::os::raw::c_void,
    cbFileInfo: DWORD,
    pcbLengthNeeded: LPDWORD,
  ) -> bool;
}
extern "C" {
  pub fn SFileGetFileName(hFile: HANDLE, szFileName: *mut ::std::os::raw::c_char) -> bool;
}
extern "C" {
  pub fn SFileFreeFileInfo(
    pvFileInfo: *mut ::std::os::raw::c_void,
    InfoClass: SFileInfoClass,
  ) -> bool;
}
extern "C" {
  pub fn SFileExtractFile(
    hMpq: HANDLE,
    szToExtract: *const ::std::os::raw::c_char,
    szExtracted: *const TCHAR,
    dwSearchScope: DWORD,
  ) -> bool;
}
extern "C" {
  pub fn SFileGetFileChecksums(
    hMpq: HANDLE,
    szFileName: *const ::std::os::raw::c_char,
    pdwCrc32: LPDWORD,
    pMD5: *mut ::std::os::raw::c_char,
  ) -> bool;
}
extern "C" {
  pub fn SFileVerifyFile(
    hMpq: HANDLE,
    szFileName: *const ::std::os::raw::c_char,
    dwFlags: DWORD,
  ) -> DWORD;
}
extern "C" {
  pub fn SFileVerifyRawData(
    hMpq: HANDLE,
    dwWhatToVerify: DWORD,
    szFileName: *const ::std::os::raw::c_char,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn SFileSignArchive(hMpq: HANDLE, dwSignatureType: DWORD) -> bool;
}
extern "C" {
  pub fn SFileVerifyArchive(hMpq: HANDLE) -> DWORD;
}
extern "C" {
  pub fn SFileFindFirstFile(
    hMpq: HANDLE,
    szMask: *const ::std::os::raw::c_char,
    lpFindFileData: *mut SFILE_FIND_DATA,
    szListFile: *const TCHAR,
  ) -> HANDLE;
}
extern "C" {
  pub fn SFileFindNextFile(hFind: HANDLE, lpFindFileData: *mut SFILE_FIND_DATA) -> bool;
}
extern "C" {
  pub fn SFileFindClose(hFind: HANDLE) -> bool;
}
extern "C" {
  pub fn SListFileFindFirstFile(
    hMpq: HANDLE,
    szListFile: *const TCHAR,
    szMask: *const ::std::os::raw::c_char,
    lpFindFileData: *mut SFILE_FIND_DATA,
  ) -> HANDLE;
}
extern "C" {
  pub fn SListFileFindNextFile(hFind: HANDLE, lpFindFileData: *mut SFILE_FIND_DATA) -> bool;
}
extern "C" {
  pub fn SListFileFindClose(hFind: HANDLE) -> bool;
}
extern "C" {
  pub fn SFileEnumLocales(
    hMpq: HANDLE,
    szFileName: *const ::std::os::raw::c_char,
    plcLocales: *mut LCID,
    pdwMaxLocales: LPDWORD,
    dwSearchScope: DWORD,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn SFileCreateFile(
    hMpq: HANDLE,
    szArchivedName: *const ::std::os::raw::c_char,
    FileTime: ULONGLONG,
    dwFileSize: DWORD,
    lcLocale: LCID,
    dwFlags: DWORD,
    phFile: *mut HANDLE,
  ) -> bool;
}
extern "C" {
  pub fn SFileWriteFile(
    hFile: HANDLE,
    pvData: *const ::std::os::raw::c_void,
    dwSize: DWORD,
    dwCompression: DWORD,
  ) -> bool;
}
extern "C" {
  pub fn SFileFinishFile(hFile: HANDLE) -> bool;
}
extern "C" {
  pub fn SFileAddFileEx(
    hMpq: HANDLE,
    szFileName: *const TCHAR,
    szArchivedName: *const ::std::os::raw::c_char,
    dwFlags: DWORD,
    dwCompression: DWORD,
    dwCompressionNext: DWORD,
  ) -> bool;
}
extern "C" {
  pub fn SFileAddFile(
    hMpq: HANDLE,
    szFileName: *const TCHAR,
    szArchivedName: *const ::std::os::raw::c_char,
    dwFlags: DWORD,
  ) -> bool;
}
extern "C" {
  pub fn SFileAddWave(
    hMpq: HANDLE,
    szFileName: *const TCHAR,
    szArchivedName: *const ::std::os::raw::c_char,
    dwFlags: DWORD,
    dwQuality: DWORD,
  ) -> bool;
}
extern "C" {
  pub fn SFileRemoveFile(
    hMpq: HANDLE,
    szFileName: *const ::std::os::raw::c_char,
    dwSearchScope: DWORD,
  ) -> bool;
}
extern "C" {
  pub fn SFileRenameFile(
    hMpq: HANDLE,
    szOldFileName: *const ::std::os::raw::c_char,
    szNewFileName: *const ::std::os::raw::c_char,
  ) -> bool;
}
extern "C" {
  pub fn SFileSetFileLocale(hFile: HANDLE, lcNewLocale: LCID) -> bool;
}
extern "C" {
  pub fn SFileSetDataCompression(DataCompression: DWORD) -> bool;
}
extern "C" {
  pub fn SFileSetAddFileCallback(
    hMpq: HANDLE,
    AddFileCB: SFILE_ADDFILE_CALLBACK,
    pvUserData: *mut ::std::os::raw::c_void,
  ) -> bool;
}
extern "C" {
  pub fn SCompImplode(
    pvOutBuffer: *mut ::std::os::raw::c_void,
    pcbOutBuffer: *mut ::std::os::raw::c_int,
    pvInBuffer: *mut ::std::os::raw::c_void,
    cbInBuffer: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn SCompExplode(
    pvOutBuffer: *mut ::std::os::raw::c_void,
    pcbOutBuffer: *mut ::std::os::raw::c_int,
    pvInBuffer: *mut ::std::os::raw::c_void,
    cbInBuffer: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn SCompCompress(
    pvOutBuffer: *mut ::std::os::raw::c_void,
    pcbOutBuffer: *mut ::std::os::raw::c_int,
    pvInBuffer: *mut ::std::os::raw::c_void,
    cbInBuffer: ::std::os::raw::c_int,
    uCompressionMask: ::std::os::raw::c_uint,
    nCmpType: ::std::os::raw::c_int,
    nCmpLevel: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn SCompDecompress(
    pvOutBuffer: *mut ::std::os::raw::c_void,
    pcbOutBuffer: *mut ::std::os::raw::c_int,
    pvInBuffer: *mut ::std::os::raw::c_void,
    cbInBuffer: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
extern "C" {
  pub fn SCompDecompress2(
    pvOutBuffer: *mut ::std::os::raw::c_void,
    pcbOutBuffer: *mut ::std::os::raw::c_int,
    pvInBuffer: *mut ::std::os::raw::c_void,
    cbInBuffer: ::std::os::raw::c_int,
  ) -> ::std::os::raw::c_int;
}
