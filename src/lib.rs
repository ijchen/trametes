//! A free and open source raster graphics editor written in Rust

#![warn(missing_docs, missing_debug_implementations, unused_unsafe)]
#![deny(unsafe_code)] // This can be revisited, but for now we don't need unsafe
#![deny(
    unsafe_op_in_unsafe_fn,
    clippy::undocumented_unsafe_blocks,
    clippy::multiple_unsafe_ops_per_block
)]

// TODO do we *really* need a lib, and if we do, do we really need a separate
// `app` mod? This feels overcomplicated

mod app;
mod fileio;
mod tools;
mod ui;
pub use app::TrametesApp;
