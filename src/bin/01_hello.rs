// Komentarze są jak w C++

/*
 Można też jak w C, ale rustfmt je może skonwertować na powyższe.
*/

// Funkcje zaczynają się od `fn`, main nie ma argumentów (wyciągamy je biblioteką standardową)
// i nic nie zwraca. Można wymusić C-owate zachowanie `std::process::exit(liczba)`.
fn main() {
    // Wszystko z `!` na końcu to makro.
    println!("Hello, world!");

    // `{}` to zwykła "rozwijka".
    println!("Hello, {}!", "World");

    // `{cyfra}` to rozwijka z argumentami pozycyjnymi
    println!("Hello, my name is {1}, {0} {1}!", "James", "Bond");

    // `{nazwa}` to rozwijka z argumentami nazwanymi
    println!("Hello, I am {lang} v. {ver}!", lang = "Rust", ver = "1.33");

    // Rust posługuje się przyrostkami do wymuszenia typów, i8..i128, u8..u128 do typów całkowitych
    // oraz f32 i f64 dla floatów.
    //
    // Dodatkowo w liczbach można dowolnie wciskać kładki dla zwiększenia czytelności, byle kładka
    // nie była pierwszym albo ostatnim znakiem.
    //
    // Opcje formatowania zbliżone do `printf`-owych daje się po `:`.
    //
    // Można opuszczać typy, jeżeli kompilator jest w stanie wymyślić właściwy.
    // Poniższe w pełnej formie to `let pi: f64 = ...`

    let pi = 355_f64 / 113_f64;
    println!("pi={0} albo ~{0:.2}", pi);

    // Rust posługuje się przyrostkami do wymuszania bazy, 0b (binarny), 0o (ósemkowy) i
    // 0x(szesnastkowy)
    println!("1000(2)={}, 1000(8)={}, 1000(16)={}", 0b_10_00, 0o_10_00, 0x_10_00);

    // Przedrostek 0 nie włącza trybu ósemkowego, jak w C/C++
    println!("010={}", 010);
}
