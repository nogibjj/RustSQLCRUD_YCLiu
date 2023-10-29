
use rusqlite::{Connection, Result};

fn main ()-> Result<()> {
    println!("Create Database...");
    let conn = Connection::open("Transactions.db")?;
    
    println!("Create Customer Table...");
    conn.execute(
        "create table if not exists Customer (
            cust_id TEXT, name TEXT, sex TEXT);
            INSERT INTO Customer VALUES ('001','John','Male');
            INSERT INTO Customer VALUES ('002','Devin','Female');
            INSERT INTO Customer VALUES ('003','Sharon','Female');",()
    )?;
    println!("Querying...");
    //Read (Select)
    //let mut stmt = conn.prepare("SELECT * FROM Customer;")?;
    
    //let rows = stmt.query_map([], |row| {
    //    Ok((row.get(0)?, row.get(1)?, row.get(2)?))
    //})?;
    

    //for row in rows {
    //    let (cust_id, name, sex): (String, String, String) = row?;
    //    println!("Customer ID: {}, Name: {}, Sex: {}", cust_id, name, sex);
    //}

    Ok(())
}    

