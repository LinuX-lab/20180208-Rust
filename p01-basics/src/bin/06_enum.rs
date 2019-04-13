use std::fmt::{Error, Formatter};

// Rust ma typy wyliczeniowe (algebraic data type, fat enum)
// To tani typ prosty, więc pozwalamy wygenerować automatyczne funkcje do kopiowania i klonowania
#[derive(Copy, Clone)]
pub enum Liczba {
    INT(i128),
    RAT(i128, i128),
    FLT(f64),
}

impl Liczba {
    fn newi(i: i128) -> Liczba {
        Liczba::INT(i)
    }

    fn newf(f: f64) -> Liczba {
        Liczba::FLT(f)
    }

    fn newr(n: i128, d: i128) -> Liczba {
        Liczba::RAT(n, d)
    }
}

impl std::fmt::Display for Liczba {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Liczba::INT(i) => f.write_fmt(format_args!("{}", i)),
            Liczba::RAT(n, d) => f.write_fmt(format_args!("{}/{}", n, d)),
            Liczba::FLT(fl) => f.write_fmt(format_args!("{}", fl)),
        }
    }
}

fn main() {
    // Mut, bo zmieniamy to później. Bez `mut` zachowuje się jak Javove `final`.
    let mut x = Liczba::newr(1, 3);
    println!("Liczba: {}", x);

    x = Liczba::newi(5);
    println!("Liczba: {}", x);

    x = Liczba::newf(355_f64 / 113_f64);
    println!("Liczba: {}", x);
}
