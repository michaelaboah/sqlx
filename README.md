# Personal SQLx

This is a personal project that uses SQLx, a Rust library that provides a safe, ergonomic, and efficient API for working with SQL databases.

### Getting Started

To start using this project, you will need to have a SQLite database set up and ready to go. You can create a new SQLite database by running the following command in your terminal:

`sqlite3 path/to/database.db`

You will also need to have the latest version of Rust installed on your machine. You can check your current version of Rust by running the following command in your terminal:

`rustc --version`

To run this project, you can use the following command:

`cargo run`

Dependencies

This project uses the following dependencies:

- tokio
- sqlx
- serde
- serde_json
- dotenvy
- num
- num-derive
- num-traits

These dependencies are specified in the Cargo.toml file and will be automatically installed when running the project.
Usage

This project uses SQLx to perform CRUD operations on the SQLite database.

The project is structured as follows:

    - mod **sql**: The main module which contains the functions for the setup of the database and the CRUD operations.
    - mod **database_setup**: A module that handles the setup of the SQLite database.
    - mod **sql_setup**: A submodule of database_setup that contains the functions for creating the tables and the triggers.
    - mod **entities**: A module that contains the structs and enums used to interact with the database.
    - mod **creation_structs**: A submodule of entities that contains the structs used to create new entries in the database.
    - mod **enums**: A submodule of entities that contains the enums used to interact with the database.
    - mod **field_structs**: A submodule of entities that contains the structs used to interact with the database.
    - mod **struct_parsing**: A submodule of entities that contains the functions used to parse the structs from the database.
    - mod **structs**: A submodule of entities that contains the struct
