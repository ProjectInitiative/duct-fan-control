//! This build script copies the `memory.x` file from the crate root into
//! a directory where the linker can always find it at build time.
//! For many projects this is optional, as the linker always searches the
//! project root directory -- wherever `Cargo.toml` is. However, if you
//! are using a workspace or have a more complicated build setup, this
//! build script becomes required. Additionally, by requesting that
//! Cargo re-run the build script whenever `memory.x` is changed,
//! updating `memory.x` ensures a rebuild of the application with the
//! new memory settings.

use std::env;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

fn main() -> io::Result<()> {
    //load environment variables from a .env file
    dotenv::dotenv().ok();
    // Retrieve the environment variables you need
    let encryption_key_value = env::var("ENCRYPTION_KEY").expect("ENCRYPTION_KEY not set");

    // Generate Rust code to include in the application
    let code = format!(
        r#"pub const ENCRYPTION_KEY: &[u8] = b"{}";"#,
        encryption_key_value
    );

    // Write the generated code to a file
    let mut output_file = File::create("src/secret.rs")?;
    output_file.write_all(code.as_bytes())?;

    // Put `memory.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());

    // By default, Cargo will re-run a build script whenever
    // any file in the project changes. By specifying `memory.x`
    // here, we ensure the build script is only re-run when
    // `memory.x` is changed.
    println!("cargo:rerun-if-changed=memory.x");

    println!("cargo:rustc-link-arg-bins=--nmagic");
    println!("cargo:rustc-link-arg-bins=-Tlink.x");
    println!("cargo:rustc-link-arg-bins=-Tlink-rp.x");
    println!("cargo:rustc-link-arg-bins=-Tdefmt.x");
    Ok(())
}
