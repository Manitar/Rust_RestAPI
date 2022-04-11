
use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::Serialize;

use crate::Person::StatusMessage;



#[derive(Serialize)]
pub struct Tasks{
    pub tasks: Vec<Task>,
}

#[derive(Clone)]
#[derive(Serialize)]
pub struct Task{
    id: String,
    ownerId: String,
    status: String, //Active or Done
}

pub fn fetch_task_by_id(id: i64) -> Result<Json<Tasks>, String> {
    //connect to sqllite
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection, //returns connection if success
        Err(_) => {
            return Err(String::from("Failed to connect to database"));//else prints error
        }
    };


    let mut statement = match db_connection.prepare(&format!("select * from tasks where id = {};",[&id][0])) { 
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),//else prints error
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| { //creat a todo_item's from all the results
    Ok(Task {
        id: row.get(0)?,
        ownerId: row.get(1)?,
        status: row.get(2)?,
         })
    });

    match results {
        Ok(rows) =>{ 
            let collection: rusqlite::Result<Vec<Task>> = rows.collect();

            match collection {
                Ok(tasks) => { Ok(Json(Tasks{tasks}))},
                Err(_) => Err("Could not collect tasks".into()),
            }
        }
            
        Err(err) => Err(format!("{:?}", err))
    }
}

pub fn fetch_task_by_person(id: i64) -> Result<Json<Tasks>, String> {
    //connect to sqllite
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection, //returns connection if success
        Err(_) => {
            return Err(String::from("Failed to connect to database"));//else prints error
        }
    };


    let mut statement = match db_connection.prepare(&format!("select * from tasks where ownerId = {};",[&id][0])) { 
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),//else prints error
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| { //creat a todo_item's from all the results
    Ok(Task {
        id: row.get(0)?,
        ownerId: row.get(1)?,
        status: row.get(2)?,
         })
    });

    match results {
        Ok(rows) =>{ 
            let collection: rusqlite::Result<Vec<Task>> = rows.collect();

            match collection {
                Ok(tasks) => { Ok(Json(Tasks{tasks}))},
                Err(_) => Err("Could not collect tasks".into()),
            }
        }
            
        Err(err) => Err(format!("{:?}", err))
    }
}

pub fn fetch_status(id: i64) -> Result<Json<[String;1]>, String> {

    //connect to sqllite
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection, //returns connection if success
        Err(_) => {
            return Err(String::from("Failed to connect to database"));//else prints error
        }
    };


    let mut statement = match db_connection.prepare(&format!("select * from tasks where id = {};",[&id][0])) { 
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),//else prints error
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| { //creat a todo_item's from all the results
    Ok(Task {
        id: row.get(0)?,
        ownerId: row.get(1)?,
        status: row.get(2)?,
         })
    });

    match results {
        Ok(rows) =>{ 
            let collection: rusqlite::Result<Vec<Task>> = rows.collect();

            match collection {
                Ok(tasks) => { 
                    let task = &tasks[0];
                    let status = &task.status;
                    let arr = [status.to_string()];
                    Ok(Json(arr)) }
                Err(_) => Err("Could not collect tasks".into()),
            }
        }
            
        Err(err) => Err(format!("{:?}", err))
    }
}

pub fn fetch_ownerId(id: i64) -> Result<Json<[String;1]>, String> {
    //connect to sqllite
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection, //returns connection if success
        Err(_) => {
            return Err(String::from("Failed to connect to database"));//else prints error
        }
    };


    let mut statement = match db_connection.prepare(&format!("select * from tasks where id = {};",[&id][0])) { 
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),//else prints error
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| { //creat a todo_item's from all the results
    Ok(Task {
        id: row.get(0)?,
        ownerId: row.get(1)?,
        status: row.get(2)?,
         })
    });

    match results {
        Ok(rows) =>{ 
            let collection: rusqlite::Result<Vec<Task>> = rows.collect();

            match collection {
                Ok(tasks) => { 
                    let task = &tasks[0];
                    let ownerId = &task.ownerId;
                    let arr = [ownerId.to_string()];
                    Ok(Json(arr)) }
                Err(_) => Err("Could not collect tasks".into()),
            }
        }
            
        Err(err) => Err(format!("{:?}", err))
    }
}

pub fn put_status(id: i64, status: Json<[String;1]>) -> Result<Json<StatusMessage>, String>{
        //connection
        let db_connection = match Connection::open("data.sqlite") {
            Ok(connection) => connection,
            Err(_) => {
                return Err(String::from("Failed to connect to database"));
            }
        };
        let checkarray = status;
        let check1 = &checkarray[0];
        if !check1.eq("") {
            let mut statement =
            match db_connection.prepare("UPDATE tasks SET status = (?1) WHERE id = (?2); ") {
                Ok(statement) => statement,
                Err(_) => return Err("Failed to prepare query".into()),
            }; 
    
            let results = statement.execute([check1,&id.to_string()]);
    
            match results {
                Ok(rows_affected) => Ok(Json(StatusMessage {
                    message: format!("{} rows updated!", rows_affected),
                })),
                Err(err) => Err(format!("{:?}", err))   
            };
        }
        Ok(Json(StatusMessage { message: "finished!".to_string()}))
    }

pub fn put_ownerId(id: i64, ownerId: Json<[String;1]>) -> Result<Json<StatusMessage>, String>{
    //connection
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };
    let checkarray = ownerId;
    let check1 = &checkarray[0];
    if !check1.eq("") {
        let mut statement =
        match db_connection.prepare("UPDATE tasks SET ownerId = (?1) WHERE id = (?2); ") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 

        let results = statement.execute([check1,&id.to_string()]);

        match results {
            Ok(rows_affected) => Ok(Json(StatusMessage {
                message: format!("{} rows updated!", rows_affected),
            })),
            Err(err) => Err(format!("{:?}", err))   
        };
    }
    Ok(Json(StatusMessage { message: "finished!".to_string()}))
}

pub fn add_task_to_person(owner_id: i64, task:Json<[String;2]>) -> Result<Json<StatusMessage>, String> {
    
    //connection
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement =
        match db_connection.prepare("insert into tasks (ownerId, id, status) values (?1, ?2, ?3);") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 
        
    let add_task = task;
    let id = &add_task[0];
    let status = &add_task[1];
    let results = statement.execute([owner_id.to_string(), id.to_string(), status.to_string()]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_affected),
        })),
        Err(err) => Err(format!("{:?}", err))   
    }

}

pub fn change_task(id:i64, task:Json<[String;2]>)  -> Result<Json<StatusMessage>, String> {
    //connection
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };
    let changearray = task;
    let check1 = &changearray[0];
    let check2 = &changearray[1];
    if !check1.eq("") {
        let mut statement =
        match db_connection.prepare("UPDATE tasks SET ownerId = (?1) WHERE id = (?2); ") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 

        let results = statement.execute([check1,&id.to_string()]);

        match results {
            Ok(rows_affected) => Ok(Json(StatusMessage {
                message: format!("{} rows updated!", rows_affected),
            })),
            Err(err) => Err(format!("{:?}", err))   
        };
    }
    if !check2.eq("") {
        let mut statement =
        match db_connection.prepare("UPDATE tasks SET status = (?1) WHERE id = (?2); ") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 

        let results = statement.execute([check2,&id.to_string()]);

        match results {
            Ok(rows_affected) => Ok(Json(StatusMessage {
                message: format!("{} rows updated!", rows_affected),
            })),
            Err(err) => Err(format!("{:?}", err))   
        };
    }
    Ok(Json(StatusMessage { message: "finished!".to_string()}))
}


pub fn remove_person(id: i64) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement = match db_connection.prepare("delete from people where id = $1;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };
    let results = statement.execute(&[&id]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows deleted!", rows_affected),
        })),
        Err(_) => Err("Failed to delete person".into()),
    }
}

pub fn remove_task(id: i64) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement = match db_connection.prepare("delete from tasks where id = $1;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };
    let results = statement.execute(&[&id]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows deleted!", rows_affected),
        })),
        Err(_) => Err("Failed to delete person".into()),
    }
}