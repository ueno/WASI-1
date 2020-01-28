use std::{env, fs::File, io::Write, path::PathBuf};

fn main() {
    let out_path: PathBuf = env::var_os("OUT_DIR").unwrap().into();
    let mut f = File::create(out_path.join("lib_generated.rs")).unwrap();
    const WASI_EPHEMERAL_WITX: &str =
        "../generate-raw/WASI/phases/ephemeral/witx";
    let witx_path: PathBuf = env::var_os("WASI_EPHEMERAL_WITX")
        .unwrap_or_else(|| WASI_EPHEMERAL_WITX.into())
        .into();
    if !witx_path.is_dir() {
        panic!("{:?} is not a directory", witx_path);
    }
    let mut paths = Vec::new();
    for entry in witx_path.read_dir().expect("failed to read directory") {
        if let Ok(entry) = entry {
            let path = entry.path();
            let name = path.file_name().unwrap().to_str().unwrap();
            if name.ends_with(".witx") {
                paths.push(path);
            }
        }
    }
    let out = generate_raw::generate(&paths);
    write!(f, "{}", out).unwrap();
    println!("cargo:rerun-if-env-changed=WASI_EPHEMERAL_WITX");
    println!("cargo:rerun-if-changed={}", witx_path.display());
    println!(
        "cargo:rerun-if-changed={}",
        witx_path.with_file_name("typenames.witx").display(),
    );
    // TODO: Account for changes in use directives.
}
