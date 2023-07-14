# 🦀 𝟘𝟘𝟘𝟙. the **Rust** language

<br>


##  **Rust** as an ahead-of-time compiled language

<br>

* let's start with the very basics. the `main` function in **Rust** is always the first code that runs in every executable **Rust** program.

* in any directory, compile and run a **Rust** program with:


```sh
rustc main.rs
./main
```


<br>

----

## cargo

<br>

* you rarely use `rustc`, though. instead, **[cargo](https://doc.rust-lang.org/cargo/)** is a dependency manager and build tool, which makes adding, compiling, and managing dependencies painless and consistent across the **Rust** ecosystem.
* a crate is a collection of **Rust** source code files. cargo coordinates external crates in the `Cargo.toml` file. 
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

* alternatively, in any directory, compile and run a **Rust** program with:


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


* cargo stores the output of a build into a `target` directory, in the root of the workspace, for either `debug/` and `release`.

<br>

---
## other in-box tools

<br>

* **[rustc](https://doc.rust-lang.org/rustc/index.html)** is the compiler for **Rust**, provided by the project itself (as we saw in the beginning of this doc), which includes built-in testing and linting.
* **[rustfmt](https://rust-lang.github.io/rustfmt/?version=v1.6.0&search=)** is a formatting tool ensuring a consistent coding style across developers.
* **[rustdocs](https://doc.rust-lang.org/rustdoc/index.html)** is Rust's standard tool called that generates markdown or HTML documentation for **Rust** projects. files should start with ` //!` to indicate module-level or crate-level documentation.
* **rustup** can be used to run **Rust** standard documentation in your browser (with `rustup doc --std`).

<br>

---
## traits

<br>

* traits allow you to abstract behavior that can be shared by different types, so that the code can express ideas in very generic and flexible ways.
* traits can be the descriptive line on top of functions:

```rust
#[test]
fn find_a_match() {
}
```

* each operator (like `+=`) corresponds to a trait, like an abstract interface that must be implemented for each concrete type.


<br>


---

##  variables

<br>

* variables and references are immutable by default, to make the mutable, you must add `mut`.
* what makes something a 'variable' is that it gets assigned a computed value at runtime (it's not a constant).
* constants are declared with `const` and always immutable.
* values can be passed by reference, which is created by `&` and dereferenced by `*`. passing by reference is important when we have a large object and don't wish to copy it.

```rust
fn by_ref(x: &i32) -> i32 {
    *x + 1
}

fn main() {
    let i = 10;
    let res1 = by_ref(&i);
    let res2 = by_ref(&41);
    println!("{} {}", res1,res2);
}
```

<br>

* **Rust** handles potential failure with `Result`, which is an `enumeration` (`enum`, a type that can be in one of multiple possible states or variant).
* a scalar type represents a single value. there are four main scalar types in Rust.
* a compound type can group multiple values into one type. there are two primitive compound types: tuples and arrays.
* in **Rust**, variables of a type can be casted to another:

```rust
fn main() {
    let mut sum = 0.0;
    for i in 0..5 {
        sum += i as f64;
    }
    println!("sum is {}", sum);
}
```


<br>

### integers

<br>

* Types are: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`. In doubt, default to `i32`.
* signed or unsigned refers to whether the number can be negative, i.e., have a sign.
* each signed variant can store numbers from `-(2^{n-1})` to `2^{n-1} - 1` inclusive, where `n` is the number of bits.
* each signed variables can store numbers from `0` to `2^n - 1`.
* the `isize` and `usize` types depend on the architecture of the computer (e.g., 64 bits if you are on a 64-bit architecture).
* When **Rust** is compiled on debug mode, **Rust** will include checks for integer overflow, however when compiled with `--release`, these checks are not included and the value will "wrap around" the size of the variable (e.g. in `iu8`, 256 becomes 0).
* Rust's standard library also offers to help with the possibility of overflow, such as `wrapping_*`, `checked_*`, `overflowing_*`, and `saturating_*`.


<br>

### floating-point numbers

<br>

* Types are `f32` and `f64`, default to `f64`.

<br>

### booleans

<br>

```rust
fn main() {
    let t = true;

    let f: bool = false; // -> explicit type annotation
}
```

<br>

### characters

<br>

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

<br>

* a `tup` is a general way of grouping many values with various types.
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

---

##  functions

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


#### macros

<br>

* a `!` (as in `println!`) calls a **Rust** macro (see more details on **[formatted_print](formatted_print/)**).
* `{}` is the default placeholder type that works for numbers and strings, but not all the types. in other cases, for example `vec`, the debug representation `{:?}` works.
* note that printing to the terminal is very slow. you want to reduce the number of writes that "flush" to the terminal. you can also wrap your `stdout` handle in a **[BugWriter](https://doc.rust-lang.org/1.39.0/std/io/struct.BufWriter.html)**, which by default, buffers up to 8 kB.

<br>

```rust
#![allow(unused)]
fn main() {
use std::io::{self, Write};

let stdout = io::stdout();
let mut handle = io::BufWriter::new(stdout); //  wrap that handle in a buffer
writeln!(handle, "foo: {}", 1337); 
}
```


<br>

#### `Result` and errors

* functions in **Rust** usually don't return a string, but instead, they return a **[Result](https://doc.rust-lang.org/1.39.0/std/result/index.html)**, a `enum` that contains either a `String` or an error of some type (for instance **[std::io::Error](https://doc.rust-lang.org/1.39.0/std/io/type.Result.html)**).

```rust
enum Result<T, E> {
   Ok(T),
   Err(E),
}
```

<br>

* you can use `match` to check which variant is (all `match` blocks need to return things of the same type):

```rust
let result = std::fs::read_to_string(FILENAME)
match result {
    Ok(content) => { println!("Nice file {}", contant); }
    Err(error) => { panic!("Nope {}, error); }
```

<br>

* this `match` with `panic` also can be achieved with `.unwrap()`:

```rust
let result = std::fs::read_to_string(FILENAME).unwrap();
```

<br>

* if we don't want to exit the program with `panic`, we can `return`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = std::fs::read_to_string(FILENAME);
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); }
    };
    println!("file content: {}", content);
    Ok(())  // Note that we can omit "return", as long as no ;
}
```

<br>

* `Box<dyn std::error:Error>` is a `Box` that contains any type that implements the standard error trait.
* this `match` with `return Err()` can be achieved with `?`:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(FILENAME)?;
    println!("file content: {}", content);
    Ok(())
}
```

<br>

* we could also create our own error type, with a `struct`:

```rust
#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<(), CustomError> {
    let content = std::fs::read_to_string(FILEPATH)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}", FILEPATH, err)))?;
    println!("file content: {}", content);
    Ok(())
}
```

<br>

* or using the **[anyhow crate](https://docs.rs/anyhow)**:

```rust
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let content = std::fs::read_to_string(FILEPATH)
        .with_context(|| format!("Error reading `{}`", FILEPATH))?;
    println!("file content: {}", content);
    Ok(())
}
```

<br>

---
## control flow

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

* **Rust** allows you to use `if` in a `let` statement:

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
```

<br>

* In **Rust**, there are three kinds of loops: `loop`, `while`, and `for`.
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

* when there are multiple loops, **Rust** provides loop labels for `break` or `continue`.

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

* a `for` loop tends to be faster and less error-prone than a `while` loop.

```rust
fn main() {

    for number in (1..4) {
        println!("the value is: {number}");
    }
}
```

<br>

---

## ownership

<br>

* ownership is a set of rules that govern how **Rust** manages memory.
* While some languages have garbage collection and others need explicit memory de-allocation, **Rust** uses a system of ownership with a set of rules that the compiler checks (so that the program won't compile if any rules are violated).

<br>

### stack vs. heap

<br>

* allocating data on the stack is fast, but limited (~megabytes), allocating data on the heap is expensive and the memory needs to be freed later (~gigabytes).
* allocating a space in the heap (and returning its pointer) is slower than pushing to the stack. it also requires more work because the allocator must find a larger enough space to hold the data.
* keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap, and cleaning up unused data on the heap are all problems that ownership addresses.
* when a vector is modified or created, it allocates from the heap and becomes the memory owner. the slice borrows the memory from the vector and when the vector dies or drop, the memory goes.

* the ownership rules in **Rust** are:
    - each value has an owner
    - there can only be one owner at a time
    - when the owner goes out of scope, the value will be dropped

<br>


---

