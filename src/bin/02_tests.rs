// Standardowe użycie testu: robimy moduł włączany tylko przy uruchomieniu testów
// Konstrukcja `#[cfg(...)]` to odpowiednik C/C++ `#ifdef`
// Konstrukcja `#[...]` bez `cfg` to bardziej `#pragma`

#[cfg(test)]
mod tests {
    #[test]
    fn matematyka_dziala() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn nie_dziel_przez_0() {
        let zero = 0;
        let wynik = 2 / zero;
        println!("Wynik dzielenie przez 0={}", wynik);
    }
}


fn main() {
    println!("Hello!");
}
