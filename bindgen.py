#! /usr/bin/env python

import os

OUTPUT_DIR = 'src'

WRAPPER_LINUXPAM = 'wrapper-linuxpam.h'
WRAPPER_OPENPAM = 'wrapper-openpam.h'

SHARED_FLAGS = [
	# Use libc for c-types 
    "--ctypes-prefix 'libc'",

	# pam_handle_t is opaque
	"--opaque-type 'pam_handle_t'",

	# Block varargs functions and related types for now
	# TODO: find a nice solution for this
	"--blocklist-type 'va_list'",
	"--blocklist-type '__va_list'",
	"--blocklist-type '__builtin_va_list'",
	"--blocklist-type '__gnuc_va_list'",
	"--blocklist-type '__va_list_tag'",

	"--blocklist-function 'pam_v.*'",
	"--blocklist-function 'pam_syslog'",
	"--blocklist-function 'pam_prompt'",

    # Allow all PAM constants
	"--allowlist-var 'PAM_.*'",
    # Allow all PAM functions
	"--allowlist-function 'pam_.*'",
	"--blocklist-function 'pam_sm_*'",
]

LINUXPAM_FLAGS = [
    # Tell cargo to invalidate the built crate whenever the wrapper changes
    # Set macro constants to signed int, as all functions that accept these
    # constants use signed int as the parameter type
    "--default-macro-constant-type signed",

    # Use libc types so our signatures are slightly nicer
    "--raw-line 'use libc::{uid_t, gid_t, group, passwd, spwd};'",
    "--blocklist-type '.*gid_t'",
    "--blocklist-type '.*uid_t'",
    "--blocklist-type 'group'",
    "--blocklist-type 'passwd'",
    "--blocklist-type 'spwd'",
]

OPENPAM_FLAGS = [
    # Use libc types so our signatures are slightly nicer
    "--raw-line 'use libc::passwd;'",
    "--blocklist-type 'passwd'",

    "--allowlist-var 'OPENPAM_.*'",
    "--allowlist-function 'openpam_.*'",
]

LINUXPAM_INCLUDES = [
    "linux-pam/libpam/include",
    "linux-pam/libpamc/include",
]

OPENPAM_INCLUDES = [
    "OpenPAM/include",
]

def run_bindgen(wrapper: str, output: str, flags, includes):
    print(f"Creating '{OUTPUT_DIR}/{output}'...")

    workspace_dir = os.path.dirname(os.path.realpath(__file__))

    wrapper = f'{workspace_dir}/{wrapper}'
    output = f'{workspace_dir}/{OUTPUT_DIR}/{output}'

    clang_args = ' '.join(map(
        lambda inc: f'-I{workspace_dir}/{inc}',
        includes,
    ))

    combined_flags = ''
    combined_flags += ' '.join(SHARED_FLAGS)
    combined_flags += ' '
    combined_flags += ' '.join(flags)

    cmd = f"bindgen '{wrapper}' -o '{output}' {combined_flags} -- {clang_args}"

    print(cmd)
    os.system(cmd)

def main():
    run_bindgen(
        WRAPPER_LINUXPAM,
        'linux_pam.rs',
        LINUXPAM_FLAGS,
        LINUXPAM_INCLUDES,
    )
    run_bindgen(
        WRAPPER_OPENPAM,
        'openpam.rs',
        OPENPAM_FLAGS,
        OPENPAM_INCLUDES,
    )

if __name__ == '__main__':
    main()