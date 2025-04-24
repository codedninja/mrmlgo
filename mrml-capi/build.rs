use std::env;
use std::path::Path;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let package_name = env::var("CARGO_PKG_NAME").unwrap();
    let output_file = Path::new(&crate_dir)
        .join("include")
        .join(format!("{}.h", package_name));

    match cbindgen::generate(crate_dir) {
        Ok(header) => {
            header.write_to_file(output_file);
        }
        Err(err) => {
            panic!("{}", err)
        }
    }
}
