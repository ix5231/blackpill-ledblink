use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

fn main() -> Result<(), Box<Error>> {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    println!("cargo:rustc-link-search={}", out.display());

    File::create(out.join("link.x"))?
        .write_all(include_bytes!("link.x"))?;

    println!("cargo:rerun-if-changed=link.x");

    Ok(())
}
