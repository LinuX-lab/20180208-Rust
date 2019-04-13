// Prosta struktura, ale tym razem każemy wygenerować implementację fmt::Debug automatycznie
#[derive(Debug)]
pub struct Osoba {
    imie: String,
    nazwisko: String,
    wiek: u8,
}

impl Osoba {
    // P.O. konstruktora w Ruście
    fn new(i: &str, n: &str, w: u8) -> Self {
        Osoba {
            imie: String::from(i),
            nazwisko: String::from(n),
            wiek: w,
        }
    }
}

fn main() {
    let s = Osoba::new("Jan", "Kowalski", 20);

    // Użycie widoku debugowania wygenerowanego automatycznie
    println!("{:?}", s);
}
