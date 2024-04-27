use rusqlite::{Connection, Result};

struct FileEntry {
    file_name: String,
    file_path: String,
}

pub fn search(table_name:&str, query:&str) -> Result<()>{
    let mut conn = Connection::open("perch.db")?;
    
    let num_of_rows:i32 = conn.query_row(&format!("SELECT COUNT(*) FROM {}", table_name), [], |row| {row.get(0)})?; 
    
    let mut stmt = conn.prepare(&format!("SELECT * FROM {} WHERE file_name LIKE '%{}%'", table_name, query))?;
    
    let results = stmt.query_map([], |row| {
        Ok(FileEntry {
            file_name: row.get(1)?,
            file_path: row.get(2)?,
        })
    })?;

    for result in results.flatten() {
        println!("{} {}", result.file_name, result.file_path);
    }

    
    Ok(())
}
