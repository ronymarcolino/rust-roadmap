 use std::mem;

// Define struct of Pessoa
#[derive(Debug)]
struct Pessoa {
    nome: String,
    idade: u8,
}

// Define tuple called Pair
struct Pair(i32, f32);

// Define struct of Point
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
}

fn main() {
    // Instance of pessoa
    let pessoa = Pessoa {
        nome: String::from("Rony"),
        idade: 34
    };
    // Instance of tuple
    let tupla = Pair(10,0.5);

    // Instance of Ponto
    let ponto = Point{ x: 0.0, y: 0.0, z: 0.0 };
    println!("tamanho em bytes: {}", mem::size_of_val(&ponto));
    println!("{:?}", pessoa);
    println!("Tupla {}, {}", tupla.0, tupla.1);
    println!("{:?}", ponto);
}
