use std::io::stdin;

struct Todo {
    id: i32,
    content: String
}

impl Todo {
    fn new(id: i32, content: String) -> Todo {
        Todo { id, content }
    }
}

fn main() {
    let mut id = 0;
    let mut todos: Vec<Todo> = Vec::new();

    loop {
        print_menu();
        let mut input = String::new();
        // stdin().read_line(&mut input);

        // println!("content: {}", content);
    }
}

fn print_menu() {
    println!("1. 添加 TODO");
    println!("2. 移除 TODO");
    println!("3. 完成 TODO");
    println!("4. 退出");
}

fn add_todo(s: String) {
    
}

fn remove_todo() {

}

fn done_todo() {
    
}
