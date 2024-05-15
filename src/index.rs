/*
 *
 *
Copyright (C) 2023,2024 Carl Marino
This file is part of Perch.
Perch is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or any later version.
Perch is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
You should have received a copy of the GNU General Public License along with Perch. If not, see <https://www.gnu.org/licenses/>.
*/

use std::fs;
use std::process::Command;
use std::path::Path;
use std::io::prelude::*;
use std::env;
use rusqlite::{Connection, Result, params, Transaction};

use crate::sort; 

pub struct AppEntry {
    pub name: String,
    pub exec: String,
    pub icon: String,
    pub desc: String,
}

//indexes home directory
pub fn index_home() {
    let home_dir = env::var("HOME").unwrap();
    let home_dir_path = Path::new(&home_dir);
    let sorted = sort::merge_sort(super_walk(home_dir_path));
    
    let _ = add_new_index("home", sorted);
}

//indexes all app dirs
pub fn index_apps() {
    let xdg_dirs = env::var("XDG_DATA_DIRS").unwrap();
    let local_app_dir = format!("{}/.local/share/applications", env::var("HOME").unwrap());
    let mut app_dirs: Vec<&str> = Vec::from(xdg_dirs.split(":").collect::<Vec<_>>());
    let mut app_entries: Vec<AppEntry> = Vec::new();
    app_dirs.push(local_app_dir.as_str());
    
    for dir in app_dirs {
        let files = super_walk(Path::new(&dir));

        for file in files {
            let file_content:Vec<&str> = file.split("//").collect();
            let file_ext = file_content[0].split(".").last().unwrap();

            if file_ext == "desktop " {
                
                app_entries.push(read_desktop(&file_content[1][1..]).unwrap());
                 
            } 
        }
    }

    let _ = add_app_index(app_entries);
}


fn read_desktop(file_path: &str) -> Option<AppEntry> {

    let desktop_file = Command::new("cat").arg(file_path).output().expect("no such file or directory");
    let output = String::from_utf8(desktop_file.stdout).unwrap();
    
    let output_lines: Vec<&str> = output.split("\n").collect();
    
    let mut is_app:bool = false;

    let mut app_desc = String::new();
    let mut app_name = String::new();
    let mut app_exec = String::new();
    let mut app_icon = String::new();

    for line in output_lines {
        let line_split:Vec<&str> = line.split("=").collect();
        
        if line == "Type=Application"{
            is_app = true;
        }
        
        else {
            match line_split[0] {
                "Name" => app_name = line_split[1].to_string(),
                "Exec" => app_exec = line_split[1].to_string(),
                "Icon" => app_icon = line_split[1].to_string(),
                "Comment" => app_desc = line_split[1].to_string(),
                &_ => (),
            }
        }

    }

    if is_app == true {
        return Some(AppEntry{name: app_name,exec: app_exec,icon: app_icon,desc: app_desc,});
    }

    return None
}

fn add_app_index(index: Vec<AppEntry>) -> Result<()> {
    let mut conn = Connection::open("perch.db")?;

    conn.execute("DROP TABLE IF EXISTS apps", [])?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS apps (
            id        INTEGER PRIMARY KEY,
            app_name TEXT NOT NULL,
            app_exec TEXT NOT NULL,
            app_icon TEXT NOT NULL,
            app_desc TEXT NOT NULL
        )", 
        [],
    )?;



    let tx = conn.transaction()?;

    insert_app_entries(&tx, index)?;

    tx.commit()?;

    Ok(())
}


fn insert_app_entries(tx: &Transaction, index: Vec<AppEntry>) -> Result<()> {
    let mut stmt = tx.prepare("INSERT into apps (id, app_name, app_exec, app_icon, app_desc) VALUES (?1,?2, ?3, ?4, ?5)")?;
   
    for i in 0..index.len() {
        stmt.execute(params![i, index[i].name, index[i].exec, index[i].icon, index[i].desc])?;
    }
    Ok(())
}

//create sqlite table and each element of the vec to the table
fn add_new_index(table_name: &str, index: Vec<String>) -> Result<()> {
    let mut conn = Connection::open("perch.db")?;

    add_table(table_name, &conn)?;

    let tx = conn.transaction()?;
    
    insert_entries(&tx, index, table_name)?;

    tx.commit()?;

    Ok(())
}

fn add_table(table_name: &str, conn: &Connection) -> Result<()> {

    conn.execute(&format!("DROP TABLE IF EXISTS {}", table_name), [])?;

    conn.execute(&format!(
        "CREATE TABLE IF NOT EXISTS {} (
            id        INTEGER PRIMARY KEY,
            item_name TEXT NOT NULL,
            item_info TEXT NOT NULL
        )", table_name), 
        [],
    )?;

    Ok(())
}
//inserts inserts all entries in a vec to a table using a single transaction 
fn insert_entries(tx: &Transaction, entries: Vec<String>, table_name:&str) -> Result<()> {
    
    let mut stmt = tx.prepare(&format!("INSERT into {} (id, item_name, item_info) VALUES (?1,?2, ?3)", table_name))?;
   
    for i in 0..entries.len() {
        let entry_content:Vec<&str> = entries[i].split("//").collect();
        stmt.execute(params![i,entry_content[0],entry_content[1]])?;
    }
    Ok(())
}

//recursive function that crawles through a directory and its subdirectories
fn super_walk(dir: &Path) -> Vec<String>{
    //gets the walkdir result from the specified directory
    let walkdir_result = walkdir(dir);

    //vector containing the directories contained in the specified directory
    let dirs_found = &walkdir_result.clone()[1];
    //vector containing all the files found
    let mut files_found: Vec<String> = Vec::new();

    //appends the files discovered during the walk of the directory to the files_found vector
    files_found.append(&mut walkdir_result.clone()[0]);
    
    //checks if dirs_found isnt empty
    if !dirs_found.is_empty() {
        //loops through all the directories in dirs_found
        for dir in dirs_found {
            //appends files found in the directory and subdirectories to files_found
            files_found.append(&mut super_walk(Path::new(dir)));
        }
    } 
    //returns files_found
    return files_found; 
}




//gets the contents of a single directory and returns the files and dirs seperatly
pub fn walkdir(dir: &Path) -> [Vec<String>; 2] {

    //String which specifies the file type filter
    let filter:String = String::from("");
    

    //vec that stores the discovered directories
    let mut dirs_found: Vec<String> = Vec::new();
    //vec that stores the discovered files
    let mut files_found: Vec<String> = Vec::new();
    
    //check if the specified path is a directory
    if dir.is_dir() {

        //loop through the directory and find all its entries
        for entry in fs::read_dir(dir).unwrap() {
            //get path from entry
            let path = entry.unwrap().path();

            //check if the path is a directory 
            if path.is_dir() {
                if format!("{}",path.display()).chars().nth(format!("{}",dir.display()).chars().count() +1).unwrap() != '.' {
                    //add path to dirs_found vec
                    dirs_found.push(format!("{}",path.display()));
                }
            }

            //runs if the path doesnt lead to a directory
            else {
                if format!("{}",path.display()).chars().nth(format!("{}",dir.display()).chars().count() +1).unwrap() != '.' {
                    //gets the file extension
                    let file_ext = format!("{}",path.display()).split(".").last().unwrap().to_string();
                    
                    //checks if filter exists and is equal to the current file extension
                    if filter != "" && filter == file_ext{
                        //pushes the path of the file to the files_found vec
                        files_found.push(format!("{} // {}", path.display().to_string().split("/").last().unwrap(),path.display().to_string()));
                    }
                    
                    //runs if there is no filter 
                    else if filter == "" {
                        //pushes the path of the file to the files_found vec
                        files_found.push(format!("{} // {}",path.display().to_string().split("/").last().unwrap(),path.display().to_string()));
                    }
                }
            }
        }
    }
    //returns the two vecs as an array
    [files_found, dirs_found]

}
