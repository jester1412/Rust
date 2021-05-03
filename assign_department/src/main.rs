use std::collections::HashMap;

fn main() {
    let mut name_list = HashMap::new();
    add_department(&mut name_list, String::from("Sirawich"), String::from("System Engineer"));
    add_department(&mut name_list, String::from("Thammanoon"), String::from("System Engineer"));
    add_department(&mut name_list, String::from("Tinapat"), String::from("Project manager"));

    for (key, value) in &name_list {
        println!("{}: {}", key, value);
    }
    let mut sorted: Vec<_> = name_list.iter().collect();
    //println!("{:?}", sorted);
    sorted.sort_by_key(|&(key,_)| key);
    //println!("{:?}", sorted);
    for (key, value) in &sorted {
        println!("{}: {}", key, value);}

    
}


fn add_department(list: &mut HashMap<String,String> ,name: String,department: String){
    list.entry(name).or_insert(department);
}
