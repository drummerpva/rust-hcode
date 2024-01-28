fn main() {
    println!("Hello, world!");
    nome_da_funcao();
    let with = com_return();
    let without = sem_return();
    println!("with return: {with}");
    println!("without return: {without}");
    println!("maior valor entre 10 e 20: {}", maior_valor(10, 20));
    let a = 10;
    println!("incrementa {a}: {}", incrementa(a));
}

fn nome_da_funcao() {
    println!("Olá HCode");
}

fn com_return() -> i32 {
    return 3;
}

fn sem_return() -> i32 {
    7
}

fn maior_valor(a: i32, b: i32) -> i32 {
    return if a > b { a } else { b };
}

fn incrementa(mut a: i32) -> i32 {
    a += 1;
    a
}
