## Przykładowa aplikacja `WASM`

[Web Assembly](https://webassembly.org/), znany też jako `WebAsm` albo `WASM` to nowa forma bezpiecznych, uniwersalnych aplikacji działających w przeglądarce. Technologicznie zajmuje niszę w połowie pomiędzy *JavaScriptem* a *Flashem*/*Javą*. Z jednej strony jest to stosowa maszyna wirtualna wykonująca bytecode (jak *Java*), mająca pełne mechanizmy separacji pamięci i wejścia-wyjścia, z drugiej strony jest to technologia osadzona w przeglądarkach bez potrzeby instalowania dodatkowych pluginów i mająca pełny dostęp do usług przeglądarki (jak *JavaScript*).

Jeżeli chodzi o kompilację i języki źródłowe, to WASM ma swój backend w `LLVM`, więc wcześniej czy później większość języków obsługiwanych przez ten stos kompilatorów będzie mogła służyć do pisania aplikacji WASM. Na dziś jest to `C++` i `Rust`. Dodatkowo kompilator `Go` potrafi generować aplikacje WASM. Oczywiście można też pisać aplikacje bezpośrednio w kodzie natywnym (tekstowy zapis WASM) - w nieślubnym dziecku Lispa i Asemblera. 

Kod w tym katalogu składa się z dwóch części:
* Prostej aplikacji WASM napisanej w języku Rust i strony WWW ładującej i wykonującej kod wynikowy. 
* Prostego serwera WWW opartego o framework [Iron](https://github.com/iron/iron) hostującego naszą aplikację (a w każdym razie hostującego *bieżący katalog*).

### Część WASM

Do generacji kodu WASM służy aplikacja [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen), opakowująca program w Rust w *middleware* zapewniający dwukierunkową łączność JS-Rust:
* Udostępnienie całej potrzebnej funkcjonalności JS dla aplikacji WASM: można wołać usługi przeglądarki bezpośrednio z kodu w Rust. 
* Eksportowanie funkcje użytkownika z Rust do JS: JavaScript może bezpośrednio wołać funkcje użytkownika napisane w Rust. 

### Serwer WWW

Standardowy serwer WWW w Iron-ie jest dużo krótszy. Rozbudowana forma służy jedynie "ładnej" obsłudze komunikatów błędów.