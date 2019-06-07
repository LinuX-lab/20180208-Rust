// Obsługa błędów

#[allow(dead_code)]
fn blad()->Result<i32,&'static str>{
    Err("Blad")
}

#[allow(dead_code)]
fn nieblad()->Result<i32,&'static str>{
    Ok(42)
}

fn main()->Result<(),&'static str>{
    let wynik=blad()?;
    //let wynik=nieblad()?;
    println!("Wynik={}",wynik);
    Ok(())
}
