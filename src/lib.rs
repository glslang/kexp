//! Windows kernel exploitation primitives.
//!
//! The debugger-side half of this crate — the DbgEng bindings and the pool and heap
//! walkers built on them — left for `dbgscope`. What remains is what runs on the
//! target rather than in the debugger: shellcode, ROP gadget search, process
//! injection, driver IOCTL plumbing and pool spraying.

pub mod process;
pub mod rop;
pub mod shellcode;
pub mod spray;
pub mod util;
pub mod win32k;
