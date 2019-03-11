use find_winsdk::{SdkInfo, SdkVersion};
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=native={}", get_lib_path().to_string_lossy());
    println!("cargo:rustc-link-lib=dylib=dismapi");
}

fn get_lib_path() -> PathBuf {
    let mut path = SdkInfo::find(SdkVersion::V10_0)
        .unwrap()
        .expect("Could not find Windows 10 kits root")
        .installation_folder()
        .to_path_buf();

    path.push("Assessment and Deployment Kit\\Deployment Tools\\SDKs\\DismApi\\Lib");

    let arch = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86" => "x86",
        "x86_64" => "amd64",
        "arm" => "arm",
        "aarch64" => "arm64",
        _ => panic!("Unsupported target architecture")
    };

    path.push(arch);

    path
}
