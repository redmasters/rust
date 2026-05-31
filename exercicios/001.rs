use std::cmp::Ordering;
use std::io;

fn main(){

    println!("----Imprima seu nome----");
    println!("Qual o seu nome?");

    let mut name = String::new();

    io::stdin()
    .read_line(&mut name)
    .expect("Tente novamente");

    let name = name.trim();
    println!("{name} eh um otimo nome!");

    println!("---Compare numeros!---");
    println!("Insira o primeiro numero:");

    let mut number_a = String::new();

    io::stdin()
        .read_line(&mut number_a)
        .expect("Falha ao ler o numero");
    let number_a: u32 = number_a.trim()
        .parse()
        .expect("Por favor insira o primeiro numero");

    println!("Voce inseriu o numero {number_a}");

    println!("Insira o segundo numero:");
    let mut number_b = String::new();

    io::stdin()
        .read_line(&mut number_b)
        .expect("Falha ao ler o numero");


    let number_b: u32 = number_b.trim()
        .parse()
        .expect("Por favor insira o segundo numero");

    match number_a.cmp(&number_b) {
        Ordering::Less => println!("{number_a} eh menor que {number_b}"),
        Ordering::Greater=> println!("{number_a} eh maior que {number_b}"),
        Ordering::Equal=> println!("{number_a} eh igual a {number_b}")
    }

}
