Rust MongoDB TodoList API
Welcome to the Rust MongoDB TodoList API! This API allows you to manage a todo list, including creating, reading, updating, and deleting tasks. It is built using Rust, Actix-web, and MongoDB.

Features
Create a new todo item
Retrieve all todo items
Retrieve a single todo item by ID
Update a todo item by ID
Delete a todo item by ID
Requirements
Rust (latest stable version)
MongoDB (running instance)
Cargo (Rust package manager)
Getting Started
Clone the repository
sh
Copy code
git clone https://github.com/yourusername/rust-mongodb-todolist-api.git
cd rust-mongodb-todolist-api
Setup MongoDB
Ensure you have MongoDB installed and running. You can find installation instructions here.

Configuration
Create a .env file in the project root directory and add your MongoDB connection string:

env
Copy code
MONGODB_URI=mongodb://localhost:27017
DATABASE_NAME=todolist
Build and Run
Install Rust dependencies:

sh
Copy code
cargo build
Run the API:

sh
Copy code
cargo run
The API will be available at http://localhost:8000.

API Endpoints
Create a new todo item
URL: /todos
Method: POST
Request Body:
json
Copy code
{
  "content": "Your todo content",
  "is_done": false
}
Response:
json
Copy code
{
  "message": "Successful"
}
Retrieve all todo items
URL: /todos
Method: GET
Response:
json
Copy code
[
  {
    "content": "Your todo content",
    "is_done": false
  },
  ...
]
Retrieve a single todo item by ID
URL: /todos/{id}
Method: GET
Response:
json
Copy code
{
  "content": "Your todo content",
  "is_done": false
}
Update a todo item by ID
URL: /todos/{id}
Method: PUT
Request Body:
json
Copy code
{
  "content": "Updated content",
  "is_done": true
}
Response:
json
Copy code
{
  "message": "Updated Successfully"
}
Delete a todo item by ID
URL: /todos/{id}
Method: DELETE
Response:
json
Copy code
{
  "message": "Todo item deleted successfully"
}
