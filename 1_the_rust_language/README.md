# module 1: the Rust language

<br>


## 🐚 rust as an ahead-of-time compiled language

<br>

* the `main` function in rust is always the first code that runs in every executable rust program.

* in any directory, compile and run a Rust program with:


```sh
rustc main.rs
./main
```


<br>



### cargo

<br>

* cargo is a dependency manager and build tool, which makes adding, compiling, and managing dependencies painless and consistent across the Rust ecosystem.
* a crate is a collection of Rust source code files. cargo coordinates external crates in the `Cargo.toml` file. 
* you can add a new crate with `cargo add <crate name>` and update with `cargo update <crate name>`.

<br>

#### creating a new project

<br>

```sh
cargo new gm_world
```

<br>

* this command creates a cargo `TOML` file and a placeholder for `main.rs`:

```sh
.
├── Cargo.toml
└── src
    └── main.rs
```

<br>

#### building and running with cargo

<br>

* alternatively, in any directory, compile and run a rust program with:


```sh
cargo build
```

<br>

* this command creates a file named `Cargo.lock` (which keeps track of the versions for all dependencies), and an executable inside `target/debug/` that can be run. 

* if you are building for release or benchmarking, add the flag `--release` to compile it with optimizations (and the target will be `release`).

* additionally, you can build and run with:

```sh
cargo run
```

<br>

* finally, to make sure your code compiles in a fast manner without executing it, you can run:

```sh
cargo check
```

<br>


* **rustfmt** is a formatting tool ensuring a consistent coding style across developers.

<br>


---

## 🐚 variables

<br>

* variables and references are immutable by default.
* constants are declared with `const` and always immutable.
* rust handles potential failure with `Result`, which is an `enumeration` (`enum`, a type that can be in one of multiple possible states or variant).
* a scalar type represents a single value. there are four main scalar types in Rust.
* a compound type can group multiple values into one type. there are two primitive compound types: tuples and arrays.

<br>

### integers

* Types are: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`. In doubt, default to `i32`.
* signed or unsigned refers to whether the number can be negative, i.e., have a sign.
* each signed variant can store numbers from `-(2^{n-1})` to `2^{n-1} - 1` inclusive, where `n` is the number of bits.
* each signed variables can store numbers from `0` to `2^n - 1`.
* the `isize` and `usize` types depend on the architecture of the computer (e.g., 64 bits if you are on a 64-bit architecture).
* When Rust is compiled on debug mode, Rust will include checks for integer overflow, however when compiled with `--release`, these checks are not included and the value will "wrap around" the size of the variable (e.g. in `iu8`, 256 becomes 0).
* Rust's standard library also offers to help with the possibility of overflow, such as `wrapping_*`, `checked_*`, `overflowing_*`, and `saturating_*`.


<br>

### floating-point numbers

* Types are `f32` and `f64`, default to `f64`.

<br>

### booleans

```rust
fn main() {
    let t = true;

    let f: bool = false; // -> explicit type annotation
}
```

<br>

### characters

* `char` literals are specified with single quotes (as opposed to string literals that use double quotes).
* `char` type is 4 bytes in size and represents a Unicode Scalar Value. They range from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFF` inclusive.

```rust
fn main() {
    let c = 'love';
    let z: char = 'ℤ'; // with explicit type annotation
    let love = '🦀';
}
```

<br>

### tuples

* a `tup` is a general way of grouping many values with a variety of types.
* they have a fixed length (can't change once declared).

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

```rust
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");
}
```

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

<br>

### arrays 

<br>

* unlike a tuple, every element of an array must have the same type.
* arrays must have a fixed length.
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

* examples how you can print arrays in Rust:

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


* Rust will panic if a *runtime error* occurs when attempting to access the array with an invalid index. however, there is a slice method `get` which does not panic:

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


### vectors

* a vector is a similar collection type provided by the standard library that can change size (they behave like Python's lists).

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

---

## 🐚 functions

<br>

* in function signatures, you must declare the type of each parameter (type annotations).
* function bodies are made up of a series of statements optionally ending in an expression:
    - statements are instructions that perform some action and do not return a value.
    - expressions evaluate to a resultant value. calling a macro or a function is an expression.
* expressions do not include ending semicolons. if you add a semicolon to the end of an expression, you turn it into a statement (and it will not return a value).

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

<br>

* functions that return values must declare their type after an arrow (`->`).

```rust
fn plus_one(x: i32) -> i32 {
    x + 1
}
```

<br>

* a `!` (as in `println!`) calls a Rust macro.

<br>

### control flow

<br>

* A typical `if` loop in Rust:

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

<br>

* Rust allows you use `if` in a `let` statement:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

<br>

* In Rust, there are three kinds of loops: `loop`, `while`, and `for`.
* `break` and `continue` can be used to break from the loop.

<br>

### `loop`

<br>

* `loop` can be used to retry an operation that might fails, such as checking whether a thread has completed its job.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

<br>

* when there are multiple loops, Rust provides loop labels for `break` or `continue`.

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```



<br>

* a `for` loop tends to be faster and less error prone than a `while` loop.

```rust
fn main() {

    for number in (1..4) {
        println!("the value is: {number}");
    }
}
```

<br>

---

## 🐚 ownership

<br>

* ownership is a set of rules that govern how Rust manages memory.
* While some languages have garbage collection and others need explicit memory de-allocation, Rust uses a system of ownership with a set of rules that the compiler checks (so that the program won't compile if any of the rules are violated).

<br>

### stack vs. heap

<br>

* allocating a space in the heap (and returning its pointer) is slower than pushing to the stack. it also requires more work because the allocator must find a larger enough space to hold the data.
* keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap are all problems that ownership addresses.

* the ownership rules in Rust are:
    - each value has an owner
    - there can only be one owner at a time
    - when the owner goes out of scope, the value will be dropped

<br>

### the `String` type

<br>

* `String` is a more complex variable type on which the data is stored in the heap, instead of the stack. This allows an amount of text that is unknown at the compile time.


```rust
let s = String::from("hello");
```

<br>


* the double colon `::` operator allows to namespace a particular function (`from`), under the `String`. With `String::from`, the memory is requested from the memory allocator at runtime. 
* in terms of freeing the memory: in Rust, the memory is automatically returned once the variable that owns it goes out of scope (at the closing curly bracket), by a function called `drop`. At that point, the author of `String` can put the code to return the memory.
* a `String` is made up of three parts: a pointer to the heap memory that holds the contents of the string, a length (how much memory in bytes), and a capacity (all stored on the stack).
* if we copy the value of one variable to another (as below), to guarantee memory safety, the first variable is no longer valid (similar to *shallow copy* in Python, but it's called *move* in Rust):

```rust
let s1 = String::from("hello");
let s2 = s1;
```

<br>

#### `clone`

* Rust will never automatically create "deep" copies of your data (default to be inexpensive in terms of runtime performance).
* To deeply copy the heap data of the `String`, not just the stack data (the pointer), Rust offers the methods `clone` (which is more expensive):

```rust
let s1 = String::from("gm anon");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

* Remember that integers have a known size at compile time and are stored entirely on the stack, therefore copies are quick and there is no difference between deep and shallow copy.

<br>

#### `Copy`

* Rust has an annotation called `Copy`, that can place on types that are stored on the stack.
* with the `Copy` trait, variables do not move, but are trivially copied, making them still valid after assignment to another variable.
* if the type has implemented `Drop` trait, Rust won't let us annotate a type with `Copy`.

<br>

