fn leet_or_not(leet: bool) -> Result<i32,String> {
    if leet {
        Ok(1337)
    } else {
        Err("no".to_string())
    }
}

fn main() {

    let text = "gm, anon";

    // match statement
    match text.find('m') {
        Some(idx) => {
            println!("found m at {}", idx);
            let hi = &text[idx..];
            println!("hi {}", hi);
        },
        None => println!("couldn't find the greeting")
    };

    // match without failure
    if let Some(idx) = text.find('n') {
        println!("hello {}", &text[idx..]);
    }

    // as a switch statement
    let n = 2;
    match n {
        0 => println!("zero"),
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("many"),
    };


    // with a function
    println!("{:?}",leet_or_not(true));
    println!("{:?}",leet_or_not(false));
    
      match leet_or_not(true) {
        Ok(n) => println!("Cool, you are {}",n),
        Err(e) => println!("Huh, {}",e)
    }
}

