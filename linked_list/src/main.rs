/// An attempt to code the infamous linked list in Rust using Enum
enum node {
    item(Box<List>),
    NULL
}

struct List {
    data: i32,
    next: node
}

impl List {
    fn add_node_tail(mut self, data: i32) -> List {
        loop {
            if let node::item(x) = self.next {
                self = *x;
            } else {
                // Add element to the end here
                self.next = node::item(Box::new(List {
                    data,
                    next: node::NULL
                }));
            }
        }
        self
    }

    fn add_node_head(mut self, data: i32) -> List {
        List {
            data,
            next: node::item(Box::new(self))
        }
    }

    fn print_list(mut self) -> List {
        loop {
            if let node::item(x) = self.next {
                println!("{}", x.data);
                self = *x;
            } else {
                break;
            }
        }
        self
    }
}

fn main() {
    // Create a new list
    let list = List { data: 0, next: node::item(Box::new(List{
        data: 1,
        next: node::NULL
    })) };
    let list = list.add_node_head(2);
    let list = list.add_node_head(3);
    let list = list.add_node_head(4);
    let list = list.add_node_head(5);
    let list = list.print_list();
    let list = list.print_list();
}
