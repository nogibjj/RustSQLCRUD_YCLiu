[![Rustfmt](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/rustfmt.yml)
[![Clippy](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/lint.yml)
[![Test](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/test.yml/badge.svg)](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/test.yml)
## SQLite CRUD with Rust 

This repository demonstrates using **rust** a to conduct **CRUD** (Create, Read, Update, Delete) on a **SQLite** database. 

Below is an overview of the files in this project:

1. **sql** : rust
   <br>a. _Cargo.toml_: Specify library names and **depencies used in the project**.
   <br>b. _main.rs_: Main function executing the following: 

   <br>         1. Build a SQLite database _Transaction.db_.
   <br>         2. Create a table named *Customer*, with the following columns: *cust_id*, *name*, *sex*. Below is the content of the resulted table.
   <br>         3. **Read** (*Select*) **all columns and rows** of the *Customer* table and display the result:
   
**Original Query Result**

| cust_id | name | sex |
|---|---|---|
|001| John | Male |
|002| Devin | Female |
|003| Sharon | Female |

   <br>         4. **Update** the value of the *sex* column of the **first row**.
   <br>         5. **Read** (*Select*) **all columns and rows** of the *Customer* table and display the result:
   
***Updated*** **Query Result**

| cust_id | name | sex |
|---|---|---|
|001| John | **Unknown** |
|002| Devin | Female |
|003| Sharon | Female |

<br>         4. **Delete** the **first row** the value of the *sex* column of the first row.
<br>         5. **Read** (*Select*) **all columns and rows** of the *Customer* table and display the result:

**Query Result After** ***Deletion***

| cust_id | name | sex |
|---|---|---|
|002| Devin | Female |
|003| Sharon | Female |

 

2. **Other files for development environment settings**
  <br>c. _.devcontainer_: set up the environment for development.
  <br>d. _.gitignore_: specify file names to ignore (using Github rust default except for *target*, the executable).
  

6. **Description of the project**
   <br>e. _README.md_: THIS FILE, explaining the purpose and structure of the directory, with visualized example output.

