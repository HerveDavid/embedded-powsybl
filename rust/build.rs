use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get manifest directory");
    let target_dir = PathBuf::from(&manifest_dir)
        .parent()
        .unwrap()
        .join("java")
        .join("target");

    // Configuration du linker pour pointer vers le .so
    println!("cargo:rustc-link-search=native={}", target_dir.display());
    println!("cargo:rustc-link-lib=embedded-powsybl");

    // Génération des bindings
    let bindings = bindgen::Builder::default()
        .header(target_dir.join("embedded-powsybl.h").to_str().unwrap())
        .clang_arg(format!("-I{}", target_dir.display())) // Ajouter cette ligne
        .allowlist_type("graal_.*")
        .allowlist_function("graal_.*")
        .derive_debug(true)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(PathBuf::from(&manifest_dir).join("src/bindings.rs"))
        .expect("Couldn't write bindings!");
}
