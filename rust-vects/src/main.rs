/// Vectors are resizable arrays used to store lists of items on the heap.
/// They can only contain values of the same type and are created using the vec! macro.
/// Indexed from zero, they are mutable, allowing iteration, passing as arguments, and returning from functions.
/// They can be updated using methods like push, pop, insert, and remove, and sorted using the sort method.
/// Vectors are defined using the Vec<T> type
/// ---------------------------------------------------------
/// /// Los vectores son matrices redimensionables utilizadas para almacenar listas de elementos en el montón.
/// Solo pueden contener valores del mismo tipo y se crean utilizando la macro vec!.
/// Indexados desde cero, son mutables, lo que permite la iteración, el paso como argumentos y el retorno desde funciones.
/// Se pueden actualizar utilizando métodos como push, pop, insert y remove, y ordenados utilizando el método sort.
/// Los vectores están definidos utilizando el tipo Vec<T>.
///

fn main() {
    // Declare vector, value = "0", length = 5
    let zeroes = vec![0; 5];
    println!("Zeroes: {:?}", zeroes);

    let three_nums = vec![16, 2, 49];
    println!("Initial vector: {:?}", three_nums);
    println!(
        "The sum of the three numbers is: {}",
        sum(three_nums.clone())
    );
    println!(
        "The average of the three numbers is: {}",
        average(three_nums)
    );

    // let mut fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
    let mut fruit = Vec::new();
    fruit.push("banana");
    fruit.push("apple");
    fruit.push("coconut");
    fruit.push("orange");
    fruit.push("strawberry");
    println!("Fruits: {:?}", fruit);
    // Pop off value at end of vector
    //remove & return the last element
    // Call pop() method from inside println! macro
    println!("Pop off: {:?}", fruit.pop());
    // fruit.pop();
    println!("Fruits: {:?}", fruit);
    // Insert value at index 1
    // Call insert() method from inside println! macro
    // println!("Insert: {:?}", fruit.insert(1, "kiwi"));
    fruit.insert(1, "kiwi");
    println!("Fruits: {:?}", fruit);
    // Remove value at index 2
    // Call remove() method from inside println! macro
    // println!("Remove: {:?}", fruit.remove(2));
    fruit.remove(2);
    println!("Fruits: {:?}", fruit);
    // Sort and reverse vector
    // Call sort() and reverse() methods from inside println! macro
    // println!("Sort: {:?}", fruit.sort());
    fruit.sort();
    println!("Fruits: {:?}", fruit);
    // Reverse vector
    // Call reverse() method from inside println! macro
    // println!("Reverse: {:?}", fruit.reverse());

    fruit.reverse();
    println!("Fruits: {:?}", fruit);
    // Access elements by index
    // Call index 0 and last index from inside println! macro
    println!("The first fruit is: {}", fruit[0]);
    println!("The last fruit is: {}", fruit[fruit.len() - 1]);
}
/// Suma los elementos de un vector de números enteros.
fn sum(numbers: Vec<i32>) -> i32 {
    let mut total = 0;
    for num in numbers {
        total += num;
    }
    total
}
/// Calcula el promedio de los elementos de un vector de números enteros.
fn average(numbers: Vec<i32>) -> f64 {
    let sum: i32 = sum(numbers.clone());
    let count: i32 = numbers.len() as i32;
    sum as f64 / count as f64
}
