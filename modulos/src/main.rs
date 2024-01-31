mod operacoes;
use operacoes::matematica::*;
use rand::random;

fn main() {
    println!("2 + 2 : {}", somar(2, 2));
    println!("2 - 2 : {}", subtrair(2, 2));

    println!("Número aletório: {}", random::<i8>());
}
