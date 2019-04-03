fn main() {
    // Iterator zwracający nieskończony ciąg (no, dobra, do 2^128-1, nie mamy tyle czasu)
    let ciag = 1_u128..;

    // Rust, jak nie musi to nie robi...
    // Take() konsumuje ciąg, więc wcześniej klonujemy go. To ultralekki obiekt, więc
    // jest tanie
    println!("Pierwsze 10={:?}, podejście #1", ciag.clone().take(10));

    // Tu go zmuszamy do rzeczywistego wyliczenia wartości i wpakowania ich do wektora
    println!("Pierwsze 10={:?}, podejście #2",
             ciag
                 .clone() // Klonujemy
                 .take(10) // Bierzemy pierwsze 10
                 .collect::<Vec<_>>() // i przerabiamy na wektor tego, co jest w iteratorze
    );

    // Tu to dodatkowo transformujemy
    println!("Kwadraty {:?}",
             ciag
                 .clone() // Klonujemy
                 .map(|x| x * x) // Transformujemy na kwadrat
                 .take(10) // Bierzemy pierwsze 10
                 .collect::<Vec<_>>() // i przerabiamy na wektor tego, co jest w iteratorze
    );

    // Tu możemy transformować na pary
    println!("Liczby i ich kwadraty {:?}",
             ciag
                 .clone() // Klonujemy
                 .map(|x| (x, x * x)) // Transformujemy na parę (liczba,kwadrat)
                 .take(10) // Bierzemy pierwsze 10
                 .collect::<Vec<_>>() // i przerabiamy na wektor tego, co jest w iteratorze
    );

    // Tu można odrazu ładnie sformatorać (`format!()` to bezpieczna wersja `sprintf`-a)
    println!("Liczby i ich kwadraty {:?}",
             ciag
                 .clone() // Klonujemy
                 .map(|x| format!("{0}*{0}={1}", x, x * x)) // Transformujemy String
                 .take(10) // Bierzemy pierwsze 10
                 .collect::<Vec<_>>() // i przerabiamy na wektor tego, co jest w iteratorze
    );

    // Tu robimy iloczyny kolejnych liczb - 1*2, 2*3, 3*4, itd
    println!("Kolejne iloczyny {:?}",
             ciag
                 .clone() // Klonujemy
                 .zip( // Przeplatamy z drugim ciągiem
                       ciag.clone().skip(1) // Nasz ciąg z pominiętym 1. wyrazem
                 ) // a.zip(b) zwraca pary (a1,b1),(a2,b2), itd
                 .map(|(x1, x2)| format!("{0}*{1}={2}", x1, x2, x1 * x2)) // Transformujemy parę na String
                 .take(10) // Bierzemy pierwsze 10
                 .collect::<Vec<_>>() // i przerabiamy na wektor tego, co jest w iteratorze
    );
}
