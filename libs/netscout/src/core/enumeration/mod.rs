use std::error:Error;

pub async fn enumerate(target: &str) -> Result<(), Box<dyn Error>> {
    println!("Enumerating target: {}", target);

    Ok(())
}
