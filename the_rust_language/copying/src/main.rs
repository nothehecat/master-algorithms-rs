fn dump(s: &str) {
    println!("{}", s);
}

fn main() {
    let s1 = "gm anon".to_string();
    dump(&s1); // if you dont use &, it causes an error as in dump() takers ownership
    // another way to do this is to use clone
    println!("{}", s1);
}
