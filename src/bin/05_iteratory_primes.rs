/// Struktura z jednym anonimowym składnikiem - wektorem. Odwołuje się do niego przez `.0`
pub struct LiczbyPierwsze(Vec<u128>);

/// Iterator po liczbach pierwszych, tworzony `LiczbyPierwsze::iter`
pub struct LiczbyPierwszeIter<'a> {
    znalezione: &'a mut LiczbyPierwsze,
    iteracja: usize,
}

/// Dodajemy metody do struktury `LiczbyPierwsze`
impl LiczbyPierwsze {
    /// Konstruktor, inicjalizuje listę pierwszymi dwiema liczbami pierwszymi
    pub fn new() -> Self {
        Self(vec![2, 3])
    }

    /// Dodaje następną liczbę pierwszą
    pub fn dodaj(&mut self) {
        // `mut` bo je zmieniamy w pętli
        // Kandydat jest o 2 większy od największej dotąd znalezionej liczby pierwszej

        // `last()` zwraca `Option<>` (bo tablica może być pusta i wtedy jest `None`).
        // My WIEMY, że tablica jest niepusta, więc `unwrap()`-em zamieniamy to na gołą wartość.
        // Jakby jednak byłą pusta, to to spanikuje.
        let mut kandydat: u128 = self.0.last().unwrap() + 2;
        let mut reszta = 0;

        // Nieskończona pętla
        loop {
            // Po kolei dla dotąd znalezionych liczb pierwszych
            for &n in &self.0 {
                reszta = kandydat % n;

                // Jak dzieli albo jest większa od sqr(ostatnia)
                if reszta == 0 || n * n > kandydat {
                    // ... wyjdź z FOR
                    break;
                }
            } // for

            // Albo reszta==0 (to nie jest pierwsza)
            // albo reszta!=0 (sprawdziliśmy wszystki potrzebne czynniki)

            if reszta != 0 {
                // Jeżeli to jednak jest pierwsza, dodaj ją do naszej listy
                self.0.push(kandydat);
                // ... i wyjdź z LOOP
                break;
            };

            // Jeżeli to nie była pierwsza, weź następną nieparzystą
            kandydat += 2;

            // ... i sprawdź od nowa
        } // loop
    }

    /// Liczba znalezionych dotąd liczb
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Iterator po wszystkich liczbach pierwszych
    pub fn iter(&mut self) -> LiczbyPierwszeIter {
        LiczbyPierwszeIter {
            znalezione: self,
            iteracja: 0,
        }
    }
}

// Implementacja właściwego iteratora
impl<'a> Iterator for LiczbyPierwszeIter<'a> {
    type Item = u128;
    fn next(&mut self) -> Option<u128> {
        // Dorób brakujące liczby, aż będzie ich wystarczająco dużo
        while self.iteracja >= self.znalezione.len() {
            self.znalezione.dodaj()
        }

        // To kolejny obieg pętli
        self.iteracja += 1;

        // Pobierz liczbę z tej iteracji
        let m = self.znalezione.0[self.iteracja - 1];

        // I ją zwróć
        Some(m)
    }
}


fn main() {
    let mut pierwsze = LiczbyPierwsze::new();

    for (ix, n) in pierwsze.iter().enumerate().take(1000) {
        println!("Liczba pierwsza #{}: {}", ix + 1, n);
    }
}
