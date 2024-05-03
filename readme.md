## Description

This project is a full-stack Rust application featuring CRUD operations implemented with Axum for the backend, PostgreSQL for the database, and AWS Cognito for user authentication. The frontend is built with Yew, HTML, and CSS, providing a GUI interface for interacting with the backend database.



## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Backend](#backend)
- [CRUD Implementation](#crud-implementation)
- [Middleware](#middleware)
- [Database](#database)
- [Cloud Service](#cloud-service)
- [Frontend](#frontend)




## Installation

To get started with the project, follow these steps:

1. Clone the repository
2. Ensure you have Rust installed. If not, follow the installation instructions from 
3. Install project dependencies: $ cargo build
4. Set up environment variables by creating a .env file with the required configuration for AWS Cognito and database connection.
5. Install necessary tools: Trunk: $ cargo install trunk  
6. After installation, you can run the project locally:
    For the backend : $ cargo run
    For the frontend: $ trunk serve

## Backend
## CRUD Implementation
The backend utilizes Axum to implement CRUD operations. Each endpoint handles Create, Read, Update, and Delete operations for the corresponding resources.

## Middleware
Middleware is implemented to handle authentication and other cross-cutting concerns.


## Database
PostgreSQL is used as the database to store application data. Ensure you have PostgreSQL installed and running, and update the .env file with the appropriate database connection details.

## Cloud Service
AWS Cognito is used for user authentication. Ensure you have set up a user pool in AWS Cognito and created an app client. Update the .env file with the necessary credentials.



## Frontend
The frontend is built with Yew, providing a rich user interface for interacting with the backend. It establishes connections with the backend database and implements CRUD operations.

Contributing
Contributions are welcome! Feel free to open issues or submit pull requests.








