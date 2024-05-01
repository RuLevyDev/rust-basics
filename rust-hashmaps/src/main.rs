use std::collections::HashMap;

fn main() {
    //Declaramos la variable reviews como mutable para que podamos agregar, o quitar, claves y valores
    let mut reviews: HashMap<String, String> = HashMap::new();
    //Agregamos elementos al mapa hash mediante el método insert(<key>, <value>). En el código, la sintaxis es <hash_map_name>.insert():
    //el método insert() toma dos argumentos: la clave y el valor. En este caso, la clave es "Babymam" y el valor es "Very accurate."
    reviews.insert(String::from("Babymam"), String::from("Very accurate."));
    reviews.insert(String::from("Koru"), String::from("Sweet."));
    reviews.insert(String::from("Prodigy"), String::from("Great examples."));
    let app: &str = "Babymam";
    //buscar un valor específico en un mapa hash
    println!("\nReview for {}: {}", app, reviews.get(app).unwrap());

    let app2: &str = "Prodigy";
    //Para acceder a un valor en un mapa hash, usamos el método get(). El método get() toma un argumento: la clave que queremos buscar. En este caso, la clave es "Babymam".
    //El método get() devuelve un Option<&V>, donde V es el tipo de valor que se almacena en el mapa hash. En este caso, el valor es una cadena, por lo que el tipo de valor es String.
    //Para acceder al valor real, necesitamos desempaquetar el Option<&V> devuelto por el método get(). Podemos hacer esto usando el método unwrap() en el resultado de get().
    //El método unwrap() devuelve el valor real si el Option es Some(valor). Si el Option es None, el método unwrap() provocará un panic.
    match reviews.get(app2) {
        Some(review) => println!("\nReview for {}: {}", app2, review),
        None => println!("\n{} has no review.", app2),
    }
    let app1: &str = "Koru";
    println!("\nReview for \'{}\': {:?}", app1, reviews.get(app1));

    println!("Review for {}: {}", app1, reviews[app1]);

    match reviews.get(app1) {
        Some(review) => println!("Review for {}: {}", app1, review),
        None => println!("{} has no review.", app1),
    }

    // eliminar un valor de un mapa hash
    let obsolete: &str = "Babymam";
    println!("\n'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    //consultar un valor eliminado de un mapa hash
    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}
