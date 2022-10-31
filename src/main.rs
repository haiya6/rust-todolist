use std::{io::stdin, fmt::Display};

struct Todo {
    id: i32,
    content: String,
    done: bool
}

impl Todo {
    fn new(id: i32, content: String) -> Todo {
        Todo {
            id,
            content,
            done: false
        }
    }
}

impl Display for Todo {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(print!("id: {}, content: {}, done: {}", self.id, self.content, self.done))
    }
}

fn main() {
    let mut id = 0;
    let mut todo_list: Vec<Todo> = Vec::new();
    let mut input = String::new();

    loop {
        print_todo_list(&todo_list);
        print_menu();
        input.clear();
        stdin().read_line(&mut input).unwrap();

        if input.trim() == "1" {
            println!("输入需要添加的 TODO：");
            input.clear();
            stdin().read_line(&mut input).unwrap();
            let will_add_todo = Todo::new(id, input.trim().to_string());
            id += 1;
            todo_list.push(will_add_todo);
            println!("添加成功 TODO: {}", todo_list.last().unwrap().content);
            println!();
        } else if input.trim() == "2" {
            println!("请输入需要删除的 TODO ID：");
            input.clear();
            stdin().read_line(&mut input).unwrap();
            remove_todo_by_id(&mut todo_list, input.trim().parse().unwrap());
        } else if input.trim() == "3" {
            println!("请输入需要完成的 TODO ID：");
            input.clear();
            stdin().read_line(&mut input).unwrap();
            done_todo_by_id(&mut todo_list, input.trim().parse().unwrap());
        } else if input.trim() == "4" {
            break;
        } else {
            println!("找不到指令");
        }
    }
}

fn print_menu() {
    println!("这是菜单：");
    println!("----");
    println!("1. 添加 TODO");
    println!("2. 移除 TODO");
    println!("3. 完成 TODO");
    println!("4. 退出");
    println!("----");
}

fn print_todo_list(list: &Vec<Todo>) {
    println!("当前的已存在的 TODO：");

    for item in list.iter() {
        println!("{}", item);
    }
    
    println!("----");
    println!();
}

fn get_todo_index_by_id(todo_list: &mut Vec<Todo>, id: i32) -> Option<usize> {
    for (index, item) in todo_list.iter().enumerate() {
        if item.id == id {
            return Some(index);   
        }
    }
    return None;
}

fn remove_todo_by_id(todo_list: &mut Vec<Todo>, id: i32) {
    let will_remove_index = get_todo_index_by_id(todo_list, id);

    if let Some(index) = will_remove_index {
        let removed_todo = todo_list.remove(index);
        println!("已删除 TODO: {}", removed_todo.content);
        return;
    }

    println!("没有找到 TODO：{}", id);
    println!();
}

fn done_todo_by_id(todo_list: &mut Vec<Todo>, id: i32) {
    let will_done_index = get_todo_index_by_id(todo_list, id);

    if let Some(index) = will_done_index {
        todo_list[index].done = true;
    }
}
