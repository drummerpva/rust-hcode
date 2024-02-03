use std::{env, fs::File};

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
        Ok(_) => println!("Arquivo criado com sucesso: {}", caminho_completo),
        Err(erro) => println!("Erro ao criar arquivo: {}", erro.to_string()),
    }
}
