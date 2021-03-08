use rustc_version::{version_meta, Channel};

fn main() {
    if let Ok(version_meta) = version_meta() {
        if version_meta.channel == Channel::Nightly {
            println!("cargo:rustc-cfg=nightly");
        }
    }
}
