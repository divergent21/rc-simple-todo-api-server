use crate::{
    model::{AppState, Todo, QueryOptions, UpdateTodoSchema},
    response::Response
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};

pub fn config (conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(todo_list_handler)
        .service(create_todo_handler)
        .service(get_todo_handler)
        .service(edit_todo_handler)
        .service(delete_todo_handler);

    conf.service(scope);
}

#[get("/todos")]
async fn todo_list_handler (params: web::Query<QueryOptions>, data: web::Data<AppState>) -> impl Responder {
    let todo_list = data.todo_list.lock().unwrap();

    let limit = params.limit.unwrap_or(15);
    let offset = (params.page.unwrap_or(1) - 1) * limit;

    let result: Vec<Todo> = todo_list.clone().into_iter().skip(offset).take(limit).collect();

    HttpResponse::Ok().json(Response::list(result))
}

#[post("/todos")]
async fn create_todo_handler (
    body: web::Json<Todo>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.todo_list.lock().unwrap();

    // check the title is unique
    let todo = vec.iter().find(|todo| todo.get_title() == body.get_title());

    if todo.is_some() {
        return HttpResponse::Conflict().json(Response::bad(&format!("Todo with title: '{}' already exists", body.get_title())));
    }

    let todo = body.to_owned();

    vec.push(body.into_inner());

    HttpResponse::Ok().json(Response::good(todo))
}

#[get("/todos/{id}")]
async fn get_todo_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let vec = data.todo_list.lock().unwrap();

    let id = path.into_inner();
    let todo = vec.iter().find(|todo| todo.get_id() == id);

    if todo.is_none() {
        return HttpResponse::NotFound().json(Response::bad(&format!("Todo with ID: {} not found", id)));
    }

    let todo = todo.unwrap();

    HttpResponse::Ok().json(Response::good(todo.clone()))
}

#[patch("/todos/{id}")]
async fn edit_todo_handler(
    path: web::Path<String>,
    body: web::Json<UpdateTodoSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let mut vec = data.todo_list.lock().unwrap();

    let id = path.into_inner();
    let todo = vec.iter_mut().find(|todo| todo.get_id() == id);

    if todo.is_none() {
        return HttpResponse::NotFound().json(Response::bad(&format!("Todo with ID: {} not found", id)));
    }

    let todo = todo.unwrap();

    if let Some(title) = body.title.to_owned() {
        todo.update_title(title);
    }

    if let Some(content) = body.content.to_owned() {
        todo.update_content(content);
    }

    match body.completed {
        Some(true) => todo.complete(),
        Some(false) => todo.uncomplete(),
        _ => ()
    }

    HttpResponse::Ok().json(Response::good(todo.clone()))
}

#[delete("/todos/{id}")]
async fn delete_todo_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut vec = data.todo_list.lock().unwrap();

    let id = path.into_inner();
    let todo = vec.iter_mut().find(|todo| todo.get_id() == id);

    if todo.is_none() {
        return HttpResponse::NotFound().json(Response::bad(&format!("Todo with ID: {} not found", id)));
    }

    vec.retain(|todo| todo.get_id() != id);

    HttpResponse::NoContent().finish()
}