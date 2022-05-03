use std::env;
use std::process::Command;

fn main() {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    env::set_current_dir(cargo_manifest_dir).unwrap();
    Command::new("clang")
        .args(["-c", "-o", "foo.o", "foo.c", "-fPIC"])
        .output()
        .unwrap();
    Command::new("ar")
        .args(["r", "libmystatic.a", "foo.o"])
        .output()
        .unwrap();
    println!("cargo:rustc-link-search=native=.");
    println!("cargo:rustc-link-lib=static=mystatic");
}
