extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("src/statx.c")
        .include("src")
        .compile("libstatx.a");
    // println!("cargo:rustc-link-lib=static=statx");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindgens = vec![
        ("/usr/include/linux/stat.h", "stat.rs"),
        ("/usr/include/fcntl.h", "fcntl.rs"),
    ];

    for (c, rs) in bindgens {
        bindgen::Builder::default()
            .header(c)
            .generate()
            .map_err(|e| panic!(format!("Unable to generate: {} for {}: {:?} ", rs, c, e)))
            .unwrap()
            .write_to_file(out_path.join(rs))
            .map_err(|e| panic!(format!("Couldn't write {}: {} !", rs, e)))
            .unwrap()
    }
}
