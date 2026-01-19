
use std::io;
use owo_colors::OwoColorize;

struct TodoItem {
 
    id: u32,
    name: String,
    completed: bool,
}

fn main() {
    let mut todo_list: Vec<TodoItem> = Vec::new();
    let ascii_art = r#"
    _________              ___.   .____    .__          __   
    \_   ___ \____________ \_ |__ |    |   |__| _______/  |_ 
    /    \  \/\_  __ \__  \ | __ \|    |   |  |/  ___/\   __\
    \     \____|  | \// __ \| \_\ \    |___|  |\___ \  |  |  
    \______  /|__|  (____  /___  /_______ \__/____  > |__|  
            \/            \/    \/        \/       \/        "#;

    println!();
    println!("{}", ascii_art.bright_red().bold());
    println!("{}", "The blazing fast CLI to-do list. ".bright_yellow());

    
    loop {
        println!();
        println!("Enter action: [1-4]");
        println!("[1] Add a to-do item");
        println!("[2] Complete a to-do item");
        println!("[3] Display to-do items");
        println!("[4] Clear full list.");
        println!("[5] Exit");
    

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line. Try again.");

        let choice = choice.trim().parse::<u32>().expect("Invalid input.");
        
        match choice {
            1 => {
                println!("Enter the name of the To-Do item: ");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();

                let id = todo_list.len() as u32 + 1;

                let item = TodoItem {
                    id,
                    name,
                    completed: false,
                };

                todo_list.push(item);
            },

            2 => {
                println!("Enter the ID of the item to mark as complete: ");
                let mut id = String::new();
                io::stdin()
                    .read_line(&mut id)
                    .expect("Failed to read line. Try again.");

                let id = match id.trim().parse::<u32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid ID. Please enter a number.");
                        continue;
                    }
                };

                match todo_list.iter_mut().find(|i| i.id == id) {
                    Some(item) => {
                        complete_item(item);
                        println!("Item {} marked as complete.", id);
                    }
                    None => {
                        println!("No to-do item found with ID {}.", id);
                    }
                }
            }

            3 => {
                display_items(&todo_list);
            },

            4 => {
                clear_items(&mut todo_list);
            }

            5 => {
                println!("Program exited.");
                break;
            },

            _ =>  {
                println!("Invalid choice. Pick from [1-4].");
            }

    }

}


}

fn complete_item(item: &mut TodoItem){
    item.completed = true;
}

fn display_items(items: &Vec<TodoItem>){

    if items.is_empty() {
        println!("No to-do items found.");
        return;
    }

    for item in items {
        let status = if item.completed {
            "completed"
        } 

        else {
            "incomplete"
        };

        println!("[{}] - {} ({})", item.id, item.name, status);

    }
}

fn clear_items(items: &mut Vec<TodoItem>) {
    if items.is_empty() {
        println!("The to-do list is already empty.");
        return;
    }

    println!("Are you sure you want to clear all items? (y/n)");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim().eq_ignore_ascii_case("y") {
        items.clear();
        println!("All to-do items have been cleared.");
    } else {
        println!("Clear operation canceled.");
    }
}

