# module 1: get started

<br>


### rust is an ahead-of-time compiled language

* the `main` function in rust is always the first code that runs in every executable rust program.

* in any directory, compile and run a tust program with:


```sh
rustc main.rs
./main
```


<br>

---

### cargo

* cargo is a dependency manager and build tool, which makes adding, compiling, and managing dependencies painless and consistent across the rust ecosystem.
* a crate is a collection of rust source code files. cargo coordinates external crates in the `Cargo.toml` file. 
* you can add a new crate with `cargo add <crate name>` and update with `cargo update <crate name>`.


#### creating a new project

```sh
cargo new gm_world
```

* this command creates a cargo TOML file and a placeholder for `main.rs`:

```sh
.
├── Cargo.toml
└── src
    └── main.rs
```


#### building and running with cargo

* alternatively, in any directory, compile and run a rust program with:


```sh
cargo build
```

* this command creates a file named `Cargo.lock` (which keeps track of the versions for all dependencies), and an executable inside `target/debug/` that can be ran. 

* if you are building for release or benchmarking, add the flag `--release` to compile it with optimizations (and the target will be `release`).

* additionally, you can build and run with:

```sh
cargo run
```

* finally, to make sure your code compile in a fast manner without executing it, you can run:

```sh
cargo check
```



<br>

---

### rustfmt

* **rustfmt** is a formatting tool ensuring a consistent coding style across developers.

<br>

---

### macros

* a `!` (as in `println!`) calls a Rust macro.

<br>

---

### variables

* variables and references are immutable by default.
* constants are declared with `const` and always immutable.

<br>

----

### results

* rust handles potential failure with `Result`, which is an `enumeration` (`enum`, a type that can be in one of multiple possible states or variant).

----

### data types

* a scalar type represents a single value. there are four main scalar types in Rust.
* a compound type can group multiple values into one type. there are two primitive compound types: tuples and arrays.

#### integers

* Types are: `i8`, `u8`, `i16`, `u16`, `i32`, `u32`, `i64`, `u64`, `i128`, `u128`. In doubt, default to `i32`.
* signed or unsigned refer to whether the number can be negative, i.e., have a sign.
* each signed variant can store numbers from `-(2^{n-1})` to `2^{n-1} - 1` inclusive, where `n` is the number of bits.
* each signed variants can store numbers from `0` to `2^n - 1`.
* the `isize` and `usize` types depend on the architecture of the computer (e.g., 64 bits if you are on a 64-bit architecture).
* When Rust is compiled on debug mode, Rust will include checks for integer overflow, however when compiled with `--release`, this checks are not included and the value will "wrap around" the size of the variable (e.g. in `iu8`, 256 becomes 0).
* Rust's standard library also offers to help with the possibility of overflow, such as `wrapping_*`, `checked_*`, `overflowing_*`, and `saturating_*`.



#### floating-point numbers

* Types are `f32` and `f64`, default to `f64`.

#### booleans

```rust
fn main() {
    let t = true;

    let f: bool = false; // -> explicit type annotation
}
```

#### characters

* `char` literals are specified with single quotes (as opposed to string literals that use double quotes).
* `char` type is 4 bytes in size and represents a Unicode Scalar Value. They range form `U+0000` to `U+D7FF` and `U+E000` to `U+10FFF` inclusive.

```rust
fn main() {
    let c = 'love';
    let z: char = 'ℤ'; // with explicit type annotation
    let love = '🖤';
}
```

#### tuples

* a `tuple` is a general way of grouping many values with a variety of types.
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

#### arrays and vectors

* unlike a tuple, every element of an array must have the same type.
* arrays must have a fixed length.
* they allocate data in the stack rather than the heap.
* a vector is a similar collection type provided by the standard library that can change size.

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let b: [i32; 5] = [1, 2, 3, 4, 5];
}
```

* you can also initialize an array containing the same value (1) and a given length (4):

```rust
let c = [1; 4;
```

* Rust will panic if a *runtime error* occurs due to attempting to access the array with an invalid index.

<br>

### functions

* in function signatures, you must declare the type of each parameter (type annotations).
* function bodies are made up of a series of statements optionally ending in an expression:
    - statements are intructions that perform some action and do not return a value.
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

* functions that return values must declare their type after an arrow (`->`).