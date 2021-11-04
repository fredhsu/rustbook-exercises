use std::collections::HashMap;

fn add_user(map: &mut HashMap<String,String>, name: String, department: String) {
    map.insert(name, department);
}

fn list_users(map: &HashMap<String,String>) {
    for (name, department) in map.iter() {
        println!("{} => {}", name, department);
    }
}

fn list_users_department(map: &HashMap<String,String>, department: &str) {
    for (name, dpt) in map {
        if dpt == department {
            println!("{} works in the {} department", name, dpt);
        }
    }
}

fn main() {
    let mut map = HashMap::new();
    add_user(&mut map, "John".to_string(), "IT".to_string());
    add_user(&mut map, "Mary".to_string(), "HR".to_string());
    list_users(&map);
    list_users_department(&map, "IT");
}
