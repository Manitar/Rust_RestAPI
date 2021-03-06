#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::Serialize;

mod Person;
use crate::Person::Person as OtherPerson;
use crate::Person::PersonRaw as OtherPersonRaw;
use crate::Person::PersonPatch as OtherPersonPatch;
use crate::Person::People;
use crate::Person::StatusMessage;

mod Task;
use crate::Task::Task as OtherTask;
use crate::Task::TaskRaw as OtherTaskRaw;
use crate::Task::TaskPatch as OtherTaskPatch;
use crate::Task::TaskType;
use crate::Task::Tasks;
use crate::Task::Status;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};



#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/people")] 
fn fetch_all_people() -> Result<Json<People>, String> {
    Person::fetch_all_people()
}


#[get("/people/<id>")] 
fn fetch_person(id: i64) -> Result<Json<OtherPerson>, String> {
    let person = Person::fetch_person(id);
    match person{
        Ok(jsonPeople) => { if(jsonPeople.0.people.len() == 0) { return Err(format!("No such person")); } Ok(Json((jsonPeople.0.people)[0].clone()))   }  ,
        Err(_) => Err("Failed to create person".into())
    }
}

// #[get("/people/<id>/tasks")]
// fn get_tasks_of_person(id: i64) -> Result<Json<OtherTask>, String> {
//     let task = Task::fetch_task_by_person(id);
//     match task{
//         Ok(jsonTask) => { Ok(Json((jsonTask.0.tasks)[0].clone()))   },
//         Err(_) => Err("Failed to create task".into())
//     }
// }



#[get("/people/<id>/tasks?<status>")]
fn get_tasks_of_person_status(id: i64, status: String) -> Result<Json<Tasks>, String> {
    let mut status_upper = "".to_string();
    if(status == "active".to_string()){
        status_upper = "Active".to_string();
    }
    if(status == "done".to_string()){
        status_upper = "Done".to_string();
    }
    Task::fetch_tasks_by_person_status(id, status_upper)
}



#[get("/people/<id>/tasks")]
fn get_tasks_of_person(id: i64) -> Result<Json<Tasks>, String> {
    Task::fetch_tasks_by_person(id)
}


#[get("/tasks/<id>")]
fn get_task(id: i64) -> Result<Json<OtherTask>, String> {
    let task = Task::fetch_task_by_id(id);
    match task{
        Ok(jsonTask) => { if(jsonTask.0.tasks.len() == 0) { return Err(format!("No such task")); } Ok(Json((jsonTask.0.tasks)[0].clone()))   },
        Err(why) => Err(format!("Failed to create task: {why}"))
    }
}

#[get("/tasks/<id>/status")]
fn get_task_status(id: i64) -> Result<Json<String>, String> {
    Task::fetch_status(id)
}

#[get("/tasks/<id>/owner")]
fn get_task_ownerId(id: i64) -> Result<Json<String>, String> {
    Task::fetch_ownerId(id)
}


#[post("/people", format = "json", data = "<person>")]  
fn add_person(person:Json<OtherPersonRaw>)  -> Result<Json<StatusMessage>, String> {

    Person::add_person(person) 
    
}

#[post("/people/<owner_id>/tasks", format = "json", data = "<task>")]
fn add_task(owner_id: i64, task: Json<OtherTaskRaw>) -> Result<Json<StatusMessage>, String> {

    match Task::get_type(&task.0) {
        TaskType::Task => {
            Task::add_task_to_person(owner_id, task)
        }

        TaskType::Chore => {
            Task::add_chore_to_person(owner_id, task)
        }

        TaskType::Homework => {
            Task::add_homework_to_person(owner_id, task)
        }
    }

}

#[put("/tasks/<id>/status", format = "json", data = "<status>")]
fn put_status(id: i64, status: Json<String>) -> Result<Json<StatusMessage>, String> {
    Task::put_status(id, status)
}

#[put("/tasks/<id>/owner", format = "json", data = "<ownerId>")]
fn put_ownerId(id: i64, ownerId: Json<String>) -> Result<Json<StatusMessage>, String> {
    Task::put_ownerId(id, ownerId)
}


#[patch("/people/<id>", format ="json", data= "<person>")] 
fn change_person(id:i64 ,person:Json<OtherPersonPatch>)  -> Result<Json<StatusMessage>, String> {
    Person::change_person(id,person) 
}

#[patch("/tasks/<id>", format ="json", data= "<task>")]
fn change_task(id:i64, task:Json<OtherTaskPatch>)  -> Result<Json<StatusMessage>, String> {
    Task::change_task(id, task) 

}


#[delete("/people/<id>")]
fn remove_person(id: i64) -> Result<Json<StatusMessage>, String> {
    Person::remove_person(id)
}

#[delete ("/tasks/<id>")]
fn remove_task(id: i64) -> Result<Json<StatusMessage>, String> {
    Task::remove_task(id)
}

use rocket::http::Method;
use rocket_cors::{AllowedOrigins, CorsOptions};

fn main() {
    {
        let db_connection = Connection::open("data.sqlite").unwrap();

        db_connection
            .execute(
                "create table if not exists people (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT NOT NULL,
                    email TEXT NOT NULL UNIQUE,
                    favoriteProgrammingLanguage TEXT NOT NULL
                );",
                rusqlite::NO_PARAMS,
            )
            .unwrap();

            // Generic table for all 3 data types
            db_connection
            .execute("
                create table if not exists tasks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    ownerId INTEGER,
                    task_type TEXT NOT NULL,
                    status TEXT NOT NULL,
                    description TEXT,
                    size TEXT,
                    course TEXT,
                    dueDate TEXT,
                    details TEXT,
                    FOREIGN KEY(ownerId) REFERENCES people(id)
                );",
                rusqlite::NO_PARAMS,
            )
            .unwrap();
    }



    let cors = CorsOptions::default()
    .allowed_origins(AllowedOrigins::all())
    .allowed_methods(
        vec![Method::Get, Method::Post, Method::Patch, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
    )
    .allow_credentials(true);


    rocket::ignite()
        .mount(
            "/api",
            routes![index, fetch_all_people, add_person, remove_person, fetch_person, change_person, get_tasks_of_person, get_tasks_of_person_status,
                        get_task, add_task, change_task, remove_task, get_task_status, get_task_ownerId, put_status, put_ownerId]
        ).attach(cors.to_cors().unwrap())
        .launch();
}



