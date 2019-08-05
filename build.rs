extern crate rustc_version;
use rustc_version::{version_meta, Channel};

fn main() {
    if let Ok(version_meta) = version_meta() {
        match version_meta.channel {
            Channel::Nightly => println!("cargo:rustc-cfg=nightly"),
            _ => {}
        }
    }
}
