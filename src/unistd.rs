#![cfg(target_os = "linux")]

use std::ptr;
use std::c_str::{CString, ToCStr};
use std::path::Path;
use libc::{c_char};
use fcntl::{Fd, OFlag};
use syscall::{syscall, SysPivotRoot};
use {SysResult, SysError};

mod ffi {
    use libc::{c_char, c_int};

    extern {
        pub fn dup(oldfd: c_int) -> c_int;

        pub fn dup2(oldfd: c_int, newfd: c_int) -> c_int;

        pub fn dup3(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int;

        // change working directory
        // doc: http://man7.org/linux/man-pages/man2/chdir.2.html
        pub fn chdir(path: *const c_char) -> c_int;

        // execute program
        // doc: http://man7.org/linux/man-pages/man2/execve.2.html
        pub fn execve(filename: *const c_char, argv: *const *const c_char, envp: *const *const c_char) -> c_int;
    }
}

#[inline]
pub fn dup(oldfd: Fd) -> SysResult<Fd> {
    let res = unsafe { ffi::dup(oldfd) };

    if res < 0 {
        return Err(SysError::last());
    }

    Ok(res)
}

#[inline]
pub fn dup2(oldfd: Fd, newfd: Fd) -> SysResult<Fd> {
    let res = unsafe { ffi::dup2(oldfd, newfd) };

    if res < 0 {
        return Err(SysError::last());
    }

    Ok(res)
}

#[inline]
pub fn dup3(oldfd: Fd, newfd: Fd, flags: OFlag) -> SysResult<Fd> {
    let res = unsafe { ffi::dup3(oldfd, newfd, flags.bits()) };

    if res < 0 {
        return Err(SysError::last());
    }

    Ok(res)
}

#[inline]
pub fn chdir<S: ToCStr>(path: S) -> SysResult<()> {
    let path = path.to_c_str();
    let res = unsafe { ffi::chdir(path.as_ptr()) };

    if res != 0 {
        return Err(SysError::last());
    }

    return Ok(())
}

#[inline]
pub fn execve(filename: CString, args: &[CString], env: &[CString]) -> SysResult<()> {
    let mut args_p: Vec<*const c_char> = args.iter().map(|s| s.as_ptr()).collect();
    args_p.push(ptr::null());

    let mut env_p: Vec<*const c_char> = env.iter().map(|s| s.as_ptr()).collect();
    env_p.push(ptr::null());

    let res = unsafe {
        ffi::execve(filename.as_ptr(), args_p.as_ptr(), env_p.as_ptr())
    };

    if res != 0 {
        return Err(SysError::last());
    }

    // Should never reach here
    Ok(())
}

pub fn pivot_root(new_root: &Path, put_old: &Path) -> SysResult<()> {
    let new_root = new_root.to_c_str();
    let put_old = put_old.to_c_str();

    let res = unsafe {
        syscall(SysPivotRoot, new_root.as_ptr(), put_old.as_ptr())
    };

    if res != 0 {
        return Err(SysError::last());
    }

    Ok(())
}
