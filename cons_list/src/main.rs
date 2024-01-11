/// An attempt to code the infamous linked list in Rust using Enum
enum Node {
    Item(Box<(i32, Node)>),
    NULL
}

use Node::{NULL, Item};

impl Node {
    fn add_node_tail(&mut self, data: i32) {
        let list = self;
        // NULL check
        if let NULL = list {
            *list = Item(Box::new((data, NULL)));
            return;
        } else if let Item(x) = list {
            x.1.add_node_tail(data);
        }
    }
    
    fn add_node_head(self, data: i32) -> Self {
        Item(Box::new((data, self)))
    }

    fn remove_node_head(self) -> Self {
        if let Item(x) = self {
            return x.1;
        }
        self
    }

    fn remove_node_index(self, index: usize) -> Self {
        // Base case
        if index == 0 {
            if let Item(x) = self {
                return x.1;
            } else {
                println!("Invalid index");
                return NULL;
            }
        } else {
            if let Item(mut x) = self {
                x.1 = x.1.remove_node_index(index - 1);
                return Node::Item(x);
            } else {
                println!("Invalid index");
                return NULL;
            }
        }
    }

    fn add_node_index(self, index: usize, data: i32) -> Self {
        // Base case
        if index == 0 {
            return Item(Box::new((data, self)));
        } else {
            if let Item(mut x) = self {
                x.1 = x.1.add_node_index(index - 1, data);
                return Item(x);
            } else {
                println!("Invalid index");
                return NULL;
            }
        }
    }

    // Function to print the contents of the list without
    // consuming it
    fn print_list(&self) {
        print!("HEAD->");
        let mut list = self;
        loop {
            if let Item(x) = list {
                print!("{}->", x.0);
                list = &x.1;
            } else {
                break;
            }
        }
        println!("REAR");
    }
}

fn main() {
    // Create a new list
    let mut list = NULL;
    list.print_list();
    list = list.add_node_head(1);
    list.add_node_tail(2);
    list.add_node_tail(3);
    list.add_node_tail(4);
    list.add_node_tail(5);
    list.print_list();
    list = list.remove_node_index(1);
    list.print_list();
    list = list.add_node_index(2, 20);
    list.print_list();
    list = list.remove_node_index(2);
    list.print_list();
    list = list.remove_node_head();
    list.print_list();
    list = list.remove_node_index(0);
    list.print_list();
}
