# Thread-Pool Based Task Scheduler

This project implements a simple thread-pool based task scheduler in Rust, utilizing the `Tokio` and `Crossbeam` libraries to execute both synchronous and asynchronous tasks concurrently.

## Features

- **Synchronous Task Execution**: Execute tasks that do not require asynchronous behavior.
- **Asynchronous Task Execution**: Utilize the `Tokio` runtime to manage and execute asynchronous tasks.
- **Dynamic Worker Management**: Create a thread pool that dynamically manages worker threads to execute submitted tasks.
- **Graceful Shutdown**: Ensure all tasks are completed before shutting down the worker threads.

## Modules Overview

### 1. `scheduler`

- **Purpose**: Manages the thread pool and worker threads that execute tasks.
- **Key Files**:
  - **mod.rs**: Entry point for the scheduler module, containing public definitions and re-exports.
  - **pool.rs**: Implements the `ThreadPool` struct that manages a collection of worker threads and task scheduling.
  - **worker.rs**: Contains the implementation of the worker threads, which continuously wait for tasks to execute.

### 2. `tasks`

- **Purpose**: Defines the types of tasks that can be scheduled, both synchronous and asynchronous.
- **Key Files**:
  - **mod.rs**: Entry point for the tasks module, re-exporting task types.
  - **task.rs**: Implements the `Task` struct, which represents a synchronous task that can be executed.
  - **async_task.rs**: Implements the `AsyncTask` struct, which represents an asynchronous task that can be awaited.

## Usage

```
cargo run
```

## Dependencies

- [crossbeam](https://docs.rs/crossbeam/latest/crossbeam/): Offers tools for concurrent programming in Rust, including channels for communication between threads and scoped thread management. In this project, `Crossbeam` is used to implement a thread-safe task scheduler, allowing for the safe sending and receiving of tasks between the main thread and worker threads.
- [tokio](https://docs.rs/tokio/latest/tokio/): Provides an asynchronous runtime for Rust, allowing for efficient execution of asynchronous tasks. `Tokio` is used to create and manage the runtime environment for executing `AsyncTask` instances, enabling seamless handling of futures.

