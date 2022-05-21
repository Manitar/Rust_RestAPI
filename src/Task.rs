
use rocket_contrib::json::Json;
use rusqlite::Connection;
use rusqlite::ToSql;
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
    status: Status, //Active or Done
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<Size>, //Small, Medium or Large
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
    status: Status, //Active or Done
    description: Option<String>,
    size: Option<Size>, //Small, Medium or Large
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
    status: Option<Status>, //Active or Done
    description: Option<String>,
    size: Option<Size>, //Small, Medium or Large
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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Status {
    Active,
    Done,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl FromSql for TaskType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let bytes = match value {
            ValueRef::Text(bytes) => {/*bytes.to_vec().make_ascii_lowercase();*/ bytes},
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

impl FromSql for Status {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let bytes = match value {
            ValueRef::Text(bytes) => {/*bytes.to_vec().make_ascii_lowercase();*/ bytes},
            other => return Err(FromSqlError::InvalidType),
        };
        let status = match bytes {
            b"Active" => Status::Active,
            b"Done" => Status::Done,
            other => return Err(FromSqlError::InvalidType),
        };

        Ok(status)
    }
}

impl FromSql for Size {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let bytes = match value {
            ValueRef::Text(bytes) => {/*bytes.to_vec().make_ascii_lowercase();*/ bytes},
            other => return Err(FromSqlError::InvalidType),
        };
        let size = match bytes {
            b"Small" => Size::Small,
            b"Medium" => Size::Medium,
            b"Large" => Size::Large,
            other => return Err(FromSqlError::InvalidType),
        };

        Ok(size)
    }
}

pub fn get_type(task: &TaskRaw) -> TaskType {
    return task.task_type;
}

pub fn get_status(task: &TaskRaw) -> Status {
    return task.status;
}

pub fn get_size(task: &TaskRaw) -> Option<Size> {
    return task.size;
}

pub fn type_to_string(task_type: TaskType) -> String {

    match task_type{
        TaskType::Task => "Task".to_string(),
        TaskType::Chore => "Chore".to_string(),
        TaskType::Homework => "Homework".to_string()
    }

}

pub fn status_to_string(status: Status) -> String {

    match status{
        Status::Active => "Active".to_string(),
        Status::Done => "Done".to_string()
    }
}

pub fn size_to_string(size: Option<Size>) -> Option<String> {
 
    match size {
        Some(Size::Small) => Some("Small".to_string()),
        Some(Size::Medium) => Some("Medium".to_string()),
        Some(Size::Large) => Some("Large".to_string()),
        _ => None //If it's None
    }

}

pub fn patch_by_case(Json(patch): Json<TaskPatch>, Json(existing_task): Json<Task>) -> Option<Task>{
    //9 cases

    let existing_type = existing_task.task_type.clone();
    let patch_type = patch.task_type.clone();

    //Existing = Task, Patch = Task;
    if(existing_type == TaskType::Task && patch_type == Some(TaskType::Task) || patch_type == None){
        return task_to_task(patch.clone(), existing_task.clone());
    }
    //Existing = Chore, Patch = Chore;
    if(existing_type == TaskType::Chore && patch_type == Some(TaskType::Chore) || patch_type == None ){
        return chore_to_chore(patch.clone(), existing_task.clone());
    }
    //Existing = Homework, Patch = Homework;
    if(existing_type == TaskType::Homework && patch_type == Some(TaskType::Homework) || patch_type == None ){
        return homework_to_homework(patch.clone(), existing_task.clone());
    }
    //Existing = Chore, Patch = Task;
    if(existing_type == TaskType::Chore && patch_type == Some(TaskType::Task)){
        return chore_to_task(patch.clone(), existing_task.clone());
    }
    //Existing = Chore, Patch = Homework;
    if(existing_type == TaskType::Chore && patch_type == Some(TaskType::Homework)){
        return chore_to_homework(patch.clone(), existing_task.clone());
    }
    //Existing = Homework, Patch = Task;
    if(existing_type == TaskType::Homework && patch_type == Some(TaskType::Task)){
        return homework_to_task(patch.clone(), existing_task.clone());
    }
     //Existing = Homework, Patch = Chore;
    if(existing_type == TaskType::Homework && patch_type == Some(TaskType::Chore)){
        return homework_to_chore(patch.clone(), existing_task.clone());
    }

    //Unreachable
    return None;


}

pub fn task_to_task(patch: TaskPatch, existing_task: Task) -> Option<Task>{
    if(patch.description != None || patch.size != None || patch.course != None || patch.dueDate != None || patch.details != None){
        return None;
    }
    return Some(Task{
        id: existing_task.id,
        ownerId: existing_task.ownerId,
        task_type: existing_task.task_type,
        status: patch.status.unwrap_or(existing_task.status),
        description: None,
        size: None,
        course: None,
        dueDate: None,
        details: None
    });
}

pub fn chore_to_chore(patch: TaskPatch, existing_task: Task) -> Option<Task>{
    if(patch.course != None || patch.dueDate != None || patch.details != None){
        return None;
    }
    return Some(Task{
        id: existing_task.id,
        ownerId: existing_task.ownerId,
        task_type: existing_task.task_type,
        status: patch.status.unwrap_or(existing_task.status),
        description: patch.description.or(existing_task.description),
        size: patch.size.or(existing_task.size),
        course: None,
        dueDate: None,
        details: None
    });
}

pub fn homework_to_homework(patch: TaskPatch, existing_task: Task) -> Option<Task>{
    if(patch.description != None || patch.size != None){
        return None;
    }
    return Some(Task{
        id: existing_task.id,
        ownerId: existing_task.ownerId,
        task_type: existing_task.task_type,
        status: patch.status.unwrap_or(existing_task.status),
        description: None,
        size: None,
        course: patch.course.or(existing_task.course),
        dueDate: patch.dueDate.or(existing_task.dueDate),
        details: patch.details.or(existing_task.details)
    });
}

pub fn chore_to_task(patch: TaskPatch, existing_task: Task) -> Option<Task>{
    if(patch.description != None || patch.size != None || patch.course != None || patch.dueDate != None || patch.details != None){
        return None;
    }
    return Some(Task{
        id: existing_task.id,
        ownerId: existing_task.ownerId,
        task_type: patch.task_type.unwrap(),
        status: patch.status.unwrap_or(existing_task.status),
        description: None,
        size: None,
        course: None,
        dueDate: None,
        details: None
    });
}

pub fn chore_to_homework(patch: TaskPatch, existing_task: Task) -> Option<Task>{
    if(patch.description != None || patch.size != None || patch.course == None || patch.dueDate == None || patch.details == None){
        return None;
    }
    return Some(Task{
        id: existing_task.id,
        ownerId: existing_task.ownerId,
        task_type: patch.task_type.unwrap(),
        status: patch.status.unwrap_or(existing_task.status),
        description: None,
        size: None,
        course: patch.course,
        dueDate: patch.dueDate,
        details: patch.details
    });
}

pub fn homework_to_task(patch: TaskPatch, existing_task: Task) -> Option<Task>{
    if(patch.description != None || patch.size != None || patch.course != None || patch.dueDate != None || patch.details != None){
        return None;
    }
    return Some(Task{
        id: existing_task.id,
        ownerId: existing_task.ownerId,
        task_type: patch.task_type.unwrap(),
        status: patch.status.unwrap_or(existing_task.status),
        description: None,
        size: None,
        course: None,
        dueDate: None,
        details: None
    });
}

pub fn homework_to_chore(patch: TaskPatch, existing_task: Task) -> Option<Task>{
    if(patch.description == None || patch.size == None || patch.course != None || patch.dueDate != None || patch.details != None){
        return None;
    }
    return Some(Task{
        id: existing_task.id,
        ownerId: existing_task.ownerId,
        task_type: existing_task.task_type,
        status: patch.status.unwrap_or(existing_task.status),
        description: patch.description.or(existing_task.description),
        size: patch.size.or(existing_task.size),
        course: None,
        dueDate: None,
        details: None
    });
}

pub fn fetch_task_by_id_2(id: i64) -> Option<Json<Task>> {
    //connect to sqllite
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection, //returns connection if success
        Err(_) => {return None;}
    };


    let mut statement = match db_connection.prepare(&format!("select * from tasks where id = {};",[&id][0])) { 
        Ok(statement) => statement,
        Err(_) => {return None;}//else prints error
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
                    Ok(tasks) => { if(tasks.len() == 0) { return None; } Some(Json(tasks[0].clone()))},
                    Err(why) => None
                }
            }
                
            Err(err) => {return None;}
        }
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
                    let status = task.status;

                    Ok(Json(status_to_string(status))) }
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

    let task_type = type_to_string(task.task_type);
    let status = status_to_string(task.status);

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
        
    let task_type = type_to_string(task.task_type);
    let status = status_to_string(task.status);
    
    let description =
        match &task.description {
            Some(t) => t,
            None => return Err("Incorrect data type sent".into()),
        };
    
    let size_opt = size_to_string(task.size);
    if size_opt == None {
        return Err("No size field".to_string());
    }
    let size = size_opt.unwrap();

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
        
    let task_type = type_to_string(task.task_type);
    let status = status_to_string(task.status);

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

pub fn change_task(id:i64, patch:Json<TaskPatch>) -> Result<Json<StatusMessage>, String> {

    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };

    let existing_task_opt = fetch_task_by_id_2(id);

    let existing_task =
    match existing_task_opt{
        Some(t) => t,
        None => { return Err("Illegal value from GET".to_string()); }
    };

    let change_opt = patch_by_case(patch, existing_task);
    let change = 
    match change_opt{
        Some(t) => t,
        None => { return Err("Illegal patch".to_string()); }
    };

    println!("Change task type: {0}", type_to_string(change.task_type));
    
    let mut statement =
        match db_connection.prepare("UPDATE tasks SET task_type = (?1), status = (?2), description = (?3), size = (?4), 
        course = (?5), dueDate = (?6), details = (?7) WHERE id = (?8); ") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 

        let results = statement.execute([&type_to_string(change.task_type) as &dyn ToSql, &status_to_string(change.status),
        &change.description, &size_to_string(change.size), &change.course, &change.dueDate, &change.details, &id.to_string()]);

        match results {
            Ok(rows_affected) => Ok(Json(StatusMessage {
                message: format!("{} rows updated!", rows_affected),
            })),
            Err(err) => Err(format!("{:?}", err))   
        };
    
    Ok(Json(StatusMessage { message: "finished!".to_string()}))
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