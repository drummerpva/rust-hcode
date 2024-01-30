use std::{
    fs::File,
    io::{self, Read},
    panic::catch_unwind,
};

fn main() {
    // funcao_com_panic(0);
    let resultado = catch_unwind(|| {
        let a = funcao_com_panic(0);
        return Ok::<i32, &str>(a);
    });
    match resultado {
        Ok(value) => {
            println!("Ok: {}", value.unwrap());
        }
        Err(_) => {
            println!("A função resultou em panic!");
        }
    }
    let arquivo_invalido = ler_arquivo("arquivo_invalido.txt");
    match arquivo_invalido {
        Ok(value) => {
            println!("Conteúdo do arquivo: {}", value);
        }
        Err(error) => {
            println!("Erro ao ler arquivo! {}", error.to_string());
        }
    }
    let arquivo_valido = ler_arquivo("Cargo.toml");
    match arquivo_valido {
        Ok(value) => {
            println!("Conteúdo do arquivo: \n{}", value);
        }
        Err(_) => {
            println!("Erro ao ler arquivo!");
        }
    }
    match dividir(100., 2.) {
        Some(value) => {
            println!("Dividir: {}", value);
        }
        None => {
            println!("Erro na divisão");
        }
    }
    match dividir(100., 0.) {
        Some(value) => {
            println!("Dividir: {}", value);
        }
        None => {
            println!("Erro na divisão");
        }
    }
    // println!("Dividir: {}", dividir(100., 0.).expect("Erro na divisão"));
}

fn ler_arquivo(caminho: &str) -> Result<String, io::Error> {
    let mut arquivo = File::open(caminho)?;
    let mut conteudo = String::new();
    arquivo.read_to_string(&mut conteudo)?;
    return Ok(conteudo);
}

fn funcao_com_panic(valor: i32) -> i32 {
    if valor == 0 {
        panic!("Valor não pode ser zero");
    }
    return valor;
}

fn dividir(dividendo: f64, divisor: f64) -> Option<f64> {
    if divisor == 0. {
        None
    } else {
        Some(dividendo / divisor)
    }
}
