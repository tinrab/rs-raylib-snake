use std::{error::Error, path::PathBuf, process::Command};

const RAYLIB_DIR: &str = "raylib";
const RAYLIB_OUT_DIR: &str = "raylib/zig-out";

fn main() -> Result<(), Box<dyn Error>> {
    let _ = Command::new("zig")
        .arg("build")
        .current_dir(RAYLIB_DIR)
        .status()?;

    // Generate bindings
    {
        let builder = bindgen::Builder::default()
            .header(
                PathBuf::from(RAYLIB_OUT_DIR)
                    .join("include")
                    .join("raylib.h")
                    .to_str()
                    .unwrap(),
            )
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));

        let bindings = builder.generate()?;

        let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
        bindings.write_to_file(out_path.join("bindings.rs"))?;
    }

    println!("cargo:rustc-link-search=native={RAYLIB_OUT_DIR}/lib");
    println!("cargo:rustc-link-lib=static=raylib");

    Ok(())
}
