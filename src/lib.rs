extern crate lazy_static;

#[cfg(windows)]
extern crate winapi;

#[cfg(unix)]
extern crate libc;

mod backend;
mod style;
mod terminal;
mod terminal_backend;
mod os;
mod key;

pub use terminal::{
    terminal,
    Terminal,
};

pub use style::{
    Color,
    Style,
};

pub use key::Key;