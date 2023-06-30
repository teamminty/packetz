#[cfg(all(feature = "server", not(target_arch = "wasm32")))]
pub mod server;
#[cfg(all(feature = "server", target_arch = "wasm32"))]
compile_error!("Packetz server is not supported with wasm.");
#[cfg(all(feature = "client", target_arch = "wasm32"))]
compile_error!("Packetz client is not supported with wasm.");
#[cfg(feature = "client")]
pub mod client;
pub mod packet;
#[cfg(test)]
mod tests;