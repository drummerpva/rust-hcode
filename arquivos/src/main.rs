mod arquivo;

fn main() {
    println!("{}", arquivo::obter_caminho_usuario().unwrap());
    arquivo::criar("./temp", "arquivo.txt");
}
