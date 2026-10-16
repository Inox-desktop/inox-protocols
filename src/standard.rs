//! Curated standard Wayland protocol bindings.

/// Core `wl_*` protocol interfaces.
pub mod core {
    /// Core protocol interfaces, client side (from `wayland-client`).
    pub use wayland_client::protocol as client;
    /// Core protocol interfaces, server side (from `wayland-server`).
    pub use wayland_server::protocol as server;
}
