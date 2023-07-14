fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

fn main() {

    // Vectors are like arrays, but can grow and shrink
    // and they are allocated dynamically on the heap
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];  // will panic if out-of-range
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);

    // relating to slices
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    dump(&v); // coerce to slice

    let slice = &v[1..];
    println!("slice is {:?}", slice);

    // using pop and extend
    let mut v1 = vec![10, 20, 30, 40];
    v1.pop();

    let mut v2 = Vec::new();
    v2.push(10);
    v2.push(20);
    v2.push(30);

    assert_eq!(v1, v2);

    v2.extend(0..2);
    assert_eq!(v2, &[10, 20, 30, 0, 1]);

    // sorting and deduping
    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v1.sort();
    v1.dedup();
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
}
    

