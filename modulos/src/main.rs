mod matematica {
    pub fn somar(a: i32, b: i32) -> i32 {
        return a + b;
    }
    pub fn subtrair(a: i32, b: i32) -> i32 {
        return a - b;
    }
}

fn main() {
    println!("2 + 2 : {}", matematica::somar(2, 2));
    println!("2 - 2 : {}", matematica::subtrair(2, 2));
}
