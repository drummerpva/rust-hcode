use std::{
    env,
    fs::{metadata, read_dir, File},
    io::{Read, Write},
};

pub fn obter_caminho_usuario() -> Option<String> {
    if let Some(caminho_home) = env::var_os("HOME") {
        return Some(caminho_home.into_string().unwrap());
    } else {
        return None;
    }
}
pub fn criar(caminho: &str, arquivo: &str) {
    let caminho_completo = format!("{}/{}", caminho, arquivo);
    match File::create(&caminho_completo) {
        Ok(mut arquivo) => {
            println!("Arquivo criado com sucesso: {}", caminho_completo);
            let texto_arquivo = "Ola, mundo!";
            arquivo.write(texto_arquivo.as_bytes()).unwrap();
        }
        Err(erro) => println!("Erro ao criar arquivo: {}", erro.to_string()),
    }
}
pub fn ler(caminho_completo: &str) {
    if existe(caminho_completo).is_err() {
        println!("Arquivo não existe!");
        return;
    }
    match File::open(&caminho_completo) {
        Ok(mut arquivo) => {
            let mut conteudo = String::new();
            arquivo.read_to_string(&mut conteudo).unwrap();
            println!("Arquivo aberto: {conteudo}")
        }
        Err(erro) => println!("Erro ao abrir arquivo: {}", erro.to_string()),
    }
}

pub fn existe(caminho_completo: &str) -> Result<bool, &'static str> {
    if metadata(caminho_completo).is_ok() {
        return Ok(true);
    }
    return Err("Arquivo não existe!");
}

pub fn ler_diretorio(caminho: &str) -> Result<(), std::io::Error> {
    let items = read_dir(caminho)?;
    for item in items {
        let item = item?;
        let item_caminho = item.path();
        if item_caminho.is_dir() {
            println!("Pasta: {}", item_caminho.display());
        }
        if item_caminho.is_file() {
            println!("Arquivo: {}", item_caminho.display());
        }
    }

    return Ok(());
}
