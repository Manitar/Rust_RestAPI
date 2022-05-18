# Rust_RestAPI

Instructions on how to run:
1) Type "cargo run" in terminal.
2) Go to postman.com, download Desktop Client and run from there.
3) Type in the main url thing: http://localhost:3000/api
4) /people, /people/id, /people/id/tasks, /tasks/id, /tasks/id/status, /tasks/id/owner are the possible urls.

When I write id, type in the actual id used in the objects. Person id for people/id, task id for tasks/id
 
To do list:
1) Finish Task Patch, don't forget to divide into Task, Chore and Homework, to stay true to them.
2) Check what other functions are left, and tests everything in postman.
 
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





