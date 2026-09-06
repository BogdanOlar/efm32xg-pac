use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    if env::var_os("CARGO_FEATURE_RT").is_some() {
        let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

        let device_x = if env::var_os("CARGO_FEATURE_EFM32PG1B").is_some() {
            include_bytes!("device-efm32pg1b.x").as_slice()
        } else if env::var_os("CARGO_FEATURE_EFM32GG11").is_some() {
            include_bytes!("device-efm32gg11.x").as_slice()
        } else {
            panic!("No chip feature enabled. Enable either `efm32pg1b` or `efm32gg11`.");
        };

        File::create(out.join("device.x"))
            .unwrap()
            .write_all(device_x)
            .unwrap();
        println!("cargo:rustc-link-search={}", out.display());
        println!("cargo:rerun-if-changed=device-efm32pg1b.x");
        println!("cargo:rerun-if-changed=device-efm32gg11.x");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
