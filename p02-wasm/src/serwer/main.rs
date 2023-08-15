use iron::prelude::*;
use iron::{AfterMiddleware, Iron};
use mount::Mount;
use staticfile::Static;
use std::path::Path;

const URL: &str = "0.0.0.0:3000";
const FORMAT_CZAS: &str = "[year][month][day] [hour][minute][second]";

struct Logger {}

impl Logger {
    fn new() -> Logger {
        Logger {}
    }

    fn time_stamp(&self) -> String {
        let format = time::format_description::parse(FORMAT_CZAS).unwrap();
        let now = time::OffsetDateTime::now_utc();
        now.format(&format).unwrap().to_string()
    }
}

// Implementujemy metody wymagane przez interfejs AfterMiddleware
impl AfterMiddleware for Logger {
    // Obsługa potoku zakończonego sukcesem
    fn after(&self, req: &mut Request, res: Response) -> Result<Response, IronError> {
        // Wypisujemy linijkę logu na konsolę

        println!(
            "{} OK {} {} {}",
            self.time_stamp(),
            req.remote_addr.to_string(),
            req.method,
            req.url,
        );

        // Zwracamy zawartość bez zmiany
        Ok(res)
    }

    // Obsługa potoku zakończonego błędem. Hint: możemy "zjeść" ten błąd i zamienić go na
    // "sukces" (tj jednak wygenerować jednak jakąś stronę).
    // Tutaj generujemy stronę z pełniejszym opisem błędu
    fn catch(&self, req: &mut Request, err: IronError) -> Result<Response, IronError> {
        use iron::mime;

        // Wypisujemy linijkę logu na konsolę
        println!(
            "{} ERR {} {} {} {:?}",
            self.time_stamp(),
            req.remote_addr.to_string(),
            req.method,
            req.url,
            err.response.status.unwrap()
        );

        // Typ zwracanego dokumentu (bez tego to surowy tekst)
        let content_type = "text/html; charset=UTF-8".parse::<mime::Mime>().unwrap();

        // Konstruujemy ciało. r"..." to "surowy" string, można go łamać między liniami
        let body = format!(
            r"<html>
<head><title>Error: {error}</title></head>
<body>
<h1>Error</h1>
<strong>{error}</strong> in <tt>{method} {url}</tt>
<h2>Details - request</h2>
<pre>{request:?}
</pre>
<h2>Details - response</h2>
<pre>{response:?}
</pre>
</body>",
            error = err.response.status.unwrap(),
            method = req.method,
            url = req.url,
            request = req,
            response = err.response,
        );

        // Zamiast przekazywać `Err(err)` zamieniamy go na Ok(strona), kopiując jednak oryginalny
        // status błędu

        let original_status = err.response.status.unwrap();

        Ok(Response::with((content_type, original_status, body)))
    }
}

fn main() {
    // Mapowanie URI na handlery
    let mut mount = Mount::new();

    // Root jest serwowany z bieżącego katalogu
    mount.mount("/", Static::new(Path::new(".")));

    // Tworzenie potoku przetwarzania
    let mut chain = Chain::new(mount);

    // Dopinanie loggera na samym końcu potoku
    chain.link_after(Logger::new());

    println!("Server running on {}", URL);

    // Uruchomienie serwera
    Iron::new(chain).http(URL).unwrap();
}
