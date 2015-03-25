use libc::{c_char, c_int};

use types::*;
use raw::*;

#[inline]
pub unsafe fn start(service: *const c_char, user: *const c_char,
    conversation: *const PamConversation, handle: *mut *mut PamHandle) -> PamReturnCode {
    PamReturnCode::from_i32(pam_start(service, user, conversation, handle as *mut *const PamHandle))
}

#[inline]
pub unsafe fn end(handle: *mut PamHandle, status: c_int) -> PamReturnCode {
    PamReturnCode::from_i32(pam_end(handle, status))
}

#[inline]
pub unsafe fn authenticate(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    PamReturnCode::from_i32(pam_authenticate(handle, flags as i32) as i32)
}

#[inline]
pub unsafe fn acct_mgmt(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    PamReturnCode::from_i32(pam_acct_mgmt(handle, flags as i32) as i32)
}

#[inline]
pub unsafe fn setcred(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    PamReturnCode::from_i32(pam_setcred(handle, flags as i32) as i32)
}

#[inline]
pub unsafe fn open_session(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    PamReturnCode::from_i32(pam_open_session(handle, flags as i32) as i32)
}

#[inline]
pub unsafe fn close_session(handle: *mut PamHandle, flags: PamFlag) -> PamReturnCode {
    PamReturnCode::from_i32(pam_close_session(handle, flags as i32) as i32)
}
