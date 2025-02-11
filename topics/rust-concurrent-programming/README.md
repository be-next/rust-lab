# Rust Concurrent Programming

## 1. Introduction to Concurrency and Parallelism

### 1.1. Definitions and issues

- **Concurrency**: This is the ability of a program to manage several tasks moving forward ‘simultaneously’, for example by sharing one or more threads. It involves managing concurrent access to shared resources (e.g. memory).
- **Parallelism**: This is the simultaneous execution of several tasks on different processors or cores. This is a form of concurrency that speeds up execution by taking advantage of available hardware.

### 1.2. Classic problems

- **Data Races**: Unsynchronised concurrent read and write accesses to data, leading to indeterminate behaviour.
- **Deadlocks**: Situations where two (or more) threads wait indefinitely for resources held by the other, blocking execution.
- **Race conditions**: The results depend on the execution order of the threads, which is often difficult to reproduce and debug.

## 2. Rust's philosophy for Concurrent Programming

Rust stands out for its guaranteed memory security at compile time, thanks to a strict type system and an ownership and borrowing model. Some key points:

- Static security: Rust prevents many classic concurrency-related errors (e.g. data races) at compile-time.
- Atomic types and synchronisation: the standard library provides tools such as `Mutex`, `RwLock`, `Arc` (atomic reference counter) and channels (`mpsc`) that enable data to be shared in complete security.
- Emphasis on immutability: By default, Rust favours the use of immutable data, reducing the risk of dangerous concurrent access.
