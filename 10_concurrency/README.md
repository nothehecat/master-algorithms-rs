# 🦀 𝟙𝟘𝟙𝟘. concurrency, threads, locks


<br>

### parallel pipelines

<br>

* examples of creating parallel pipelines (for instance, when using [ZeroMQ](https://zguide.zeromq.org/#Divide-and-Conquer)), where there is a data source and a data sink, with data being processed by two worker threads in parallel.

<br>

### mutex

<br>

* in terms of mutex, we see an example where we can declare global state using lazy_static, which creates a globally available static ref that requires a Mutex.

<br>

### locks

* since `println!` makes your program slow (as we discussed in the first module), it helps to acquire a lock on `stdout` or `stdeer` and use `writeln!` to print directly, preventing the system from locking and unlocking `stdout` over and over:

```rust
use std::io::{self, Write};

let stdout = io::stdout(); // get the global stdout entity
let mut handle = stdout.lock(); // acquire a lock on it
writeln!(handle, "foo: {}", 1337); 
```

<br>

---

### external resources

<br>

* **[rayon crate for data-parallelism](https://docs.rs/rayon/latest/rayon/)**
* **[crossbean crate for concurrency](https://docs.rs/crossbeam/latest/crossbeam/)**
* **[lazy_static crate for declaring lazily evaluated statics](https://docs.rs/lazy_static/latest/lazy_static/)**
