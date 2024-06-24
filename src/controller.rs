use actix_web::{web, HttpResponse, Responder};
use bson::{doc, oid};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::*;
use MongoDB::{options::FindOptions, Client};

// Define a struct to represent a Todo item
#[derive(Deserialize, Serialize)]
pub struct Todo {
    pub content: String,
    pub is_done: bool,
}

// Define a struct to represent a standard response message
#[derive(Serialize)]
struct Response {
    message: String,
}

// Constants for the MongoDB database and collection names
const MONGO_DB: &'static str = "crudapidb";
const MONGOCOLLECTION: &'static str = "todo";

// Function to create a new todo item
pub async fn create_todo(data: web::Data<Mutex<Client>>, todo: web::Json<Todo>) -> impl Responder {
    // Get the collection
    let todos_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGOCOLLECTION);

    // Insert a new todo item into the collection
    match todos_collection
        .insert_one(
            doc! {"content": &todo.content, "is_done": &todo.is_done},
            None,
        )
        .await
    {
        Ok(db_result) => {
            if let Some(new_id) = db_result.inserted_id.as_object_id() {
                println!("New document inserted with id {}", new_id);
            }
            // Return a success response
            let response = Response {
                message: "Successful".to_string(),
            };
            return HttpResponse::Created().json(response);
        }
        Err(err) => {
            println!("Failed! {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}

// Function to retrieve all todo items
pub async fn get_todos(data: web::Data<Mutex<Client>>) -> impl Responder {
    // Get the collection
    let todos_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGOCOLLECTION);

    let filter = doc! {};
    let find_options = FindOptions::builder().sort(doc! { "_id": -1}).build();
    let mut cursor = todos_collection.find(filter, find_options).await.unwrap();
    let mut results = Vec::new();

    // Iterate over the cursor and collect results
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                results.push(document);
            }
            _ => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }
    HttpResponse::Ok().json(results)
}

// Function to retrieve a single todo item by ID
pub async fn fetch_one(
    data: web::Data<Mutex<Client>>,
    todo_id: web::Path<String>,
) -> impl Responder {
    // Get the collection
    let todos_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGOCOLLECTION);

    let filter = doc! {"_id": oid::ObjectId::with_string(&todo_id.to_string()).unwrap() };
    let obj = todos_collection.find_one(filter, None).await.unwrap();
    return HttpResponse::Ok().json(obj);
}

// Function to update a todo item by ID
pub async fn update_todo(
    data: web::Data<Mutex<Client>>,
    todo_id: web::Path<String>,
    todo: web::Json<Todo>,
) -> impl Responder {
    // Get the collection
    let todos_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGOCOLLECTION);

    let filter = doc! {"_id": oid::ObjectId::with_string(&todo_id.to_string()).unwrap() };
    let data = doc! { "$set": { "content": &todo.content, "is_done": &todo.is_done } };
    todos_collection
        .update_one(filter, data, None)
        .await
        .unwrap();

    let response = Response {
        message: "Updated Successfully".to_string(),
    };
    return HttpResponse::Ok().json(response);
}

// Function to delete a todo item by ID
pub async fn delete_todo(
    data: web::Data<Mutex<Client>>,
    todo_id: web::Path<String>,
) -> impl Responder {
    // Get the collection
    let todos_collection = data
        .lock()
        .unwrap()
        .database(MONGO_DB)
        .collection(MONGOCOLLECTION);

    let filter = doc! {"_id": oid::ObjectId::with_string(&todo_id.to_string()).unwrap() };

    todos_collection.delete_one(filter, None).await.unwrap();
    return HttpResponse::NoContent();
}
