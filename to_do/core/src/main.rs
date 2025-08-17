mod api;
mod enums;
mod structs;

use api::basic_actions::create::create;

fn main() {
    let todo_item = create("washing", enums::TaskStatus::PENDING);
    println!("{}", todo_item);
}
