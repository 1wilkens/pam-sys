#include <security/pam_appl.h>

#ifdef __linux__
#include <security/pam_client.h>
#include <security/pam_ext.h>
#include <security/pam_filter.h>
#include <security/pam_modutil.h>
#include <security/pam_misc.h>
#include <security/pam_modules.h>
#endif

#ifdef __FreeBSD__
#include <security/pam_constants.h>
#include <security/pam_mod_misc.h>
#include <security/pam_modules.h>
#include <security/pam_types.h>
#endif
