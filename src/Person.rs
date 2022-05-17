//use rocket::*;
use rocket_contrib::json::Json;
use rusqlite::Connection;
use serde::Serialize;
use serde::Deserialize;



#[derive(Serialize)]
pub struct StatusMessage {
    pub message: String,
}

#[derive(Serialize)]
pub struct People {
    pub people: Vec<Person>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Person {
    id: String,
    name: String,
    email: String,
    favoriteProgrammingLanguage: String 
}



pub fn fetch_all_people() -> Result<Json<People>, String> {
    //connect to sqllit
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection, //returns connection if success
        Err(_) => {
            return Err(String::from("Failed to connect to database"));//else prints error
        }
    };


    let mut statement = match db_connection.prepare("select * from people;") { //get id,item for table todo_list
        Ok(statement) => statement,//returns if success
        Err(_) => return Err("Failed to prepare query".into()),//else prints error
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| { //creat a todo_item's from all the results
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            favoriteProgrammingLanguage: row.get(3)?,
        })
    });

    match results {
        Ok(rows) => {
            let collection: rusqlite::Result<Vec<Person>> = rows.collect();

            match collection {
                Ok(people) => { Ok(Json(People{people}))},
                Err(_) => Err("Could not collect people".into()),
            }
        }
        Err(_) => Err("Failed to fetch people".into()),
    }
}

pub fn fetch_person(id: i64) -> Result<Json<People>, String> {
    //connect to sqllit
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection, //returns connection if success
        Err(_) => {
            return Err(String::from("Failed to connect to database"));//else prints error
        }
    };


    let mut statement = match db_connection.prepare(&format!("select * from people where id = {};",[&id][0])) { 
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),//else prints error
    };


    let results = statement.query_map(rusqlite::NO_PARAMS, |row| { //creat a todo_item's from all the results
    Ok(Person {
        id: row.get(0)?,
        name: row.get(1)?,
        email: row.get(2)?,
        favoriteProgrammingLanguage: row.get(3)?,
         })
    });

    match results {
        Ok(rows) =>{ 
            let collection: rusqlite::Result<Vec<Person>> = rows.collect();
            match collection {
                Ok(people) => { Ok(Json(People{people}))},
                Err(_) => Err("Could not collect people".into()),
            }
        }
            
        Err(err) => Err(format!("{:?}", err))
    }
}


pub fn add_person(person:Json<Person>, id: String) -> Result<Json<StatusMessage>, String> {
    
    //connection
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };


    let mut statement =
        match db_connection.prepare("insert into people (id, name, email, favoriteProgrammingLanguage) values (?1, ?2 ,?3 ,?4);") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 

    let add_person = person.0;
    let string_id = &id.to_string();
    let name = add_person.name;
    let email = add_person.email;
    let favoriteProgrammingLanguage = add_person.favoriteProgrammingLanguage;

    let results = statement.execute([string_id.to_string(), name.to_string(),
    email.to_string(), favoriteProgrammingLanguage.to_string()]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_affected),
        })),
        Err(err) => Err(format!("{:?}", err))   
    }

}



pub fn change_person(id:i64 ,person:Json<[String;3]>)  -> Result<Json<StatusMessage>, String> {
    //connection
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };
    let changearray = person;
    let check1 = &changearray[0];
    let check2 = &changearray[1];
    let check3 = &changearray[2];
    if !check1.eq("") {
        let mut statement =
        match db_connection.prepare("UPDATE people SET name = (?1) WHERE id = (?2); ") {
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
        match db_connection.prepare("UPDATE people SET email = (?1) WHERE id = (?2); ") {
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
    if !check3.eq("") {
        let mut statement =
        match db_connection.prepare("UPDATE people SET favoriteProgrammingLanguage = (?1) WHERE id = (?2); ") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 

        let results = statement.execute([check3,&id.to_string()]);

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
