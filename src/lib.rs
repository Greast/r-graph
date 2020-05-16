#![feature(type_alias_impl_trait)]
pub mod dev;
pub mod extended;
pub mod wrapper;

pub mod graph {
    pub use crate::dev::simple::Simple;
}
