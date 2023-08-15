// Obsługa błędów

fn blad() -> Result<i32, &'static str> {
    Err("Blad")
}

fn main() -> Result<(), &'static str> {
    let wynik = blad()?;

    Ok(())
}
