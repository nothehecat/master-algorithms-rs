# 🦀 𝟘𝟘𝟙𝟘. arrays, vectors, strings

<br>

## arrays

<br>

* all statically-typed languages have arrays, which are values packed in memory.
* unlike a tuple, every element of an array must have the same type.
* arrays must have a fixed length. they can be mutable but you cannot add new elements.
* they allocate data in the stack rather than the heap.


```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
}
```

<br>

* you can also initialize an array containing the same value (1) and a given length (4):

```rust
let c = [1; 4;
```

<br>

* examples of how you can print arrays in Rust:

```rust
fn main() {
    let ints = [1, 2, 3];
    let floats = [1.1, 2.1, 3.1];
    let strings = ["hello", "world"];
    let ints_ints = [[1, 2], [10, 20]];
    println!("ints {:?}", ints);
    println!("floats {:?}", floats);
    println!("strings {:?}", strings);
    println!("ints_ints {:?}", ints_ints);
}
```

<br>

which results to:

```rust
ints [1, 2, 3]
floats [1.1, 2.1, 3.1]
strings ["hello", "world"]
ints_ints [[1, 2], [10, 20]]
```

<br>


* **Rust** will panic if a *runtime error* occurs when attempting to access the array with an invalid index. however, there is a slice method `get` which does not panic:

```rust
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);
}
```

<br>

---
## slices

<br>

* slices are more frequently used than arrays. they are views into an underlying array of values, in the sense that slices give you different views of the same array where a copy of data is never made.
* slices keep track of their size and will panic if you try to access outside that size.
* you explicitly say you want to pass an array as a slice by using `&` (call it "borrow", and note that it remains owned by the original owner).


```rust
fn sum(values: &[i32]) -> i32 {
    let mut res = 0;
    for i in 0..values.len() {
        res += values[i]
    }
    res
}

fn main() {
    let arr = [10,20,30,40];
    // look at that &
    let res = sum(&arr);
    println!("sum {}", res);
}
```

<br>

---
## `Option`

<br>

* slices can look like lists in Python, but a copy of the data is never made. the size of an array is known at compile-time, but the size of the slice is only known at run-time. so `array[i]` can cause out-of-bounds error and panic. the slice method `get` dos not panic, it returns `None`. 
* `get` returns an `Option`, which is either `Some` or `None`.

```rust
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;
    let first = slice.get(0);
    let last = slice.get(5);

    println!("first {:?}", first);
    println!("last {:?}", last);
    
    println!("first {} {}", first.is_some(), first.is_none());
    println!("last {} {}", last.is_some(), last.is_none());
    println!("first value {}", first.unwrap());
}
```

<br>

* you can think of `Option` as a box that can contain a value (its type parameter) or `None`. so, for instance, a full type could be `Option<&i32>`. [here](https://doc.rust-lang.org/std/option/enum.Option.html) is the `std` reference for `Option`:

```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

<br>

---
## iterators

<br>

*  the iterator `0..n` is similar to Python's `range`. it's an object with a `next` method that returns an `Option`. as long as the value is not `None`, it will keep calling `next`.

```rust
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
}
```


<br>

* it's more efficient to iterate over an array or slice this way than to use `for i in 0..slice.len() {}` because **Rust** does not have to check every index operation.
* in addition, the `windows` method gives you an iterator of slices:

```rust
fn main() {
    let ints = [1, 2, 3, 4, 5];
    let slice = &ints;

    for s in slice.windows(2) {
        println!("window {:?}", s);
    }
}
```

<br>

* and the `chunks` method:

```rust
for s in slice.chunks(2) {
        println!("chunks {:?}", s);
}
```


<br>

---
## vectors

<br>

* a vector is a similar collection type provided by the standard library that can change size (they behave like Python's lists). they are allocated dynamically on the heap.
* they behave very much like an slice, the difference is that you can append extra values to a vector (if it's declared as mutable).

```rust
fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    let first = v[0];  // will panic if out-of-range
    let maybe_first = v.get(0);

    println!("v is {:?}", v);
    println!("first is {}", first);
    println!("maybe_first is {:?}", maybe_first);
}
```

<br>

which can be sliced like:

```rust
fn dump(arr: &[i32]) {
    println!("arr is {:?}", arr);
}

fn main() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v.push(30);

    dump(&v);

    let slice = &v[1..];
    println!("slice is {:?}", slice);
}
```

<br>

* the macro `vec!` can be used for initalizing a vector.
* values can be removed form the end of a vector with `pop`.
* vectors have a size and a capacity: if you `clear` a vector, it becomes zero but it still retains its old capacity. in this case, refilling it with `push` only requires reallocation when the size gets larger than the capacity.
* vectors can be sorted in-place with `sort` and the duplicates can be removed with `dedup`.
* vectors can be copied with `clone`.

```rust
fn main() {
    let mut v1 = vec![1, 10, 5, 1, 2, 11, 2, 40];
    v1.sort();
    v1.dedup();
    assert_eq!(v1, &[1, 2, 5, 10, 11, 40]);
}
```



<br>

---


## `String` 

<br>

* the`String` type, like `Vec`, allocates dynamically and is resizeable.
* it's more complex variable type on which the data is stored in the heap, instead of the stack. This allows an amount of text that is unknown at the compile time.


```rust
let s = String::from("hello");
```

<br>


* the double colon `::` operator allows to namespace a particular function (`from`), under the `String`. With `String::from`, the memory is requested from the memory allocator at runtime. 
* in terms of freeing the memory: in **Rust**, the memory is automatically returned once the variable that owns it goes out of scope (at the closing curly bracket), by a function called `drop`. At that point, the author of `String` can put the code to return the memory.
* a `String` is made up of three parts: a pointer to the heap memory that holds the contents of the string, a length (how much memory in bytes), and a capacity (all stored on the stack).
* if we copy the value of one variable to another (as below), to guarantee memory safety, the first variable is no longer valid (similar to *shallow copy* in Python, but it's called *move* in Rust):

```rust
let s1 = String::from("hello");
let s2 = s1;
```

<br>

* a system language has to have two kinds of string, allocated and static.
* in embedded micros, the system language should be able to store statically strings in the executable itself. this means putting strings in a cheap ROM rather than expensive RAM (for lower-power devices, RAM is also expensive in terms of power consumption).
* a string "gm, anon" is not of the type `String`, but of `&str` (string slice). it's the distinction between `const char*` and `std::string` in C++, except `&str` is smarter. in fact, `&str` and `String` have the same relationship than `&[T]` to `Vec<T>` (the borrow operator coerce `String` into `&str` just as `Vec<T>` could be coerced into `&[T]`).

```rust
fn dump(s: &str) {
    println!("str '{}'", s);
}

fn main() {
    // the string slice
    let text = "gm anon"; 

    // it's now an allocated string 
    let other_text = text.to_string(); 

    dump(text);
    dump(&other_text);
}
```

<br>

* under the hood, `String` is a `Vec<u8>` and `&str` is `&[u8]`, where bytes must represent valid UTF-8 text. like a vector, you can use `push` and `pop`.

<br>

### `to_string()` and `format!`

<br>

* many types can be converted to strings with `to_string()`.
* the macro `format!` is useful to build complicated strings and it uses the same format strings as `println!`.
* however, you cannot index strings, since they one UTF-8 encoding, where a character can be a number of bytes. strings are not arrays of chars!

<br>

### `collect()`, `extend()`, `split_whitespace()`

<br>

* string slicing does not work because it uses byte offsets (i.e. two bytes). the method `split_whitespace()` returns an iterator.
* the method `collect()` puts things together.

```rust
let text = "gm anon";
let words: Vec<&str> = text.split_whitespace().collect();
```

<br>

* or using `extend()`:

```rust
let text = "the red fox and the lazy dog";
let words: Vec<&str> = text.split_whitespace().collect();
```

<br>

* or putting back together:

```rust
let stripped: String = text.chars()
    .filter(|ch| ! ch.is_whitespace()).collect();
```


<br>

### `clone`

* **Rust** will never automatically create "deep" copies of your data (default to be inexpensive in terms of runtime performance).
* To deeply copy the heap data of the `String`, not just the stack data (the pointer), **Rust** offers the methods `clone` (which is more expensive):

```rust
let s1 = String::from("gm anon");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```


<br>

* remember that integers have a known size at compile time and are stored entirely on the stack, therefore copies are quick and there is no difference between deep and shallow copy.

<br>

### `Copy`

* **Rust** has an annotation called `Copy`, that can place on types that are stored on the stack.
* with the `Copy` trait, variables do not move, but are trivially copied, making them still valid after assignment to another variable.
* if the type has implemented `Drop` trait, **Rust** won't let us annotate a type with `Copy`.

<br>


---

## `filter`

<br>

* the `filter` method takes a closure (lambdas or anonymous functions).


<br>

---

## examples

<br>

- [arrays](simple_arrays/)
- [iterators](iterators/)
- [slices](slice/)
- [option](option/)
- [strings](strings/)

