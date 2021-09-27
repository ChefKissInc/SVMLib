#![no_std]
#![deny(
    warnings,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![feature(asm)]
#![feature(derive_default_enum)]

pub mod vmcb;
