#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[repr(C)]
pub enum DRESULT {
    RES_OK = 0,     /* 0: Successful */
    RES_ERROR,      /* 1: R/W Error */
    RES_WRPRT,      /* 2: Write Protected */
    RES_NOTRDY,     /* 3: Not Ready */
    RES_PARERR      /* 4: Invalid Parameter */
}

use DRESULT::*;

pub type DSTATUS = u8;

/// User provided fatfs methods.
pub trait FatfsDiskHandler : Send {
    fn disk_status(&mut self, _pdrv: BYTE) -> DSTATUS { 0 as DSTATUS }

    fn disk_initialize(&mut self, pdrv: BYTE) -> DSTATUS { disk_status(pdrv) }

    fn disk_read(&mut self, pdrv: BYTE, buf: *mut BYTE, sector: DWORD, count: UINT) -> DRESULT;

    fn disk_write(&mut self, pdrv: BYTE, buf: *const BYTE, sector: DWORD, count: UINT) -> DRESULT;

    fn disk_ioctl(&mut self, _pdrv: BYTE, _cmd: BYTE, _buf: *mut libc::c_void) -> DRESULT { RES_PARERR as DRESULT }
}

lazy_static::lazy_static! {
    static ref DISK_HANDLER: std::sync::Mutex<Option<Box<dyn FatfsDiskHandler>>> = std::sync::Mutex::new(None);
}

/// Register user-provided fatfs functions. All fatfs functions will panic if this is not called.
pub unsafe fn register_disk_handler(handler: impl FatfsDiskHandler + 'static) {
    *DISK_HANDLER.lock().unwrap() = Some(Box::new(handler));
}

#[no_mangle]
pub extern fn disk_status(pdrv: BYTE) -> DSTATUS { DISK_HANDLER.lock().unwrap().as_mut().unwrap().disk_status(pdrv) }

#[no_mangle]
pub extern fn disk_initialize(pdrv: BYTE) -> DSTATUS { DISK_HANDLER.lock().unwrap().as_mut().unwrap().disk_initialize(pdrv) }

#[no_mangle]
pub extern fn disk_read(pdrv: BYTE, buf: *mut BYTE, sector: DWORD, count: UINT) -> DRESULT {
    DISK_HANDLER.lock().unwrap().as_mut().unwrap().disk_read(pdrv, buf, sector, count)
}

#[no_mangle]
pub extern fn disk_write(pdrv: BYTE, buf: *const BYTE, sector: DWORD, count: UINT) -> DRESULT {
    DISK_HANDLER.lock().unwrap().as_mut().unwrap().disk_write(pdrv, buf, sector, count)
}

#[no_mangle]
pub extern fn disk_ioctl(pdrv: BYTE, cmd: BYTE, buf: *mut libc::c_void) -> DRESULT {
    DISK_HANDLER.lock().unwrap().as_mut().unwrap().disk_ioctl(pdrv, cmd, buf)
}
