## concurrency, threads, locks

<br>

### tl; dr

<br>

* examples of creating parallel pipelines (for instance, when using [ZeroMQ](https://zguide.zeromq.org/#Divide-and-Conquer)), where there is a data source and a data sink, with data being processed by two worker threads in parallel.
* in terms of mutex, we see an example where we can declare global state using lazy_static, which creates a globally available static ref that requires a Mutex.

<br>

---

### external resources

<br>

* **[rayon crate for data-parallelism](https://docs.rs/rayon/latest/rayon/)**
* **[crossbean crate for concurrency](https://docs.rs/crossbeam/latest/crossbeam/)**
* **[lazy_static crate for declaring lazily evaluated statics](https://docs.rs/lazy_static/latest/lazy_static/)**
