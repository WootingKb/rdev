mod common;
mod display;
#[cfg(feature = "unstable_grab")]
mod grab;
mod keyboard;
mod keycodes;
mod listen;
mod simulate;

pub use crate::windows::display::display_size;
#[cfg(feature = "unstable_grab")]
pub use crate::windows::grab::grab;
pub use crate::windows::keyboard::Keyboard;
pub use crate::windows::listen::listen;
pub use crate::windows::simulate::simulate;

// types not defined by windows-sys
#[allow(clippy::upper_case_acronyms)]
pub type DWORD = u32;
#[allow(clippy::upper_case_acronyms)]
pub type WORD = u16;
#[allow(clippy::upper_case_acronyms)]
pub type LONG = i32;
#[allow(clippy::upper_case_acronyms)]
pub static MOUSE_FORWARD: u8 = 0x01;
#[allow(clippy::upper_case_acronyms)]
pub static MOUSE_BACKWARD: u8 = 0x02;
