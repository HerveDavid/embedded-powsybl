use embedded_powsybl::EmbeddedPowsybl;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Récupérer le premier argument (le chemin du fichier)
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <chemin_du_fichier>", args[0]);
        std::process::exit(1);
    }

    let powsybl = EmbeddedPowsybl::init()?;
    let filepath = std::fs::canonicalize(&args[1])?;
    let content = powsybl.read_xiidm_file(&filepath.to_str().unwrap());

    println!("{}", content.unwrap());

    Ok(())
}
