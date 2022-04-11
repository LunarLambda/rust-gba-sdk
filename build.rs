/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

fn var(s: &str) -> String { std::env::var(s).unwrap() }

fn assemble_and_link<P: AsRef<std::path::Path>>(s: P) -> (String, String) {
    let s = s.as_ref();

    // blah/beans.s -> $OUT_DIR/beans.o
    let out_dir = var("OUT_DIR");
    let mut o = std::path::PathBuf::from(&out_dir);
    o.push(s.file_stem().unwrap());
    o.set_extension("o");

    let out =
        std::process::Command::new("arm-none-eabi-as")
        .arg("-mcpu=arm7tdmi")
        .arg("-mthumb-interwork")
        .arg("-o")
        .arg(&o)
        .arg(&s)
        .output()
        .expect("failed to run arm-none-eabi-as");

    if !out.status.success() {
        panic!("{}", String::from_utf8_lossy(&out.stderr));
    }

    println!("cargo:rerun-if-changed={}", s.display());
    println!("cargo:rustc-link-arg-examples={}", o.display());
    println!("cargo:rustc-link-search={}", &out_dir);

    (out_dir, o.display().to_string())
}

fn main() {
    let manifest_dir    = var("CARGO_MANIFEST_DIR");

    let minrt_path      = format!("{}/gba-minrt", manifest_dir);
    let link_script     = "rom.ld";

    let crt0            = format!("{}/rt/crt0.s", minrt_path);

    let (search, arg) = assemble_and_link(crt0);

    // Linker script stuff
    println!("cargo:rustc-link-search={}/rt", minrt_path);
    println!("cargo:rustc-link-arg-examples=-T{}", link_script);

    // Downstream stuff
    println!("cargo:link-search={}:{}", search, minrt_path);
    println!("cargo:link-args={}:-T{}", arg, link_script);
}

