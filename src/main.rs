#![feature(proc_macro_hygiene, decl_macro)]

use rocket::*;
use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::Serialize;

mod Person;
use crate::Person::Person as OtherPerson;
use crate::Person::People;
use crate::Person::StatusMessage;

mod Task;
use crate::Task::Task as OtherTask;
use crate::Task::Chore as OtherChore;
use crate::Task::Homework as OtherHomework;
use crate::Task::GenericTask as OtherGenericTask;
use crate::Task::Tasks;
use crate::Task::GenericTasks;


enum Any_Task {OtherTask, OtherChore, OtherHomework}

static mut people_id: i64 = -1;
static mut tasks_id: i64 = -1;

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
        Ok(jsonPeople) => {  Ok(Json((jsonPeople.0.people)[0].clone()))  }  ,
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

#[get("/people/<id>/tasks")]
fn get_tasks_of_person(id: i64) -> Result<Json<OtherGenericTask>, String> {
    let task = Task::fetch_task_by_person_generic(id);
    match task{
        Ok(jsonTask) => { Ok(Json((jsonTask.0.generic_tasks)[0].clone()))   },
        Err(_) => Err("Failed to create task".into())
    }
}

#[get("/tasks/<id>")]
fn get_task(id: i64) -> Result<Json<OtherTask>, String> {
    let task = Task::fetch_task_by_id(id);
    match task{
        Ok(jsonTask) => { Ok(Json((jsonTask.0.tasks)[0].clone()))   },
        Err(_) => Err("Failed to create task".into())
    }
}

#[get("/tasks/<id>/status")]
fn get_task_status(id: i64) -> Result<Json<[String;1]>, String> {
    Task::fetch_status(id)
}

#[get("/tasks/<id>/owner")]
fn get_task_ownerId(id: i64) -> Result<Json<[String;1]>, String> {
    Task::fetch_ownerId(id)
}


#[post("/people", format = "json", data = "<person>")]  
fn add_person(person:Json<OtherPerson>)  -> Result<Json<StatusMessage>, String> {
    unsafe{
        people_id+=1;
        let string_id = people_id.to_string();
        Person::add_person(person, string_id) 
    }
    
}

#[post("/people/<owner_id>/tasks", format = "json", data = "<task>")]
fn add_task(owner_id: i64, task: Json<Vec<String>>) -> Result<Json<StatusMessage>, String> {
    unsafe{
        tasks_id+=1;
        let string_id = tasks_id.to_string();
        if task.0.len() == 2 {
            Task::add_task_to_person(owner_id, task, string_id)
        }
        else if task.len() == 4 {
            Task::add_chore_to_person(owner_id, task, string_id)
        }
        else /* task.len() == 5 OR invalid */ {
            Task::add_homework_to_person(owner_id, task, string_id)
        }
    }

}

#[put("/tasks/<id>/status", format = "json", data = "<status>")]
fn put_status(id: i64, status: Json<[String;1]>) -> Result<Json<StatusMessage>, String> {
    Task::put_status(id, status)
}

#[put("/tasks/<id>/owner", format = "json", data = "<ownerId>")]
fn put_ownerId(id: i64, ownerId: Json<[String;1]>) -> Result<Json<StatusMessage>, String> {
    Task::put_ownerId(id, ownerId)
}


#[patch("/people/<id>", format ="json", data= "<person>")] 
fn change_person(id:i64 ,person:Json<[String;3]>)  -> Result<Json<StatusMessage>, String> {
    Person::change_person(id,person) 
}

#[patch("/tasks/<id>", format ="json", data= "<task>")]
fn change_task(id:i64, task:Json<[String;2]>)  -> Result<Json<StatusMessage>, String> {
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


fn main() {
    {
        let db_connection = Connection::open("data.sqlite").unwrap();

        db_connection
            .execute(
                "create table if not exists people (
                    id TEXT PRIMARY KEY,
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
                    id TEXT PRIMARY KEY,
                    ownerId TEXT,
                    type TEXT NOT NULL,
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


    rocket::ignite()
        .mount(
            "/api",
            routes![index, fetch_all_people, add_person, remove_person, fetch_person, change_person, get_tasks_of_person,
                        get_task, add_task, change_task, remove_task, get_task_status, get_task_ownerId, put_status, put_ownerId]
        )
        .launch();
}



