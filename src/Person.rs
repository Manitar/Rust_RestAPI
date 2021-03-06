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
    id: Option<i64>,
    name: String,
    email: String,
    favoriteProgrammingLanguage: String 
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PersonRaw {
    name: String,
    email: String,
    favoriteProgrammingLanguage: String 
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PersonPatch {
    name: Option<String>,
    email: Option<String>,
    favoriteProgrammingLanguage: Option<String>
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


pub fn add_person(person:Json<PersonRaw>) -> Result<Json<StatusMessage>, String> {
    
    //connection
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };


    let mut statement =
        match db_connection.prepare("insert into people (name, email, favoriteProgrammingLanguage) values (?1, ?2 ,?3);") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 

    let add_person = person.0;
    let name = &add_person.name;
    let email = &add_person.email;
    let favoriteProgrammingLanguage = &add_person.favoriteProgrammingLanguage;

    let results = statement.execute([name.to_string(),
    email.to_string(), favoriteProgrammingLanguage.to_string()]);

    match results {
        Ok(rows_affected) => Ok(Json(StatusMessage {
            message: format!("{} rows inserted!", rows_affected),
        })),
        Err(err) => Err(format!("{:?}", err))   
    }

}



pub fn change_person(id:i64 ,person:Json<PersonPatch>)  -> Result<Json<StatusMessage>, String> {
    //connection
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => {
            return Err(String::from("Failed to connect to database"));
        }
    };
    let change_person = person.0;

    let check_name = change_person.name;
    let check_email = change_person.email;
    let check_favoriteProgrammingLanguage = change_person.favoriteProgrammingLanguage;

    let mut check_name_flag = false;
    let mut check_email_flag = false;
    let mut check_favoriteProgrammingLanguage_flag = false;

    let name =
    match check_name {
        Some(t) => {check_name_flag = true; t},
        None =>  "".to_string(),
    };

    let email =
    match check_email {
        Some(t) => {check_email_flag = true; t},
        None =>  "".to_string(),
    };

    let favoriteProgrammingLanguage =
    match check_favoriteProgrammingLanguage {
        Some(t) => {check_favoriteProgrammingLanguage_flag = true; t},
        None =>  "".to_string(),
    };



    if check_name_flag {
        let mut statement =
        match db_connection.prepare("UPDATE people SET name = (?1) WHERE id = (?2); ") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 

        let results = statement.execute([name.to_string() , id.to_string()]);

        match results {
            Ok(rows_affected) => Ok(Json(StatusMessage {
                message: format!("{} rows updated!", rows_affected),
            })),
            Err(err) => Err(format!("{:?}", err))   
        };
    }
    if check_email_flag {
        let mut statement =
        match db_connection.prepare("UPDATE people SET email = (?1) WHERE id = (?2); ") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 

        let results = statement.execute([email.to_string(), id.to_string()]);

        match results {
            Ok(rows_affected) => Ok(Json(StatusMessage {
                message: format!("{} rows updated!", rows_affected),
            })),
            Err(err) => Err(format!("{:?}", err))   
        };
    }
    if check_favoriteProgrammingLanguage_flag {
        let mut statement =
        match db_connection.prepare("UPDATE people SET favoriteProgrammingLanguage = (?1) WHERE id = (?2); ") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        }; 

        let results = statement.execute([favoriteProgrammingLanguage.to_string(), id.to_string()]);

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
