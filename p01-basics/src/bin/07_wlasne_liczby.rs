use std::fmt::{Display, Error, Formatter};
use std::ops::{Add, Div, Mul};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Ulamek {
    licznik: u64,
    mianownik: u64,
}

#[inline]
fn gcd(x: u64, y: u64) -> u64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

impl Ulamek {
    fn new(l: u64, m: u64) -> Self {
        Ulamek {
            licznik: l,
            mianownik: m,
        }
        .reduce()
    }

    fn reduce(&self) -> Ulamek {
        let g = gcd(self.licznik, self.mianownik);
        Ulamek {
            licznik: self.licznik / g,
            mianownik: self.mianownik / g,
        }
    }
}

impl Add for Ulamek {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Trochę magii: linter zauważa, że używamy * (mnożenia) w implementacji
        // traita Add (dodawania) i zgłasza to jako warning
        //
        // Poniższa linijka wyłącza to ostrzeżenie
        #![allow(clippy::suspicious_arithmetic_impl)]
        Ulamek::new(
            self.licznik * rhs.mianownik + rhs.licznik * self.mianownik,
            rhs.mianownik * self.mianownik,
        )
    }
}

impl Mul for Ulamek {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Ulamek::new(self.licznik * rhs.licznik, rhs.mianownik * self.mianownik)
    }
}

impl Div for Ulamek {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Ulamek::new(self.licznik * rhs.mianownik, rhs.licznik * self.mianownik)
    }
}

impl Display for Ulamek {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_fmt(format_args!("{}/{}", self.licznik, self.mianownik))
    }
}

fn main() {
    let u1 = Ulamek::new(2, 6);
    println!("U1={}", u1);

    let u2 = Ulamek::new(6, 16);
    println!("U2={}", u2);

    println!("U1+U2={}", u1 + u2);
    println!("U1*U2={}", u1 * u2);
    println!("U1/U2={}", u1 / u2);
}
