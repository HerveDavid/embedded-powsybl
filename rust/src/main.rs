use embedded_powsybl::EmbeddedPowsybl;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialisation simple
    let powsybl = EmbeddedPowsybl::init()?;
    
    // Utilisation
    let result = powsybl.factorial(5)?;
    println!("5! = {}", result);
    
    // Pas besoin de cleanup manuel - il sera fait automatiquement
    Ok(())
}