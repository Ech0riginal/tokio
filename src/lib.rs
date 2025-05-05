pub use _tokio::*;

#[cfg(feature = "axum")]
pub use axum;

#[cfg(feature = "bytes")]
pub use bytes;

#[cfg(feature = "hyper")]
pub use hyper;

#[cfg(feature = "io-uring")]
pub use io_uring;

#[cfg(feature = "postgres")]
pub mod postgres {
    pub use postgres_types as types;
    pub use tokio_postgres::*;
}

#[cfg(feature = "prost")]
pub use prost;

#[cfg(feature = "rustls")]
pub mod rustls {
    pub use ::rustls::Error;
    pub use tokio_rustls::*;
}

#[cfg(feature = "util")]
pub use tokio_util as util;

#[cfg(feature = "stream")]
pub use tokio_stream as stream;

#[cfg(feature = "stream")]
pub mod pin {
    pub use pin_project as project;
    pub use _tokio::pin;
}

#[cfg(feature = "tracing")]
pub mod tracing {
    pub use tracing::*;
    pub use tracing_indicatif as indicatif;
    pub use tracing_subscriber as subscriber;
}
