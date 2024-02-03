mod arquivo;

fn main() {
    println!("{}", arquivo::obter_caminho_usuario().unwrap());
    arquivo::criar("./temp", "arquivo.txt");
    arquivo::ler("./temp/arquivo.txt");
    arquivo::ler_diretorio(".").expect("Erro ao ler diretório");
}
