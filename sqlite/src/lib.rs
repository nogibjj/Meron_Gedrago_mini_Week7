use csv::ReaderBuilder; //for loading from csv
use rusqlite::{params, Connection, Result};
use std::error::Error;
use std::fs::File; //for loading csv //for capturing errors from loading
                   // Here we will have a function for each of the commands

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
    Ok(()) //returns nothing except an error if it occurs
}

//Read
pub fn query_exec(conn: &Connection, query_string: &str) -> Result<()> {
    // Prepare the query and iterate over the rows returned
    let mut stmt = conn.prepare(query_string)?;

    // Use query_map to handle multiple rows
    let rows = stmt.query_map([], |row| {
        // Assuming the `users` table has an `id` and `name` column
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

//delete
pub fn drop_table(conn: &Connection, table_name: &str) -> Result<()> {
    let drop_query = format!("DROP TABLE IF EXISTS {}", table_name);
    conn.execute(&drop_query, [])?;
    println!("Table '{}' dropped successfully.", table_name);
    Ok(())
}

//load data from a file path to a table
pub fn load_data_from_csv(
    conn: &Connection,
    table_name: &str,
    file_path: &str,
) -> Result<(), Box<dyn Error>> {
    //Box<dyn Error> is a trait object that can represent any error type
    let file = File::open(file_path).expect("failed to open the file path");
    let mut rdr = ReaderBuilder::new().from_reader(file);

    let insert_query = format!(
        "INSERT INTO {} (year,month, date_of_month, day_of_week, births) VALUES (?, ?, ?, ?, ?)",
        table_name
    );
    //this is a loop that expects a specific schema, you will need to change this if you have a different schema
    for result in rdr.records() {
        let record = result.expect("failed to parse a record");
        //let year: i32 = record[0].trim().parse().expect("failed to parse year"); //.parse() is a method that converts a string into a number
        let year_str = record[0].trim();
        let year: i32 = match year_str.parse() {
            Ok(year) => year,
            Err(_) => {
                println!("Failed to parse year '{}'", year_str);
                // You can either return an error, panic, or set a default value
                // For example, you can set a default value like this:
                0
            }
        };
        println!("year: {}", year);
        let month: f32 = record[1].trim().parse()?;
        let date_of_month: f32 = record[2].trim().parse()?;
        let day_of_week: f32 = record[3].trim().parse()?;
        let births: f32 = record[4]
            .trim()
            .parse()
            .expect("failed to parse advanced degree");
        println!(
            "Year: {}, month: {}, Date of month: {}, Day of week: {}, Births: {}",
            year, month, date_of_month, day_of_week, births
        );

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
    // Construct the SQL UPDATE query using the provided table name, set clause, and condition
    let update_query = format!(
        "UPDATE {} SET {} WHERE {};",
        table_name, set_clause, condition
    );

    // Execute the update query
    let affected_rows = conn.execute(&update_query, [])?;

    // Output the number of rows updated
    println!(
        "Successfully updated {} row(s) in table '{}'.",
        affected_rows, table_name
    );

    Ok(())
}
