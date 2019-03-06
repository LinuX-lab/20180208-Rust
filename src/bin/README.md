## Uruchomienie przykładów

Kolejne przykłady uruchamia się przez `cargo run --bin NAZWA` (bez rozszerzenia), np:

```bash
> cargo run --bin 01_hello
```

Aby uruchomić to w wersji *release* (z pełnymi optymalizacjami), dodać trzeba flagę `--release`:

```bash
> cargo run --release --bin 01_hello
```

Jeżeli pakiet generuje tylko jedną aplikację binarną (w katalogu `/src/bin` jest tylko jeden plik źródłowy, albo w ogóle
nie ma katalogu `/src/bin` a aplikacja jest w pliku `/src/main.rs`), można
opuścić selektor `--bin NAZWA`.

## Uruchomienie testów

Testy uruchamia się globalnie przez `cargo test`, ewentualnie
per-program przez `cargo test --bin NAZWA` (oczywiście jeżeli
program w ogóle zawiera jakieś testy).
