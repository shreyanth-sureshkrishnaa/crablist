# CrabList

A blazing fast CLI to-do list application written in Rust.

## Description

CrabList is a simple, fast command-line todo list manager built with rust that allows you to add tasks, mark them as complete, view your list, and clear all items.

## Features

- Add new to-do items
- Mark items as complete
- Display all to-do items with status
- Clear entire list with confirmation

## Prerequisites

- Rust (1.70.0 or higher recommended)
- Cargo (comes with Rust)

## Installation

1. Clone this repository or download the source code
2. Navigate to the project directory
3. Build the project:

```bash
cargo build --release
```

## Dependencies

This project uses the following crate:

- `owo-colors` - For terminal color output


Run the application:

```bash
cargo run
```

### Menu Options

Once running, you'll see a menu with the following options:

1. **Add a to-do item** - Create a new task
2. **Complete a to-do item** - Mark an existing task as complete by ID
3. **Display to-do items** - View all tasks and their status
4. **Clear full list** - Remove all tasks (with confirmation)
5. **Exit** - Close the application
