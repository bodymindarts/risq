#[cfg(not(target_os = "android"))]
mod client;
mod graphql;
mod server;

#[cfg(not(target_os = "android"))]
pub use client::GrqphQLClient as Client;
#[cfg(not(target_os = "android"))]
pub use client::WithQueryFields;
pub use server::listen;
