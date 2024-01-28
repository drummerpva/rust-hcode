fn main() {
    se();
    declaracao_if();
    loop_infinito();
    loop_finito();
    loop_finito_for();
    deu_match();
}

fn se() {
    let numero = -99;
    if numero > 0 {
        println!("Número {numero} é positivo");
    } else if numero == 0 {
        println!("Número {numero} é zero");
    } else {
        println!("Número {numero} é negativo");
    }
}

fn declaracao_if() {
    let condicao = true;
    // Rust não possio ternário
    let result = if condicao {
        "A condição é verdadeira"
    } else {
        "A condição é falsa"
    };
    println!("O valor de result é: {}", result);
}

fn loop_infinito() {
    let mut contador = 0;
    loop {
        println!("loop: Contador {contador}");
        contador += 1;
        if contador > 5 {
            break;
        }
    }
}

fn loop_finito() {
    let mut contador = 0;
    while contador > 5 {
        println!("while: Contador {contador}");
        contador += 1;
    }
}

fn loop_finito_for() {
    for numero in 0..=5 {
        println!("for: Contador {numero}");
    }
}

fn deu_match() {
    let estacao_ano = "Outono";
    match estacao_ano {
        "Verão" => println!("Calor"),
        "Primavera" => println!("Flores"),
        "Outono" => println!("Secou as folhas"),
        "Inverno" => println!("Frio"),
        _ => println!("Isso não existe"),
    }
}
