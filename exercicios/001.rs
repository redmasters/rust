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
    let mut number_b = String::new();

    io::stdin()
        .read_line(&mut number_a)
        .expect("Falha ao ler o numero")


    let number_a: u32 = number_a.trim().parse().expect("Por favor insira um numero")
    let number_b: u32 = number_b.trim().parse().expect("Por favor insira um numero")

}
