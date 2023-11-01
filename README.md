[![Build binary release](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/release.yml/badge.svg)](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/release.yml)
[![Format](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/format.yml/badge.svg)](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/format.yml)
[![Clippy](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/lint.yml/badge.svg)](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/lint.yml)
[![Test](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/test.yml/badge.svg)](https://github.com/nogibjj/RustSQLCRUD_YCLiu/actions/workflows/test.yml)
## SQLite CRUD with Rust 

This repository demonstrates using **rust** a to conduct **CRUD** (Create, Read, Update, Delete) on a **SQLite** database. 

Below is an overview of the files in this project:

1. **Main files for database manipulation** : 
   <br>a. _Cargo.toml_: Specify library names and **depencies used in the project**.
   <br>b. _./sql/lib.rs_: Define functions used in _main.rs_ for database manipulation.
   <br>c. _./sql/main.rs_: Main script executing the following:
   
      <br>         i. Build a SQLite database _Transaction.db_.
      <br>         ii. Create a table named *Customer*, with the following columns: *cust_id*, *name*, *sex*. Below is the content of the resulted table.
      <br>         iii. **Read** (*Select*) **all columns and rows** of the *Customer* table and display the result:
   
      **Original Query Result**

      | cust_id | name | sex |
      |---|---|---|
      |001| John | Male |
      |002| Devin | Female |
      |003| Sharon | Female |

      <br>         iv. **Update** the value of the *sex* column of the **first row**.
      <br>         v. **Read** (*Select*) **all columns and rows** of the *Customer* table and display the result:
   
      ***Updated*** **Query Result**

      | cust_id | name | sex |
      |---|---|---|
      |001| John | **Unknown** |
      |002| Devin | Female |
      |003| Sharon | Female |

      <br>         vi. **Delete** the **first row** the value of the *sex* column of the first row.
      <br>         vii. **Read** (*Select*) **all columns and rows** of the *Customer* table and display the result:

      **Query Result After** ***Deletion***

      | cust_id | name | sex |
      |---|---|---|
      |002| Devin | Female |
      |003| Sharon | Female |


2. **Rust executable binary file**:
  <br>d. _.RustSQLCRUD_YCLiu/sql/target/release/sql_: the released binary file executable using command line tool.

3. **Other files for development environment settings**
  <br>e. _.devcontainer_: set up the environment for development.
  <br>f. _.gitignore_: specify file names to ignore (using Github rust default except for *target*, the executable).

4. **Github actions setup for continuous integration**
  <br>g. _.github/workflows/~.yml_: Quality control actions are triggered when pushed/ pulled to main branch. After setting up the environment, actions of **releasing packages** (_release.yml_), **formatting** (_format.yml_), **linting** (_lint.yml_), and **testing** (_test.yml_) would be executed in order (as specified in _Makefile_). 

5. **Description of the project**
   <br>h. _README.md_: THIS FILE, explaining the purpose and structure of the directory, with visualized example output.

6. **Note on the use of Copilot**: _Copilot did not help much_ for this project. When translated from python to rust, the code provided did not work well.
