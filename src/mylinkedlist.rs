use crate::mylinkedlist::MyLinkedList::Nil;

pub enum MyLinkedList<T>{
    Some(Box<Node<T>>),
    Nil,
}

pub struct Node<T> {
    next: Box<Node<T>>,
    data: T,
}


impl<T> MyLinkedList<T> {
    pub fn new() -> MyLinkedList<T> {
        Nil
    }

/*
    fn len(&self) -> usize {

    }
*/

    pub fn insert(&mut self, data: T) {
        match self {
            Nil => {

            },
            _ => {
/*
                let old_node = self.unwrap();
                let new_node = Box::new(
                    Node {
                        next: old_node.next,
                        data: data,
                    }
                );

                s.next = new_node;
*/
            },
        }
    }

/*
    fn erase(&mut self, data: T) -> {

    }
*/

/*
    fn print(&mut self) -> {

    }
*/
}
