
use rocket_contrib::json::Json;
use rusqlite::Connection;
use rusqlite::types::FromSql;
use rusqlite::types::FromSqlError;
use rusqlite::types::FromSqlResult;
use rusqlite::types::ValueRef;
use serde::Serialize;
use serde::Deserialize; 

use crate::Person::StatusMessage;




#[derive(Clone, Serialize, Deserialize)]
pub struct Task{
    id: i64,
    ownerId: i64,
    #[serde(rename = "type")]
    task_type: TaskType,
    status: Option<String>, //Active or Done
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<String>, //Small, Medium or Large
    #[serde(skip_serializing_if = "Option::is_none")]
    course: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dueDate: Option<String>, // Date
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}



#[derive(Clone, Serialize, Deserialize)]
pub struct TaskRaw{
    #[serde(rename = "type")]
    task_type: TaskType,
    status: String, //Active or Done
    description: Option<String>,
    size: Option<String>, //Small, Medium or Large
    course: Option<String>,
    dueDate: Option<String>, // Date
    details: Option<String>,
}

#[derive(Serialize)]
pub struct Tasks{
    pub tasks: Vec<Task>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TaskPatch{
    #[serde(rename = "type")]
    task_type: Option<TaskType>,
    status: Option<String>, //Active or Done
    description: Option<String>,
    size: Option<String>, //Small, Medium or Large
    course: Option<String>,
    dueDate: Option<String>, // Date
    details: Option<String>,
}


#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum TaskType {
    Task,
    Chore,
    Homework,
}

impl FromSql for TaskType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let bytes = match value {
            ValueRef::Text(bytes) => bytes,
            other => return Err(FromSqlError::InvalidType),
        };
        
        let task_type = match bytes {
            b"Task" => TaskType::Task,
            b"Chore" => TaskType::Chore,
            b"Homework" => TaskType::Homework,
            other => return Err(FromSqlError::InvalidType),
        };

        Ok(task_type)
    }
}

pub fn get_type(task: &TaskRaw) -> TaskType {
    return task.task_type;
}



// pub fn fetch_task_by_id_2(id: i64) -> Option<Json<Task>> {
//     //connect to sqllite
//     let db_connection = match Connection::open("data.sqlite") {
//         Ok(connection) => connection, //returns connection if success
//         Err(_) => {return None;}
//     };


//     let mut statement = match db_connection.prepare(&format!("select * from tasks where id = {};",[&id][0])) { 
//         Ok(statement) => statement,
//         Err(_) => {return None;}//else prints error
//     };

    
//     let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
//         Ok(Task {
//             id: row.get(0)?,
//             ownerId: row.get(1)?,
//             task_type: row.get(2)?,
//             status: row.get(3)?,
//             description: row.get(4)?,
//             size: row.get(5)?,
//             course: row.get(6)?,
//             dueDate: row.get(7)?,
//             details: row.get(8)?,
//              })
//         });
    
        
    
//         match results {
//             Ok(rows) =>{ 
//                 let collection: rusqlite::Result<Vec<Task>> = rows.collect();
    
//                 match collection {
//                     Ok(tasks) => { if(tasks.len() == 0) { return None; } Some(Json(tasks[0]))},
//                     Err(why) => None
//                 }
//             }
                
//             Err(err) => {return None;}
//         }
// }

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

    
    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
        Ok(Task {
            id: row.get(0)?,
            ownerId: row.get(1)?,
            task_type: row.get(2)?,
            status: row.get(3)?,
            description: row.get(4)?,
            size: row.get(5)?,
            course: row.get(6)?,
            dueDate: row.get(7)?,
            details: row.get(8)?,
             })
        });
    
        
    
        match results {
            Ok(rows) =>{ 
                let collection: rusqlite::Result<Vec<Task>> = rows.collect();
    
                match collection {
                    Ok(tasks) => { Ok(Json(Tasks{tasks}))},
                    Err(why) => Err(format!("Could not collect tasks: {why}"))
                }
            }
                
            Err(err) => Err(format!("{:?}", err))
        }
}
pub fn fetch_tasks_by_person(id: i64) -> Result<Json<Tasks>, String> {
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


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| {
    Ok(Task {
        id: row.get(0)?,
        ownerId: row.get(1)?,
        task_type: row.get(2)?,
        status: row.get(3)?,
        description: row.get(4)?,
        size: row.get(5)?,
        course: row.get(6)?,
        dueDate: row.get(7)?,
        details: row.get(8)?,
         })
    });

    

    match results {
        Ok(rows) =>{ 
            let collection: rusqlite::Result<Vec<Task>> = rows.collect();

            match collection {
                Ok(tasks) => { Ok(Json(Tasks{tasks}))},
                Err(why) => Err(format!("Could not collect tasks: {why}"))
            }
        }
            
        Err(err) => Err(format!("{:?}", err))
    }
}

pub fn fetch_status(id: i64) -> Result<Json<String>, String> {

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
        task_type: row.get(2)?,
        status: row.get(3)?,
        description: row.get(4)?,
        size: row.get(5)?,
        course: row.get(6)?,
        dueDate: row.get(7)?,
        details: row.get(8)?,
         })
    });

    match results {
        Ok(rows) =>{ 
            let collection: rusqlite::Result<Vec<Task>> = rows.collect();

            match collection {
                Ok(tasks) => { 
                    let task = &tasks[0];
                    let status_opt = &task.status;
                    let status = 
                    match status_opt {
                        Some(t) => t,
                        None => return Err("Incorrect data type sent".into()),
                    };

                    Ok(Json(status.to_string())) }
                Err(_) => Err("Could not collect tasks".into()),
            }
        }
            
        Err(err) => Err(format!("{:?}", err))
    }
}

pub fn fetch_ownerId(id: i64) -> Result<Json<String>, String> {
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
        task_type: row.get(2)?,
        status: row.get(3)?,
        description: row.get(4)?,
        size: row.get(5)?,
        course: row.get(6)?,
        dueDate: row.get(7)?,
        details: row.get(8)?,
         })
    });

    match results {
        Ok(rows) =>{ 
            let collection: rusqlite::Result<Vec<Task>> = rows.collect();

            match collection {
                Ok(tasks) => { 
                    let task = &tasks[0];
                    let owner_id = &task.ownerId.to_string();
                    Ok(Json(owner_id.to_string())) }
                Err(_) => Err("Could not collect tasks".into()),
            }
        }
            
        Err(err) => Err(format!("{:?}", err))
    }
}

pub fn put_status(id: i64, status: Json<String>) -> Result<Json<StatusMessage>, String>{
        //connection
        let db_connection = match Connection::open("data.sqlite") {
            Ok(connection) => connection,
            Err(_) => {
                return Err(String::from("Failed to connect to database"));
            }
        };
        let check1 = status.0;
        if !check1.eq("") {
            let mut statement =
            match db_connection.prepare("UPDATE tasks SET status = (?1) WHERE id = (?2); ") {
                Ok(statement) => statement,
                Err(_) => return Err("Failed to prepare query".into()),
            }; 
    
            let results = statement.execute([check1.to_string(), id.to_string()]);
    
            match results {
                Ok(rows_affected) => Ok(Json(StatusMessage {
                    message: format!("{} rows updated!", rows_affected),
                })),
                Err(err) => Err(format!("{:?}", err))   
            };
        }
        Ok(Json(StatusMessage { message: "finished!".to_string()}))
    }

pub fn put_ownerId(id: i64, ownerId: Json<String>) -> Result<Json<StatusMessage>, String>{
    //connection
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };
    let check1 = ownerId.0;
    if !check1.eq("") {
        let mut statement =
        match db_connection.prepare("UPDATE tasks SET ownerId = (?1) WHERE id = (?2); ") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 

        let results = statement.execute([check1.to_string(), id.to_string()]);

        match results {
            Ok(rows_affected) => Ok(Json(StatusMessage {
                message: format!("{} rows updated!", rows_affected),
            })),
            Err(err) => Err(format!("{:?}", err))   
        };
    }
    Ok(Json(StatusMessage { message: "finished!".to_string()}))
}

pub fn add_task_to_person(owner_id: i64, task:Json<TaskRaw>) -> Result<Json<StatusMessage>, String> {
    
    //connection
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement =
        match db_connection.prepare("insert into tasks (ownerId, task_type, status) 
        values (?1, ?2, ?3);") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 

    let task_type = "Task".to_string();
    let status = &task.status;

    // let description = NULL;
    // let size = NULL;
    // let course = NULL;
    // let dueDate = NULL;
    // let details = NULL;
    

    let results = statement.execute([owner_id.to_string(), task_type.to_string(), status.to_string()]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_affected),
        })),
        Err(err) => Err(format!("{:?}", err))   
    }

}

pub fn add_chore_to_person(owner_id: i64, task:Json<TaskRaw>) -> Result<Json<StatusMessage>, String> {
    
    //connection
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement =
        match db_connection.prepare("insert into tasks (ownerId, task_type, status, description, size) 
        values (?1, ?2, ?3, ?4, ?5);") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 
        
    let task_type = "Chore".to_string();
    let status = &task.status;
    
    let description =
        match &task.description {
            Some(t) => t,
            None => return Err("Incorrect data type sent".into()),
        };
    
    let size =
        match &task.size {
            Some(t) => t,
            None => return Err("Incorrect data type sent".into()),
        };

    // let course = NULL;
    // let dueDate = NULL;
    // let details = NULL;
    
    let results = statement.execute([owner_id.to_string(), task_type.to_string(),
    status.to_string(), description.to_string(), size.to_string()]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_affected),
        })),
        Err(err) => Err(format!("{:?}", err))   
    }

}

pub fn add_homework_to_person(owner_id: i64, task:Json<TaskRaw>) -> Result<Json<StatusMessage>, String> {
    //connection
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let mut statement =
        match db_connection.prepare("insert into tasks (ownerId, task_type, status, course, dueDate, details)
         values (?1, ?2, ?3, ?4, ?5, ?6);") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 
        
    let task_type = "Homework".to_string();
    let status = &task.status;

    // let description = NULL;
    // let size = NULL;
    
    let course =
        match &task.course {
            Some(t) => t,
            None => return Err("Incorrect data type sent".into()),
        };

    let dueDate =
        match &task.dueDate {
            Some(t) => t,
            None => return Err("Incorrect data type sent".into()),
        };

    let details =
        match &task.details {
            Some(t) => t,
            None => return Err("Incorrect data type sent".into()),
        };
    
    let results = statement.execute([owner_id.to_string(), task_type.to_string(), status.to_string(),
    course.to_string(), dueDate.to_string(), details.to_string()]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_affected),
        })),
        Err(err) => Err(format!("{:?}", err))   
    }
}


pub fn change_task(id:i64, task:Json<TaskPatch>)  -> Result<Json<StatusMessage>, String> {
    //connection

    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let change_task = task.0; 

    // task_type: Option<TaskType>,
    // status: Option<String>, //Active or Done
    // description: Option<String>,
    // size: Option<String>, //Small, Medium or Large
    // course: Option<String>,
    // dueDate: Option<String>, // Date
    // details: Option<String>,

    let check_task_type = change_task.task_type;
    let check_status = change_task.status;

    let check_description = change_task.description;
    let check_size = change_task.size;

    let check_course = change_task.course;
    let check_dueDate = change_task.dueDate;
    let check_details = change_task.details;

    let check_task_type_flag = false;
    let check_status_flag = false;

    let check_description_flag = false;
    let check_size_flag = false;

    let check_course_flag = false;
    let check_dueDate_flag = false;
    let check_details_flag = false;


    // let task_type =
    // match check_task_type {
    //     Some(t) => {check_task_type_flag = true; 
    //         match t {
    //         TaskType::Task => "Task".to_string(),
    //         TaskType::Chore => "Chore".to_string(),
    //         TaskType::Homework => "Homework".to_string(),}
    //     },
    //     None => "".to_string(),
    // };

    // let status =
    // match check_status {
    //     Some(t) => {check_status_flag = true; t},
    //     None => "".to_string(),
    // };

    // let description =
    // match check_description {
    //     Some(t) => {check_description_flag = true; t},
    //     None => "".to_string()
    // };

    // let size =
    // match check_size {
    //     Some(t) => {check_size_flag = true; t},
    //     None => "".to_string()
    // };

    // let course =
    // match check_course {
    //     Some(t) => {check_course_flag = true; t},
    //     None => "".to_string()
    // };

    // let dueDate =
    // match check_dueDate {
    //     Some(t) => {check_dueDate_flag = true; t},
    //     None => "".to_string()
    // };

    // let details =
    // match check_details {
    //     Some(t) => {check_details_flag = true; t},
    //     None => "".to_string()
    // };

    Ok(Json(StatusMessage { message: "finished!".to_string()}))
}

// pub fn patch_task(id:i64, task:Json<TaskPatch>) -> Option<Json<Task>> {

//     let db_connection = match Connection::open("data.sqlite") {
//         Ok(connection) => connection,
//         Err(_) => {
//             return None;
//         }
//     };

//     let existing_task = fetch_task_by_id_2(id)?.0; //Unwrap JSON

//     return Some(Json(Task{
//         id: id,
//         ownerId: existing_task.ownerId,
//         task_type: task.task_type,
//         status: task.status.or(existing_task.status),
//         description: task.description,
//         size: task.size, //Small, Medium or Large
//         course: task.course,
//         dueDate: task.dueDate, // Date
//         details: task.details,
//     }))
    


// }

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