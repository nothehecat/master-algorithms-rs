#[derive(Debug)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool)
}

fn main() {
    use Value::*;
    let n = Number(2.3);
    let s = Str("gm".to_string());
    let b = Bool(true);

    println!("n {:?} s {:?} b {:?}", n,s,b);
}
