fn main() {

    // iterators example
    let mut iter = 0..3;
    assert_eq!(iter.next(), Some(0));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);

    // or with a for loop
    let arr = [1, 2, 3];
    for i in arr.iter() {
        println!("{}", i);
    }

    // slices will be converted implicitly to iterators
    let slice = &arr;
    for i in slice {
        println!("{}", i);
    }

    // using windows for a sliding window
    for s in slice.windows(2) {
        println!("window {:?}", s);
    }

    // using chunks
    for s in slice.chunks(2) {
        println!("chunks {:?}", s);
    }
}
