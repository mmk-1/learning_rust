use std::mem;

pub struct List<T> {
    head: Link<T>,
}

// We change this to use Option as its basically that
// enum Link {
//     Empty,
//     More(Box<Node>),
// }
type Link<T> = Option<Box<Node<T>>>; // Type aliases. remember Haskell/typedef?

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node: Box<Node<T>> = Box::new(Node {
            elem,
            // We use this trick to replace self.head temporarily with None
            // So the ownership can be given to "next".
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // Peek returns a *reference* to the top of stack(linked list)
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        // The condition in while `let ...`  means loop until you have no 
        // i.e. loop until we hit an None matching for Link enum
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next;
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
