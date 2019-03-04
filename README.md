## Instalacja Rust

Rust jest dostępny w pakietach w niektórych dystrybucjach, jednak często
jest to wersja (mocno) nieaktualna - a jak już, to wyłącznie z gałęzi
*stable*. Na przykład *rust* spaczkowany w *Ubuntu Cosmic* jest w wersji
1.30, gdy obecne *stable* to 1.33.  

Polecaną metodą instalacji jest aplikacja
[rustup](https://rustup.rs/)
instalowana w następujący sposób (z **konta użytkownika**):

```bash
curl https://sh.rustup.rs -sSf | sh
```

To zainstaluje i skonfiguruje kompilator `rustc` w wersji *stable* dla
bieżącego systemu i procesora, manager pakietów `cargo` oraz bibliotekę
standardową oraz zmodyfikuje zmienną `PATH` aby konto użytkownika
zobaczyło te narzędzia.

Tu są ciekawe polecenia komendy `rustup` (sama komenda ma rozbudowany
system pomocy aktywowany przez `--help`):

### `rustup toolchain list`
Wyświetla zainstalowane wersje kompilatora i bibliotek. W użyciu są trzy
podstawowe kanały: `stable`, `beta` i `nightly`.

### `rustup toolchain install <CHANNEL>` i `rustup toolchain uninstall <CHANNEL>` 
Umożliwia instalację i deinstalację różnych wersji kompilatora. Można
podać kanał, konkretną wersję wydania oraz konkretną datę wydania. Więcej w
`rustup toolchain --help`. Przykład:
```bash
> rustup toolchain install nightly
```

### `rustup default <CHANNEL>`
Wybiera dany kanał kompilatora jako domyślny. Przykład:
```bash
> rustup default nightly
```

### `rustup target list`
Wyświetla listę obsługiwanych i zainstalowanych (wytłuszczone) platform do
krosskompilacji.

### `rustup target add <PLATFORM1> <PLATFORM2>` i `rustup target remove <PLATFORM1> <PLATFORM2>`
Instalacja i deinstalacja platform w bieżącym kanale. Instalacja w kanale
innym, niż domyślny jest możliwa przez dodanie `--toolchain <CHANNEL>`  po
`add` bądź `remove`:

Jeżeli chcemy robić coś na mikrokontrolery *ARM Cortex-M4* albo pod *WebAssembly* to
wydajemy polecenia:
```bash
# ARM Cortex-M4
> rustup target add thumbv7em-none-eabihf

# Web-Assembly
> rustup target add wasm32-unknown-unknown
```

### `rustup component list`
Tu jest lekki bajzel. O ile w założeniu ta rodzina poleceń ma instalować *narzędzia
dodatkowe*, to z jakiegoś względu wyświetla także targety. Wyświetla, ale nie można
ich stąd instalować ani usunąć.

### `rustup component add <TOOL> <TOOL>` i `rustup component remove <TOOL> <TOOL>`
Dodaje i usuwa narzędzia i składniki dodatkowe. Tak jak przy targetach, pracuje na
domyślnym kanale i można wymusić inny przez `--toolchain <CHANNEL>` po `add` bądź
`remove`.

Dostępne komponenty:
* `rust-src` - must-have, kod źródłowy biblioteki standardowej. Wymagany przez
niektóre narzędzia i pakiety.
* `rustfmt` - drugi must-have, narzędzie do autoformatowania kodu źródłowego.
* `rls` - *rust language server*, narzędzie do introspekcji kodu używane przez
niektóre IDE, jak *VSCode*, *vim* czy *emacs*.
* `clippy` - liner kodu, też wykorzystywany przez IDE

