//! 
//! Common test utilities
//!

use std::env;
use std::path::PathBuf;


pub fn asset_path(asset_path: &str) -> PathBuf {
    let mut path = PathBuf::new();
    // start with CWD
    path.push(env::current_dir().unwrap().as_path());
    // test assets prefix
    path.push("assets/test/");
    // path of this specific asset
    path.push(asset_path);
    return path.clone();
}
