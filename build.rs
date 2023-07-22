extern crate bindgen;
extern crate cc;
use std::path::PathBuf;

fn handle_ui_coordinate_bindings() {
    println!("cargo:rerun-if-changed=src/ui/ui_coordinates.c");

    // Generate the bindings.
    let bindings = bindgen::Builder::default()
        .header("src/ui/ui_coordinates.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from("src/ui");
    bindings
        .write_to_file(out_path.join("ui_coordinates_bindings.rs"))
        .expect("Couldn't write bindings!");

    // Compile the C code.
    cc::Build::new()
        .file("src/ui/ui_coordinates.c")
        .compile("ui_coordinates.a");
}

#[cfg(target_os = "macos")]
fn main() {
    // Only rerun on changes to the Obj-C code.
    println!("cargo:rerun-if-changed=src/ui/ui_darwin.h");
    println!("cargo:rerun-if-changed=src/ui/ui_darwin.m");

    // Handle UI coordinates.
    handle_ui_coordinate_bindings();

    // Generate the bindings.
    let bindings = bindgen::Builder::default()
        .header("src/ui/ui_darwin.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from("src/ui");
    bindings
        .write_to_file(out_path.join("ui_darwin_bindings.rs"))
        .expect("Couldn't write bindings!");

    // Compile the Obj-C code.
    cc::Build::new()
        .file("src/ui/ui_darwin.m")
        .compile("ui_darwin.a");

    // Tell rust to add -framework Cocoa to the linker flags
    println!("cargo:rustc-link-lib=framework=Cocoa");
}

#[cfg(not(target_os = "macos"))]
fn main() {
    handle_ui_coordinate_bindings();
}
