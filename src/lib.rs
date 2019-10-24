//! FuseMT -- A higher-level FUSE (Filesystem in Userspace) interface and wrapper around the
//! low-level `rust-fuse` library that makes implementing a filesystem a bit easier.
//!
//! FuseMT translates inodes to paths and dispatches I/O operations to multiple threads, and
//! simplifies some details of filesystem implementation, for example: splitting the `setattr` call
//! into multiple separate operations, and simplifying the `readdir` call so that filesystems don't
//! need to deal with pagination.
//!
//! To implement a filesystem, implement the `FilesystemMT` trait. Not all functions in it need to
//! be implemented -- the default behavior is to return `ENOSYS` ("Function not implemented"). For
//! example, a read-only filesystem can skip implementing the `write` call and many others.

//
// Copyright (c) 2016-2019 by William R. Fraser
//

pub extern crate fuse;
extern crate libc;
extern crate threadpool;
extern crate time;

#[macro_use]
extern crate log;

mod directory_cache;
mod fusemt;
mod inode_table;
mod types;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use fuse::{FileType, mount, spawn_mount};

pub use fusemt::*;
pub use types::*;