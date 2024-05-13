/*
 *
 *
Copyright (C) 2023,2024 Carl Marino
This file is part of Perch.
Perch is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or any later version.
Perch is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
You should have received a copy of the GNU General Public License along with Perch. If not, see <https://www.gnu.org/licenses/>.
*/


use rusqlite::{Connection, Result};

struct ItemEntry {
    item_name: String,
    item_info: String,
}

pub fn search(table_name:&str, query:&str) -> Result<Vec<Vec<String>>>{
    let mut conn = Connection::open("perch.db")?;
    
    let num_of_rows:i32 = conn.query_row(&format!("SELECT COUNT(*) FROM {}", table_name), [], |row| {row.get(0)})?; 
    
    let mut stmt = conn.prepare(&format!("SELECT * FROM {} WHERE item_name LIKE '{}%'", table_name, query))?;
    let mut final_res:Vec<Vec<String>> = Vec::new();
    
    let results = stmt.query_map([], |row| {
        Ok(ItemEntry {
            item_name: row.get(1)?,
            item_info: row.get(2)?,
        })
    })?;

    for result in results.flatten() {
        final_res.push(vec![result.item_name, result.item_info]);
    }

    
    Ok(final_res)
}
