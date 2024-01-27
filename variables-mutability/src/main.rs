fn type_of<T>(_variavel: T) -> &'static str {
    return std::any::type_name::<T>();
}

fn main() {
    // Estáticamente tipada com inferência de tipos
    // Escalares: integer, float, boolean, char
    // integer: unsigned (não aceita valores negativos "u23") e signed (aceita valores negativos "i23")
    // float: f32 e f64
    // boolean: true e false
    // char: qualquer valor dentro de aspas simples(Rust usa Unicode por padrão ao invés de ASCII como as outras linguagens)

    // Compostos: tuple e array (ambos são fixos)
    // tuple: pode conter valores de tipos diferentes (ex: (i32, f64, char, u8))
    // array: pode conter valores de um único tipo (ex: [i32; 5] = [1, 2, 3, 4, 5])
    let inteiro = 10;
    let float = inteiro as f64;
    let string = float.to_string();
    // let new_float: f64 = string.into();
    let new_float: f64 = string.parse().unwrap();
    let _new_inteiro = new_float as i32;
    let boolean = true;
    let bool_string: String = boolean.to_string();
    let _new_boolean: bool = bool_string.parse().unwrap();
    let bool_from_value: bool = "true".parse().expect("Erro ao converter para boolean");
    println!("type_of(inteiro): {}", type_of(inteiro));
    println!("type_of(bool_from_value): {}", type_of(bool_from_value));
}
