# Smart Pointers in Rust

Smart pointers are data structures that behave like pointers but also include additional metadata and capabilities. They are primarily used to:

- manage memory,
- enforce ownership rules,
- provide additional functionality (such as reference counting or interior mutability),
- and enhance safety guarantees.

In Rust, smart pointers are implemented as structs that typically implement the `Deref` trait (to enable pointer-like behavior) and the `Drop` trait (to manage resource cleanup). Some smart pointers, such as `Rc<T>` and `Arc<T>`, also implement reference counting, while others, like `RefCell<T>` and `Mutex<T>`, provide interior mutability.

Unlike raw pointers (`*const T`, `*mut T`), smart pointers in Rust enforce memory safety at compile-time or runtime, preventing common issues like dangling pointers, double frees, or data races.

## 📚 Ressources

- [Smart Pointers in official Rust documentation](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html): because everything starts by RTFM!

## 🧬 Taxonomy of smart pointers

### 1. Basic Allocation

Smart pointers that allocate data on the heap.

Box<T> → Single ownership of a heap-allocated value.

- ✅ Moves large data off the stack.
- ✅ Stores dynamically sized types (dyn Trait).
- ❌ No shared ownership.

### 2. Reference Counting (Shared Ownership)

Smart pointers that enable multiple owners of the same value.

#### Single-threaded Reference Counting

`Rc<T>` → Non-thread-safe reference counting:

- ✅ Multiple owners within a single thread.
- ✅ Efficient memory sharing.
- ❌ Not safe for multi-threading.
- 🔄 Use with `RefCell<T>` for mutability.

#### Multi-threaded Reference Counting

`Arc<T>` → Atomic reference counting (thread-safe):

- ✅ Allows shared ownership across threads.
- ✅ Safe for concurrent read access.
- ❌ More expensive than `Rc<T>` (atomic operations).
- 🔄 Use with `Mutex<T>` or `RwLock<T>` for mutable shared access.

### 3. Interior Mutability

Smart pointers that allow modifying data even when borrowed immutably.

`RefCell<T>` → Borrow-checked mutability at runtime:

- ✅ Allows mutable borrowing at runtime (single-threaded).
- ✅ Works well with `Rc<T>`.
- ❌ Panics on borrow rule violations.

`Cell<T>` → Copy-based interior mutability:

- ✅ Allows overwriting values without borrowing.
- ✅ Best for Copy types.
- ❌ Cannot return references to internal data.

### 4. Thread Synchronization

Smart pointers that provide safe, mutable access to shared data across threads.

`Mutex<T>` → Exclusive locking for safe mutation:

- ✅ Ensures safe mutable access across threads.
- ❌ Can cause deadlocks.
- 🔄 Combine with `Arc<T>` for shared access.

`RwLock<T>` → Optimized for read-heavy workloads.

- ✅ Allows multiple readers, single writer.
- ✅ More efficient than `Mutex<T>` for read-heavy workloads.
- ❌ Writers may starve if readers dominate.

### 5. Copy-on-Write (COW) Optimization

Smart pointers that defer cloning until mutation is needed.

`Cow<T>` → Copy-on-write optimization:

- ✅ Avoids unnecessary allocations.
- ✅ Useful when working with immutable data that may need modifications.
- ❌ Clone happens only when mutation occurs.

## 📊 Summary Table

| Category                   | Smart Pointer | Thread-Safe? | Mutable? | Use Case |
|----------------------------|--------------|-------------|----------|---------|
| **Heap Allocation**        | `Box<T>`     | 🚫 N/A | ❌ Immutable | Store single value on the heap |
| **Reference Counting**     | `Rc<T>`      | ❌ No  | ❌ Immutable | Shared ownership in single-threaded context |
|                            | `Arc<T>`     | ✅ Yes | ❌ Immutable | Shared ownership across threads |
| **Interior Mutability**    | `RefCell<T>` | ❌ No  | ✅ Yes | Mutability at runtime (single-threaded) |
|                            | `Cell<T>`    | ❌ No  | ✅ Yes | Fast, non-borrowing mutability for `Copy` types |
| **Thread Synchronization** | `Mutex<T>`   | ✅ Yes | ✅ Yes | Safe mutation across threads (exclusive lock) |
|                            | `RwLock<T>`  | ✅ Yes | ✅ Yes | Optimized multi-threaded read access |
| **Copy-on-Write**         | `Cow<T>`     | ✅ Yes | ❌ Immutable (until modified) | Avoids cloning until needed |

## 🔥 Choosing the Right Smart Pointer

- Use `Box<T>` when you need heap allocation with single ownership.
- Use `Vec<T>` if you need a dynamic, growable array.
- Use `Rc<T>` when multiple parts of your program share ownership (single-threaded).
- Use `Arc<T>` when multiple threads share ownership.
- Use `RefCell<T>` if you need mutability inside an immutable struct (single-threaded).
- Use `Mutex<T>` if you need safe mutation across threads.
- Use `RwLock<T>` when read operations dominate write operations.
- Use `Cow<T>` when avoiding unnecessary copies.
