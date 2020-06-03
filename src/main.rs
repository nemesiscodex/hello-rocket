#![feature(decl_macro)]

#[macro_use]
extern crate diesel;

mod schema;

use crate::schema::todo;
use rocket::{self, get, post, put, routes};
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel::PgConnection};
use diesel::{Queryable, Insertable};
use diesel::prelude::*;
use serde_derive::{Serialize, Deserialize};

#[database("postgres")]
struct DbConn(PgConnection);

#[derive(Queryable, Serialize)]
struct Todo {
    id: i32,
    title: String,
    checked: bool
}

#[derive(Insertable, Deserialize)]
#[table_name="todo"]
struct NewTodo {
    title: String
}

#[get("/")]
fn get_todos(conn: DbConn) -> Json<Vec<Todo>> {
    let todos = todo::table
        .order(todo::columns::id.desc())
        .load::<Todo>(&*conn)
        .unwrap();
    Json(todos)
}

#[post("/", data = "<new_todo>")]
fn create_todo(conn: DbConn, new_todo: Json<NewTodo>) -> Json<Todo> {
    let result = diesel::insert_into(todo::table)
        .values(&*new_todo)
        .get_result(&*conn)
        .unwrap();
    Json(result)
}

#[put("/<id>")]
fn check_todo(conn: DbConn, id: i32) -> Json<Todo> {
    let target = todo::table
        .filter(todo::columns::id.eq(id));
    let result = diesel::update(target)
        .set(todo::columns::checked.eq(true))
        .get_result(&*conn)
        .unwrap();


    Json(result)
}


#[get("/")]
fn hello() -> &'static str {
    "Hello, World!"
}

#[get("/<name>")]
fn hello_name(name: String) -> String {
    format!("Hello, {}!", name)
}

fn main() {
    rocket::ignite()
        .attach(DbConn::fairing())
        .mount("/hello", routes![hello, hello_name])
        .mount("/todos", routes![get_todos, create_todo, check_todo])
        .launch();
}
