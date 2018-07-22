#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//! [http://man7.org/linux/man-pages/man2/statx.2.html](http://man7.org/linux/man-pages/man2/statx.2.html)
//!
//! [https://raw.githubusercontent.com/torvalds/linux/master/samples/statx/test-statx.c](https://raw.githubusercontent.com/torvalds/linux/master/samples/statx/test-statx.c)
//!
//! [https://github.com/torvalds/linux/blob/master/include/uapi/linux/stat.h](https://github.com/torvalds/linux/blob/master/include/uapi/linux/stat.h)
//!
extern crate chrono;
use chrono::NaiveDateTime;

use std::ffi::CString;
use std::io::{self, Error};
use std::mem;
use std::os::raw::c_char;

pub mod stat {
    use super::*;

    include!(concat!(env!("OUT_DIR"), "/stat.rs"));

    #[link(name = "statx")]
    extern "C" {
        pub fn statxf(
            dirfd: i32,
            filename: *const c_char,
            flags: u32,
            mask: u32,
            stat: *mut statx,
        ) -> i32;
    }
}

pub mod fcntl {
    include!(concat!(env!("OUT_DIR"), "/fcntl.rs"));
}

use fcntl::{AT_FDCWD, AT_SYMLINK_NOFOLLOW};
use stat::{statxf, STATX_ALL};

pub fn statx<S: Into<Vec<u8>>>(filepath: S, follow_symlink: bool) -> io::Result<Statx> {
    let cstring = CString::new(filepath.into()).unwrap();
    let csp = cstring.as_ptr();
    let mut flags = AT_SYMLINK_NOFOLLOW;
    if !follow_symlink {
        flags &= !AT_SYMLINK_NOFOLLOW;
    };
    let mut stat: stat::statx = unsafe { mem::uninitialized() };
    if 0 > unsafe { statxf(AT_FDCWD, csp, flags, STATX_ALL, &mut stat as *mut _) } {
        Err(Error::last_os_error())
    } else {
        Ok(stat.into())
    }
}

#[derive(Debug)]
pub struct Statx {
    pub stx_mask: u32,
    pub stx_blksize: u32,
    pub stx_attributes: u64,
    pub stx_nlink: u32,
    pub stx_uid: u32,
    pub stx_gid: u32,
    pub stx_mode: u16,
    pub __spare0: [u16; 1usize],
    pub stx_ino: u64,
    pub stx_size: u64,
    pub stx_blocks: u64,
    pub stx_attributes_mask: u64,
    pub stx_atime: NaiveDateTime,
    pub stx_btime: NaiveDateTime,
    pub stx_ctime: NaiveDateTime,
    pub stx_mtime: NaiveDateTime,
    pub stx_rdev_major: u32,
    pub stx_rdev_minor: u32,
    pub stx_dev_major: u32,
    pub stx_dev_minor: u32,
    pub __spare2: [u64; 14usize],
}

impl From<stat::statx> for Statx {
    fn from(c: stat::statx) -> Self {
        Self {
            stx_mask: c.stx_mask,
            stx_blksize: c.stx_blksize,
            stx_attributes: c.stx_attributes,
            stx_nlink: c.stx_nlink,
            stx_uid: c.stx_uid,
            stx_gid: c.stx_gid,
            stx_mode: c.stx_mode,
            __spare0: c.__spare0,
            stx_ino: c.stx_ino,
            stx_size: c.stx_size,
            stx_blocks: c.stx_blocks,
            stx_attributes_mask: c.stx_attributes_mask,
            stx_atime: NaiveDateTime::from_timestamp(c.stx_atime.tv_sec, c.stx_atime.tv_nsec),
            stx_btime: NaiveDateTime::from_timestamp(c.stx_btime.tv_sec, c.stx_btime.tv_nsec),
            stx_ctime: NaiveDateTime::from_timestamp(c.stx_ctime.tv_sec, c.stx_ctime.tv_nsec),
            stx_mtime: NaiveDateTime::from_timestamp(c.stx_mtime.tv_sec, c.stx_mtime.tv_nsec),
            stx_rdev_major: c.stx_rdev_major,
            stx_rdev_minor: c.stx_rdev_minor,
            stx_dev_major: c.stx_dev_major,
            stx_dev_minor: c.stx_dev_minor,
            __spare2: c.__spare2,
        }
    }
}
