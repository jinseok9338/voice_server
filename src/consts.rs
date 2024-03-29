use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

pub struct CheckPath<'a> {
    pub path: &'a str,
    pub method: &'a str,
    pub skip_check: bool,
}
pub(crate) const AUTH_MIDDLEWARE_CHECK_PATHS: &[CheckPath<'static>] = &[
    CheckPath {
        path: r"^/auth/token$",
        method: "POST",
        skip_check: true,
    },
    CheckPath {
        path: r"^/auth/token/reissue$",
        method: "PUT",
        skip_check: true,
    },
    CheckPath {
        path: r"^/auth/signup$",
        method: "POST",
        skip_check: true,
    },
    CheckPath {
        path: r"^/ws/.*$",
        method: "GET",
        skip_check: true,
    },
    CheckPath {
        path: r"^/swagger-ui/.*$",
        method: "GET",
        skip_check: true,
    },
    CheckPath {
        path: r"^/swagger-ui",
        method: "GET",
        skip_check: true,
    },
    CheckPath {
        path: r"^/api-docs/openapi.json",
        method: "GET",
        skip_check: true,
    },
    CheckPath {
        path: r"^/favicon.ico",
        method: "GET",
        skip_check: true,
    },
    // Add more excluded paths as needed
];

pub type Conn = PooledConnection<ConnectionManager<PgConnection>>;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
