// Copyright (C) 2015-2017 Florian Wilkens
//
// Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
// associated documentation files (the "Software"), to deal in the Software without restriction,
// including without limitation the rights to use, copy, modify, merge, publish, distribute,
// sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
// NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES
// OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

//! FFI wrappers for the Linux Pluggable Authentication Modules (PAM)
//!
//! This crate provides easy access to the Linux-PAM. Both raw and wrapped versions
//! of the common functions from `libpam.so` are provided as well as types used in
//! these functions.
//!
//! Note: Not all functions are fully supported yet. For now mainly the ones required
//! for authentication with PAM rather than writing a custom PAM module are provided.
extern crate libc;

pub use types::*;
pub use wrapped::*;

pub mod raw;
#[allow(non_camel_case_types, match_same_arms)]
pub mod types;
pub mod wrapped;
