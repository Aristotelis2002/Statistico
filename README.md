# Statistico
## How to run it locally
First clone the repo.  
Run the server by opening a terminal in the backend folder and running the command:  
```cargo run```  
After that run the shell script named *frontend.sh*  
Open the index.html and log in with a user id (every id is a different account).  
## Documentation
### Introduction
The website provides a user-friendly platform that allows you to create tables with data and enables you to visualize the information onto a bar chart.  
### Implementation
The project predominantly relies on the 'actix-web' and 'diesel sql' crates for its core functionality.  
**Database Management**: This project utilizes SQLite for database management.  
For the visualization of the charts I am using the crate 'charts'.  
In the api folder the controllers modules can be found. It's simple to understand how they function. More functionality can be added easily to the project by defining a new function and attaching it to the 'App' in main.rs as a service.  
