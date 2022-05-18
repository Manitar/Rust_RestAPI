# Rust_RestAPI

Instructions on how to run:
1) Type "cargo run" in terminal.
2) Go to postman.com, download Desktop Client and run from there.
3) Type in the main url thing: http://localhost:3000/api
4) /people, /people/id, /people/id/tasks, /tasks/id, /tasks/id/status, /tasks/id/owner are the possible urls.

When I write id, type in the actual id used in the objects. Person id for people/id, task id for tasks/id
 
To do list:
1) Make Raw structs for each struct needed (Raw = no id, regular = yes id). Serde deserialize anyone needed.
2) Change functions in main to get Json < TaskRaw > instead of Json<Vec<String>>.
3) Generalize a task struct to get all possible features.
 
Swagger link:
https://mbarsinai.com/files/bgu/2022a/miniproj/swagger/#/default



