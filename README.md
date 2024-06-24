# Rust MongoDB TodoList API
A Simple Todolist API in Rust and Actix Web! This API allows you to manage a todo list, including creating, reading, updating, and deleting tasks. It is built using Rust, Actix-web, and MongoDB.

## Features

- Create a new todo item
- Retrieve all todo items
- Retrieve a single todo item by ID
- Update a todo item by ID
- Delete a todo item by ID

## Requirements

- Rust (latest stable version)
- MongoDB (running instance)
- [Cargo](https://doc.rust-lang.org/cargo/) (Rust package manager)

## Getting Started

### Clone the repository

```sh
git clone https://github.com/yourusername/rust-mongodb-todolist-api.git
cd rust-mongodb-todolist-api
```

### Setup MongoDB

Ensure you have MongoDB installed and running. You can find installation instructions [here](https://docs.mongodb.com/manual/installation/).

### Configuration

Create a `.env` file in the project root directory and add your MongoDB connection string:

```env
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=todolist
```

### Build and Run

1. Install Rust dependencies:

    ```sh
    cargo build
    ```

2. Run the API:

    ```sh
    cargo run
    ```

The API will be available at `http://localhost:8000`.

## API Endpoints

### Create a new todo item

- **URL:** `/todos`
- **Method:** `POST`
- **Request Body:**
  ```json
  {
    "content": "Your todo content",
    "is_done": false
  }
  ```
- **Response:**
  ```json
  {
    "message": "Successful"
  }
  ```

### Retrieve all todo items

- **URL:** `/todos`
- **Method:** `GET`
- **Response:**
  ```json
  [
    {
      "content": "Your todo content",
      "is_done": false
    },
    ...
  ]
  ```

### Retrieve a single todo item by ID

- **URL:** `/todos/{id}`
- **Method:** `GET`
- **Response:**
  ```json
  {
    "content": "Your todo content",
    "is_done": false
  }
  ```

### Update a todo item by ID

- **URL:** `/todos/{id}`
- **Method:** `PUT`
- **Request Body:**
  ```json
  {
    "content": "Updated content",
    "is_done": true
  }
  ```
- **Response:**
  ```json
  {
    "message": "Updated Successfully"
  }
  ```

### Delete a todo item by ID

- **URL:** `/todos/{id}`
- **Method:** `DELETE`
- **Response:**
  ```json
  {
    "message": "Todo item deleted successfully"
  }
  ```
