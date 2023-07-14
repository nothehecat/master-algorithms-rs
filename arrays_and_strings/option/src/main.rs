fn main() {

    // simple example
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);
    
    let x: Option<u32> = None;
    assert_eq!(x.is_some(), false);

    // matching example
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some_and(|x| x > 1), true);

    let x: Option<u32> = Some(0);
    assert_eq!(x.is_some_and(|x| x > 1), false);

    let x: Option<u32> = None;
    assert_eq!(x.is_some_and(|x| x > 1), false);

}
