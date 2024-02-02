use std::collections::HashMap;

// Vector(Vec)
// String
// Hash Map (HashMap) chave-valor "JSON"
fn main() {
    vetor();
    string();
    hash_map();
}

fn vetor() {
    let lista_array: [u8; 5] = [10, 20, 30, 40, 50];
    println!("Valor do array na posição 2 {}", lista_array[2]);

    let mut lista_vec: Vec<u8> = Vec::new();
    lista_vec.push(10);
    lista_vec.push(20);
    lista_vec.push(30);
    lista_vec.push(40);
    lista_vec.push(50);
    println!("Valores do vetor {:?}", lista_vec);
    for valor in lista_vec {
        println!("Valor atual do valor {valor}");
    }
}

fn string() {
    // let texto = String::new();
    let mut texto = String::from("Hcode");
    texto.push(' ');
    texto.push_str("Treinamentos");
    println!("Texto: {texto}");
}

fn hash_map() {
    let mut mapa: HashMap<String, i32> = HashMap::new();
    mapa.insert("id".into(), 32);
    mapa.insert("idade".to_string(), 99);
    mapa.insert("peso".to_owned(), 80);
    println!("Mapa: {:?}", mapa);

    for (chave, valor) in &mapa {
        println!("{chave}: {valor}");
    }
    println!("GET Idade: {}", mapa.get("idade").unwrap());
}
