extern crate cmake;

use std::env;

fn main() {
    // Gets StormLib source path from env STORMLIB_DIR
    let stormlib_path = env::var("STORMLIB_DIR").unwrap_or("../../deps/StormLib".to_string());

    println!("cargo:rerun-if-changed={}", stormlib_path);

    // Builds StormLib using cmake
    let dst = cmake::Config::new(&stormlib_path)
        .define("BUILD_SHARED_LIBS", "OFF")
        .build();

    let lib = dst.join("lib");

    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-lib=z");
        println!("cargo:rustc-link-lib=bz2");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }

    println!("cargo:rustc-link-search=native={}", lib.display());
    println!("cargo:rustc-link-lib=static=storm");
}
