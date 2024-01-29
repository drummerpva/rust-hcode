enum Fruta {
    Maca, //padrão = 0, 1, 2, 3
    Banana,
    Morango,
    Abacaxi,
}
enum Coordenada {
    DoisD(i32, i32),
    TresD(i32, i32, i32),
}

struct Pessoa {
    nome: String,
    idade: u8,
    altura: f32,
}

struct Retangulo {
    altura: u32,
    largura: u32,
}
impl Retangulo {
    fn calcular_area(&self) -> u32 {
        return self.altura * self.largura;
    }
    fn new(largura: u32, altura: u32) -> Self {
        return Self { altura, largura };
    }
}

fn main() {
    enumeracao(Fruta::Abacaxi);
    enumeracao_com_dados();
    estrutura();
}

fn estrutura() {
    let anyone = Pessoa {
        altura: 1.99,
        nome: String::from("Anyone"),
        idade: 99,
    };
    println!(
        "Nome: {}, altura: {}, idade: {}",
        anyone.nome, anyone.altura, anyone.idade
    );
    let retangulo = Retangulo {
        altura: 10,
        largura: 20,
    };
    println!("Area do retangulo: {}", retangulo.calcular_area());
    let retangulo_factory = Retangulo::new(5, 12);
    println!(
        "Area do retangulo Factory: {}",
        retangulo_factory.calcular_area()
    );
}

fn enumeracao(fruta: Fruta) {
    match fruta {
        Fruta::Maca => println!("Maca: {}", fruta as i32),
        Fruta::Banana => println!("Banana: {}", fruta as i32),
        Fruta::Morango => println!("Morango: {}", fruta as i32),
        Fruta::Abacaxi => println!("Abacaxi: {}", fruta as i32),
    }
}

fn enumeracao_com_dados() {
    let ponto2d = Coordenada::DoisD(10, 20);
    let ponto3d = Coordenada::TresD(10, 20, 30);
    match ponto2d {
        Coordenada::DoisD(x, y) => println!("Coordernada 2D: {}, {}", x, y),
        _ => (),
    }
    match ponto3d {
        Coordenada::TresD(x, y, z) => println!("Coordernada 3D: {}, {}, {}", x, y, z),
        _ => {}
    }
}
