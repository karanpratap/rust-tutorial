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

    // fn remove_node_index(&mut self, index: usize) {
    //     let mut curr_index = 0;
    //     let prev_node = NULL;
    //     let list = &self;
    //     loop {
    //         if let Item(x) = list {
    //             if curr_index == index {
    //                 // Remove the element
    //                 if let Item(y) = prev_node {
    //                     y.1 = x.1;
    //                     return;
    //                 } else {
    //                     *self = x.1;
    //                     return;
    //                 }
    //             } else {
    //                 // next!
    //                 curr_index += 1;
    //                 prev_node = Item(*x);
    //                 list = x.1;
    //             }
    //         }
    //     }
    // }

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
    list = list.remove_node_head();
    list.print_list();
}
