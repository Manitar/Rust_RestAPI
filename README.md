# Rust_RestAPI

Instructions on how to run:
1) Type "cargo run" in terminal.
2) Go to postman.com, download Desktop Client and run from there.
3) Type in the main url thing: http://localhost:3000/api
4) /people, /people/id, /people/id/tasks, /tasks/id, /tasks/id/status, /tasks/id/owner are the possible urls.

When I write id, type in the actual id used in the objects. Person id for people/id, task id for tasks/id
 
To do list:
1) Check in postman: PATCH: All 9 different types, and the whole API.
2) Case sensitivity for Chore, Task, Homework??
3) Case sensitivity for Active, Done: Put status "active" works, while get and post "active" doesn't work, because of small letter.
4) Put status and put ownerId not working.
5) Tests left: Put status, put ownerId, get ownerId, patch task.
 
Swagger link:
https://mbarsinai.com/files/bgu/2022a/miniproj/swagger/#/default

JSONs to work with:

POST: http://localhost:3000/api/people
{
  "name": "D. J. Wheeler",
  "email": "djw@ESDAC.uk",
  "favoriteProgrammingLanguage": "EDSAC Assembly"
}

POST: http://localhost:3000/api/people/<ID>/tasks
{
  "type": "Chore",
  "status": "Active",
  "description": "Buy milk.",
  "size": "Large"
}

POST: http://localhost:3000/api/people/<ID>/tasks
{
  "type": "Homework",
  "status": "Active",
  "course": "Logics",
  "dueDate": "10/5/2021",
  "details": "Very good"
}





