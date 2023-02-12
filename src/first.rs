use std::mem;

pub struct List {
    head: Link,
}
enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty,
        }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node: Box<Node> = Box::new(Node {
            elem,
            // We use this trick to replace self.head temporarily with Empty
            // So the ownership can be given to "next".
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            },
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        // The condition in while `let ...`  means loop until you have no 
        // i.e. loop until we hit an Empty matching for Link enum
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

// Tests
// a module is an "inline whole new file"
#[cfg(test)] // Tell compiler to compile tests only when we run tests!
mod tests {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check if empty pop case works!
        assert_eq!(list.pop(), None);
        
        // fill in
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
