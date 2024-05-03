use serde::Deserialize;

// Definir la estructura del JSON
#[derive(Debug, Deserialize)]
struct JsonData {
    urls: Vec<String>,
}

fn main() {
    // Lee el JSON desde un archivo
    let json_str =
        std::fs::read_to_string("videosURL.json").expect("Error al leer el archivo JSON");

    // Parsea el JSON
    let data: JsonData = serde_json::from_str(&json_str).expect("Error al parsear el JSON");

    let mut contador = 0;

    // Itera sobre las URLs y realiza solicitudes GET
    for url in data.urls {
        contador += 1;
        let _emoji = "✅";
        match reqwest::blocking::get(&url) {
            Ok(response) => {
                let status = response.status();
                let emoji = match status {
                    reqwest::StatusCode::OK => "✅",
                    _ => "❌",
                };
                println!("{} {} {}", emoji, contador, url);
            }
            Err(_) => println!("❌ Error al hacer la solicitud a {}", url),
        }
    }
}
