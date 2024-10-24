use assert_cmd::Command;
use predicates::prelude::*;
use rusqlite::{Connection, Result}; // Import for the database connection

#[test]
fn test_create_table() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    let table_name = "test_table";
    sqlite::create_table(&conn, table_name)?;

    // Check if the table was created by executing a query against sqlite_master
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
    let table_exists = stmt.exists([table_name])?;
    assert!(table_exists, "Table was not created.");

    Ok(())
}

#[test]
fn test_update_table() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    let table_name = "test_table";
    sqlite::create_table(&conn, table_name)?;

    // Insert some data
    conn.execute("INSERT INTO test_table (year, month, date_of_month, day_of_week, births) VALUES (2023, 10, 22, 6, 500)", [])?;

    // Update the data
    sqlite::update_table(&conn, table_name, "births = 600", "year = 2023 AND month = 10")?;

    // Check if the update was successful
    let mut stmt = conn.prepare("SELECT births FROM test_table WHERE year = 2023 AND month = 10")?;
    let updated_births: i32 = stmt.query_row([], |row| row.get(0))?;
    assert_eq!(updated_births, 600, "Update operation failed.");

    Ok(())
}

#[test]
fn test_drop_table() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    let table_name = "test_table";
    sqlite::create_table(&conn, table_name)?;

    // Drop the table
    sqlite::drop_table(&conn, table_name)?;

    // Check if the table was dropped by querying sqlite_master
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
    let table_exists = stmt.exists([table_name])?;
    assert!(!table_exists, "Table was not dropped.");

    Ok(())
}
