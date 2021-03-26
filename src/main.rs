use actix_files::{NamedFile, self as fs};
use actix_web::*;
use serde_derive::*;
use std::sync::Mutex;

#[derive(Serialize, Debug)]
struct State {
    todo_items: Mutex<Vec<String>>,
}

#[derive(Deserialize)]
struct Item {
    item: String
}

#[get("/api/todo")]
async fn get_data(data: web::Data<State>) -> HttpResponse {
    HttpResponse::Ok()
        .json(data.todo_items.lock().unwrap().clone())
}

#[get("/")]
async fn page() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./index.html")?)
}

#[post("/api/new")]
async fn save_item(data: web::Data<State>, item: web::Json<Item>) -> HttpResponse {
    let mut new_state = data.todo_items.lock().unwrap();
    new_state.push(item.item.to_string());

    HttpResponse::Ok()
        .json(new_state.clone())
}

#[get("/api/clear")]
async fn clear_items(data: web::Data<State>) -> HttpResponse {
    let mut new_state = data.todo_items.lock().unwrap();

    while new_state.len() > 0 {
        new_state.pop();
    }

    HttpResponse::Ok()
        .json(new_state.clone())
}

#[post("/api/delete")]
async fn delete_item(data: web::Data<State>, item: web::Json<Item>) -> HttpResponse {
    let mut new_state = data.todo_items.lock().unwrap();
    let item_to_delete = item.item.to_string();
    let mut counter = 0;

    loop {
        if new_state[counter] == item_to_delete {
            new_state.remove(counter);
            break;
        }
        counter += 1;
    }

    HttpResponse::Ok()
        .json(new_state.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(State {
                todo_items: Mutex::new(vec!["This".to_string(), "Is".to_string(), "Working!".to_string()]),
            })
            .service(get_data)
            .service(page)
            .service(save_item)
            .service(clear_items)
            .service(delete_item)
            .service(fs::Files::new("/static", "./pkg").show_files_listing())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}