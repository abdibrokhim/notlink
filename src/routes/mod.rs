// src/routes/mod.rs
mod shorten;
mod redirect;
mod wcreate;
mod wupdate;
mod expire;
mod getshorturls;

pub use shorten::*;
pub use redirect::*;
pub use wcreate::*;
pub use wupdate::*;
pub use expire::*;
pub use getshorturls::*;