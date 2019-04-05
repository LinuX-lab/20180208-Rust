// Struktury kontrolne

#[allow(dead_code)]
fn dubluj1(i: u32) -> u32 {
    // Model według C/C++, słowo kluczowe return
    return i * 2;
}

#[allow(dead_code)]
fn dubluj2(i: u32) -> u32 {
    // Model zalecany w Rust, typowy dla języków funkcyjnych
    // Funkcja zwraca ostatnie wyrażenie w niej wyliczone
    i * 2 // <-- Bez średnika
}

// `'static` to tzw. `lifetime`, sprawa na następne spotkanie
// Tu zwracamy stałe stringi istniejące przez cały czas życia programu

fn ile1(i: u32) -> &'static str {

    // To odpowiednik switch/case, tak jak to się robi w C/C++/Go

    match i {
        0 => return "nic",
        1 => return "jeden",
        2 => return "dwa",
        _ => return "mnóstwo",
    };
}

fn ile2(i: u32) -> &'static str {

    // `match`, tak jak `if` i inne słowa kluczowe jest *wyrażeniem*, tj zwraca wartość.
    // Tutaj wartością jest ostatnie wyrażenie każdej gałęzi.

    // Rust (prawie) nie ma const. Jeżeli nic nie napiszemy, to "zmienna" jest typu const,
    // bardziej coś jak `final` w Javie. Musimy jawnie napisać, że coś nie jest const (słowo
    // kluczowe `mut`).
    //
    // Jest tylko jeden scenariusz z jawnym `const` - surowe wskaźniki (Bad Thing™). Te **muszą**
    // być jawnie zapisane albo jako `mut` albo jako `const`.

    let wynik = match i {
        0 => "nic",
        1 => "jeden",
        2 => "dwa",
        _ => "mnóstwo",
    };

    return wynik; // Zwracamy to
}

fn ile3(i: u32) -> &'static str {

    // Forma klasyczna dla Rust-a

    match i {
        0 => "nic",
        1 => "jeden",
        2 => "dwa",
        _ => "mnóstwo",
    } // <-- Bez średnika, funkcja zwraca wartość z odpowiedniej gałęzi
}

fn main() {
    // Jak pisałem, Rust sam wymyśla właściwe typy. W przypadku wątpliwości zwraca błąd i wtedy
    // trzeba podać typ jawnie.

    let argumenty = std::env::args(); // Zwraca iterator
    let ile_argumentow = argumenty.len(); // Bierzemy jego długość, typ `usize`

    // Nie ma automatycznych konwersji typów, ma być bezpiecznie
    println!("Program ma {} parametr/y/ów", ile1(ile_argumentow as u32));

    // Wędrujemy po iteratorze
    for argument in argumenty {
        println!(" * {}", argument);
    }

    println!("----");

    // Jak interesują nas numery kolejne argumentów, na iterator możemy nałożyć
    // filtr `enumerate()`, który zamienia elementy na parę (numer_kolejny,element).
    // Możemy tą parę od razu rozpakować do dwóch zmiennych.
    //
    // Poprzedni `for` "skonsumował" iterator `argumenty`, nie można go użyć ponownie.

    for (numer, argument) in std::env::args().enumerate() {
        println!("{}. {}", numer, argument);
    }
}

#[cfg(test)]
mod tests {
    // Wciągamy wszystkie symbole nadrzędne tutaj
    use super::*;

    #[test]
    fn dublowanie() {
        assert_eq!(dubluj1(2), 4);
        assert_eq!(dubluj2(2), 4);
    }

    #[test]
    fn tekst() {
        assert_eq!(ile1(0), "nic");
        assert_ne!(ile2(1), "mnóstwo");
        assert_eq!(ile3(2), "dwa");
    }
}
