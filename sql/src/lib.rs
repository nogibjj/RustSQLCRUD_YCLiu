// lib.rs
use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Customer {
    cust_id: String,
    name: String,
    sex: String,
}

pub fn create(conn: &Connection) -> Result<()> {
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Customer (
            cust_id TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            sex TEXT NOT NULL
        )",
        [],
    )?;
    
    Ok(())
}

pub fn insert(conn: &Connection) -> Result<()> {
    conn.execute( "INSERT INTO Customer (cust_id, name, sex)
              VALUES (?1, ?2, ?3)", params!["001", "John", "Male"])?;
    conn.execute( "INSERT INTO Customer (cust_id, name, sex)
              VALUES (?1, ?2, ?3)", params!["002", "Devin", "Female"])?;              
    conn.execute( "INSERT INTO Customer (cust_id, name, sex)
              VALUES (?1, ?2, ?3)", params!["003", "Sharon", "Female"])?;                  
    Ok(())                        
            
}

pub fn query(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM Customer")?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<usize, String>(0)?,
            row.get::<usize, String>(1)?,
            row.get::<usize, String>(2)?
        ))
    })?;
    for row in rows {
        println!("{:?}", row?);
    }
    Ok(())
}

pub fn update(conn: &Connection) -> Result<()> {
    conn.execute(
        "UPDATE Customer SET sex = 'Unknown' WHERE cust_id = '001'",
        [],
    )?;
    Ok(())
}

pub fn delete(conn: &Connection) -> Result<()> {
    conn.execute(
        "DELETE FROM Customer WHERE cust_id = '001' ",
        [],
    )?;
    Ok(())
}

pub fn drop(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE IF EXISTS Customer", [])?;
    Ok(())
}