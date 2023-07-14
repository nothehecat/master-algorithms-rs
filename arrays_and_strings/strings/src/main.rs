// several ways to create a string

fn dump(s: &str) {
    println!("str '{}'", s);
}

fn array_to_str(arr: &[i32]) -> String {
    let mut res = '['.to_string();
    for v in arr {
        res += &v.to_string(); // the operator is defined on a string slice, not a String so we need to coerce
        res.push(',');
    }
    res.pop();
    res.push(']');
    res
}

fn main() {
    // the string slice
    let text = "gm anon"; 

    // it's now an allocated string 
    let other_text = text.to_string(); 

    dump(text);
    dump(&other_text);

    // create a string from a vector of integers
    let arr = array_to_str(&[10, 20, 30]);
    let res = format!("hello {}", arr);
    
    assert_eq!(res, "hello [10,20,30]");

    // more string methods
    let text = "gm! привет!";
    for ch in text.chars() {
        print!("'{}' ", ch);
    }
    println!("");
    println!("len {}", text.len());
    println!("count {}", text.chars().count());

    let maybe = text.find('п');
    if maybe.is_some() {
        let hi = &text[maybe.unwrap()..];
        println!("Russian hi {}", hi);
    }

    // splitting strings
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("words: {:?}", words);

    // or using extend
    let mut words = Vec::new();
    words.extend(text.split_whitespace());  
    println!("words: {:?}", words);

    // and putting them back together
    let stripped: String = text.chars()
        .filter(|ch| ! ch.is_whitespace()).collect();
    println!("stripped: {}", stripped);
    
}
