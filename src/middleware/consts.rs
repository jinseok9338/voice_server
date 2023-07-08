pub struct CheckPath<'a> {
    pub path: &'a str,
    pub method: &'a str,
    pub skip_check: bool,
}

// Update the AUTH_MIDDLEWARE_EXCLUDE_PATHS to use ExcludedPath struct
pub(crate) const AUTH_MIDDLEWARE_CHECK_PATHS: &[CheckPath<'static>] = &[
    CheckPath {
        path: "/auth/token",
        method: "POST",
        skip_check: true,
    },
    CheckPath {
        path: "/auth/token",
        method: "PUT",
        skip_check: false,
    },
    CheckPath {
        path: "/users/me",
        method: "GET",
        skip_check: false,
    },
    CheckPath {
        path: "/users/me",
        method: "PUT",
        skip_check: false,
    },
    // Add more excluded paths as needed
];
