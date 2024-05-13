/*
 *
 *
Copyright (C) 2023,2024 Carl Marino
This file is part of Perch.
Perch is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or any later version.
Perch is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
You should have received a copy of the GNU General Public License along with Perch. If not, see <https://www.gnu.org/licenses/>.
*/


mod window;
mod ui;
mod search; 
mod index;
mod sort;

fn main() {
    index::index_apps();
//    let search_results = search::search("home", "hello");
//    println!("{:?}", search_results.unwrap());

    ui::start_ui();
}
