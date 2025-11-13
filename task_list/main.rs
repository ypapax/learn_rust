use std::io;


fn main(){
    println!("This todo list");
    let mut tasks: Vec<String> = Vec::new();
    loop {
        println!("please input command to execute:");
        println!("1. Adding an item");
        println!("2. List todo list");
        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1"=>add_item(&mut tasks),
            "2"=>list_items(&tasks),
            _=>println!("Wrong command")
        }
    }

}

fn add_item(tasks: &mut Vec<String>){
    println!("Please input todo list item text");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    if input.trim().len()>0{
        tasks.push(input);
    }
}

fn list_items(tasks: & Vec<String>){
    println!("Here is task list: ");
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. {}", i+1, task)
    }
}