fn main() {
    let s = "gm anon".to_string();
    let mut r = &s;
    {
        let s = "gm anon".to_string();
        r = &s;
    }   
    println!("this will cause an error: {}", r);
}
