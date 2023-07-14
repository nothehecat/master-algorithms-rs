fn add_mul(x: f64, y: f64) -> (f64, f64) {
    (x + y, x * y)
}

fn main() {
    let t = add_mul(2.0, 10.0);

    // debug print
    println!("t {:?}", t);

    // 'index' the values
    println!("add {} mul {}", t.0, t.1);

    // _extract_ values
    let (add, mul) = t;
    println!("add {} mul {}", add, mul);

    // enumerate
    for (i, v) in (1..11).enumerate() {
        println!("{} {}", i, v);
    }

    // enumerate with a slice
    for t in ["zero","one","two"].iter().enumerate() {
        print!(" {} {};",t.0, t.1);
    }

    // with zip
    let names = ["ten", "hundred", "thousand"];
    let nums = [10, 100, 1000];
    for p in names.iter().zip(nums.iter()) {
    println!(" {} {};", p.0, p.1);
}
}
