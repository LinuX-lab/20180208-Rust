// Automatyczny import standardowych oobiektów do łączności z JS
use wasm_bindgen::prelude::*;

/// Wołane podczas inicjalizowania modułu WASM. Znacznik `start` w nawiasie to informacja, że należy
/// tę funkcję wywołać natychmiast po powołaniu modułu do życia.
#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    // To zwraca obraz elementu `window` normalnie wykorzystywanego w JS do dostępu do DOM
    let window = web_sys::window().expect("no global `window` exists");

    // Teraz już z górki: dobieramy się do window.document
    let document = window.document().expect("should have a document on window");

    // Dobieramy się do window.document.body
    let body = document.body().expect("document should have a body");

    // Tworzymy nowy, "samobieżny" element typu <P>
    let val = document.create_element("p")?;

    // Ważne: `val` w tym miejscu to uchwyt do elementu żyjącego w DOM przeglądarki, nie sam
    // element. Gdyby było inaczej, zakończenie funkcji automatycznie zlikwidowałoby ten element
    // i nic byśmy nie zobaczyli - takie trochę *use-after-free*. A tak likwodowany jest tylko
    // interfejs sterujący, sam element istnieje dalej w przeglądarce.

    // Wstawiamy do niego tekst ...
    val.set_inner_html("Hello from Rust!");

    // ... i dopinamy na końcu elementu body
    body.append_child(&val)?;

    // Informujemy przeglądarkę, że moduł zainicjalizował się poprawnie.
    Ok(())
}

/// Funcja użytkownika eksportowana do JavaScriptu
#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
