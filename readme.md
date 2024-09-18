# Todo CLI App

A simple and efficient command-line interface (CLI) application for managing your tasks, written in Rust.

## Features

- Add new tasks
- List all tasks
- Mark tasks as done

## Installation
p
### Prerequisites

- Rust (Make sure you have Rust installed)

### Steps

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/todo.git
    cd todo-app
    ```

2. Build the project:
    ```sh
    cargo build --release
    ```

3. Run the executable:
    ```sh
    ./target/release/todo
    ```

## Usage

### Adding a Task

To add a new task:
```sh
todo add "Buy groceries"

```

### Mark a Task done

To mark a task done:
```sh
todo done "Buy groceries"

```
### Show Tasks

To show all tasks:
```sh
todo show

```
