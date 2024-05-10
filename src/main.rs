mod window;
mod ui;
mod search; 
mod index;
mod sort;

fn main() {
    let search_results = search::search("home", "hello");
    println!("{:?}", search_results.unwrap());

    ui::start_ui();
}
