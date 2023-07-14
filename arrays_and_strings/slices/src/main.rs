// slices give views of arrays, but a copy of the data is not made

fn main() {
    let ints = [1, 2, 3, 4, 5];

    // slices with range
    let slice1 = &ints[0..2];
    let slice2 = &ints[1..];  // open range

    println!("ints {:?}", ints);
    println!("slice1 {:?}", slice1);
    println!("slice2 {:?}", slice2);

    // slices with get
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);
    
    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());

    // dealing with slices that are out of bounds
    let maybe_last = slice.get(5);
    let last = if maybe_last.is_some() {
        *maybe_last.unwrap() //  the precise type inside the Some is &i32, which is a reference
    } else {
        -1
    };

    // or a simpler notation
    let last = *slice.get(5).unwrap_or(&-1);
    println!("last {}", last);

}
