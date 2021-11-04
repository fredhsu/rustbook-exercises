use std::collections::HashMap;
use std::io;

fn add_user(map: &mut HashMap<String, String>, name: String, department: String) {
    map.insert(name, department);
}

fn list_users(map: &HashMap<String, String>) {
    for (name, department) in map.iter() {
        println!("{} => {}", name, department);
    }
}

fn list_users_department(map: &HashMap<String, String>, department: &str) {
    for (name, dpt) in map {
        if dpt == department {
            println!("{} works in the {} department", name, dpt);
        }
    }
}

fn main() {
    let mut line = String::new();
    let mut users = HashMap::new();
    let mut stdin = io::stdin().read_line(&mut line);
    while let Ok(_) = stdin {
        let mut iter = line.split_whitespace();
        match iter.next() {
            Some("Add") => {
                let name = iter.next().unwrap().to_string();
                iter.next();
                let department = iter.next().unwrap().to_string();
                add_user(&mut users, name, department);
            }
            Some("List") => match iter.next() {
                Some(dept) => {
                    list_users_department(&users, dept);
                }
                None => {
                    list_users(&users);
                }
            },
            _ => {
                println!("Unknown command");
                break;
            }
        }
        line.clear();
        stdin = std::io::stdin().read_line(&mut line);
    }
}
