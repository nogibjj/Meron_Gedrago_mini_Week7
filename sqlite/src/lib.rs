use csv::ReaderBuilder; // for loading from CSV
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File; // for loading CSV and capturing errors from loading

// Create a table
pub fn create_table(conn: &Connection, table_name: &str) -> Result<()> {
    let create_query = format!(
        "CREATE TABLE IF NOT EXISTS {} (
            year INTEGER NOT NULL,
            month INTEGER NOT NULL,
            date_of_month INTEGER NOT NULL,
            day_of_week INTEGER NOT NULL,
            births INTEGER NOT NULL
        )",
        table_name
    );
    conn.execute(&create_query, [])?;
    println!("Table '{}' created successfully.", table_name);
    Ok(())
}

// Read
pub fn query_exec(conn: &Connection, query_string: &str) -> Result<()> {
    let mut stmt = conn.prepare(query_string)?;

    // Use query_map to handle multiple rows
    let rows = stmt.query_map([], |row| {
        let year: i32 = row.get(0)?;
        let month: i32 = row.get(1)?;
        let date_of_month: i32 = row.get(2)?;
        let day_of_week: i32 = row.get(3)?;
        let births: i32 = row.get(4)?;
        Ok((year, month, date_of_month, day_of_week, births))
    })?;

    // Iterate over the rows and print the results
    for row in rows {
        let (year, month, date_of_month, day_of_week, births) = row?;
        println!(
            "Year: {}, month: {}, Date of month: {}, Day of week: {}, Births: {}",
            year, month, date_of_month, day_of_week, births
        );
    }

    Ok(())
}

// Delete
pub fn drop_table(conn: &Connection, table_name: &str) -> Result<()> {
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    Ok(())
}

// Load data from a file path to a table
pub fn load_data_from_csv(
    conn: &Connection,
    table_name: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path).expect("failed to open the file path");
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let insert_query = format!(
        "INSERT INTO {} (year, month, date_of_month, day_of_week, births) VALUES (?, ?, ?, ?, ?)",
        table_name
    );

    for result in rdr.records() {
        let record = result.expect("failed to parse a record");
        let year_str = record[0].trim();
        let year: i32 = match year_str.parse() {
            Ok(year) => year,
            Err(_) => {
                println!("Failed to parse year '{}'", year_str);
                0
            }
        };
        println!("year: {}", year);
        let month: f32 = record[1].trim().parse()?;
        let date_of_month: f32 = record[2].trim().parse()?;
        let day_of_week: f32 = record[3].trim().parse()?;
        let births: f32 = record[4].trim().parse().expect("failed to parse births");

        conn.execute(
            &insert_query,
            params![year, month, date_of_month, day_of_week, births],
        )
        .expect("failed to execute data into db table");
    }

    println!(
        "Data loaded successfully from '{}' into table '{}'.",
        file_path, table_name
    );
    Ok(())
}

pub fn update_table(
    conn: &Connection,
    table_name: &str,
    set_clause: &str,
    condition: &str,
) -> Result<()> {
    let update_query = format!(
        "UPDATE {} SET {} WHERE {};",
        table_name, set_clause, condition
    );

    let affected_rows = conn.execute(&update_query, [])?;

    println!(
        "Successfully updated {} row(s) in table '{}'.",
        affected_rows, table_name
    );

    Ok(())
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_create_table() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        let table_name = "test_table";
        create_table(&conn, table_name)?;

        let mut stmt =
            conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
        let table_exists = stmt.exists([table_name])?;
        assert!(table_exists, "Table was not created.");

        Ok(())
    }

    #[test]
    fn test_update_table() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        let table_name = "test_table";
        create_table(&conn, table_name)?;

        conn.execute("INSERT INTO test_table (year, month, date_of_month, day_of_week, births) VALUES (2023, 10, 22, 6, 500)", [])?;

        update_table(
            &conn,
            table_name,
            "births = 600",
            "year = 2023 AND month = 10",
        )?;

        let mut stmt =
            conn.prepare("SELECT births FROM test_table WHERE year = 2023 AND month = 10")?;
        let updated_births: i32 = stmt.query_row([], |row| row.get(0))?;
        assert_eq!(updated_births, 600, "Update operation failed.");

        Ok(())
    }

    #[test]
    fn test_drop_table() -> Result<()> {
        let conn = Connection::open_in_memory()?;
        let table_name = "test_table";
        create_table(&conn, table_name)?;

        drop_table(&conn, table_name)?;

        let mut stmt =
            conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
        let table_exists = stmt.exists([table_name])?;
        assert!(!table_exists, "Table was not dropped.");

        Ok(())
    }
}
