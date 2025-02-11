use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Obtenir le chemin du projet
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get manifest directory");
    let java_dir = PathBuf::from(&manifest_dir)
        .parent()
        .expect("Failed to get parent directory")
        .join("java");

    println!("cargo:rerun-if-changed=../java/pom.xml");
    println!("cargo:rerun-if-changed=../java/src");

    // Vérifier que Maven est installé
    if !Command::new("mvn").arg("--version").output().is_ok() {
        panic!("Maven not found! Please install Maven and make sure it's in your PATH");
    }

    // Construire le projet Java avec Maven
    let status = Command::new("mvn")
        .current_dir(&java_dir)
        .arg("clean")
        .arg("package")
        .arg("-Pnative")
        .status()
        .expect("Failed to execute Maven command");

    if !status.success() {
        panic!("Failed to build Java project");
    }

    // Vérifier que le fichier .so a été généré
    let lib_path = java_dir.join("target").join("embedded-powsybl.so");
    if !lib_path.exists() {
        panic!("Library file not found at {:?}", lib_path);
    }

    // Indiquer à Cargo où trouver la bibliothèque
    println!(
        "cargo:rustc-link-search=native={}",
        java_dir.join("target").display()
    );
}
