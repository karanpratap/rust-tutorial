use std::collections::HashMap;
use std::io;

fn main() {
    let mut dept: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut buf = String::new();
        println!("Enter command line (Add/Remove <name> to/from <dept>");
        io::stdin().read_line(&mut buf).expect("Failed to read input");
        let cmd_elem: Vec<&str> = buf.trim().trim_end().split_whitespace().collect();

        match cmd_elem[0] {
            "add" => {
                if cmd_elem.len() != 4 {
                    println!("Invalid command format: Required add <name> to <dept>");
                    continue;
                }

                let entry = dept.entry(cmd_elem[3].to_string()).or_insert({
                    println!("Department {} not found, creating new", cmd_elem[3]);
                    Vec::new()
                });

                // Sorted insert into the vector
                match entry.binary_search(&cmd_elem[1].to_string()) {
                    Ok(_) => {},
                    Err(pos) => entry.insert(pos, cmd_elem[1].to_string())
                }
            }
            "remove" => {
                if cmd_elem.len() != 4 {
                    println!("Invalid command format: Required remove <name> from <dept>");
                    continue;
                }
                let entry = dept.get(&cmd_elem[3].to_string());
                match entry {
                    Some(v) => {
                        match v.binary_search(&cmd_elem[1].to_string()) {
                            Ok(pos) => {
                                let elem = dept.entry(cmd_elem[3].to_string()).or_insert(Vec::new());
                                elem.remove(pos);
                                println!("Removed employee {} from department {}", cmd_elem[1], cmd_elem[3]);
                            }
                            Err(_) => println!("Employee {} not found in department {}", cmd_elem[1], cmd_elem[3])                       }
                    }
                    None => println!("Department {} not found...", cmd_elem[3])
                }
            }
            "list" => {
                if cmd_elem.len() >= 2 {
                    // Get by department
                    let elem = dept.get(cmd_elem[1]);
                    match elem {
                        Some(vec) => {
                            println!("Employees in department {}:", cmd_elem[1]);
                            println!("{:?}", vec);
                        }
                        None => println!("Department {} not found", cmd_elem[1])
                    };
                } else {
                    println!("Listing all employees by department");
                    for (k, v) in &dept {
                        println!("[Department {}]: {:?}", k, v);
                    }
                }
            },
            "quit" => break,
            _ => println!("Invalid command: {}", cmd_elem[0])
        };
    }



    // dept.entry
}
