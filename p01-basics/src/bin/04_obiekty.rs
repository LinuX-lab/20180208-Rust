use std::fmt::{Error, Formatter};

// Prosta struktura
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

    fn drukuj(&self) -> String {
        format!("{} {} ({})", self.imie, self.nazwisko, self.wiek)
    }
}

// std::fmt::Display to interfejs w terminologii Javy/Go
// Jest wywoływany jako uniwersalny odpowiednik "ToString()"
impl std::fmt::Display for Osoba {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&self.drukuj())
    }
}

// std::fmt::Debug to interfejs w terminologii Javy/Go
// Jest wywoływany do inspekcji zawartości (formatka {:?})
impl std::fmt::Debug for Osoba {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_fmt(format_args!(
            "Osoba(imie:'{}'; nazwisko:'{}'; wiek:{})",
            self.imie, self.nazwisko, self.wiek
        ))
    }
}

fn main() {
    let s = Osoba::new("Jan", "Kowalski", 20);

    // Użycie metody do konwersji na String
    println!("{}", s.drukuj());

    // Użycie automatycznego formatowania
    println!("{}", s);

    // Użycie widoku debugowania
    println!("{:?}", s);
}
