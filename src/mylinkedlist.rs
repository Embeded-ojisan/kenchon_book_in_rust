/*
ベースは以下

https://rust-unofficial.github.io/too-many-lists/second-final.html

http://www.nct9.ne.jp/m_hiroi/linux/rust04.html
*/

pub struct MyLinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    elem: T,
    next: Link<T>,
}


impl<T> MyLinkedList<T> {

    pub fn new() -> Self {
        MyLinkedList {
            head: None,
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(
            Node {
                elem: elem,
                next: self.head.take()
            }
        );

        self.head = Some(new_node);
    }

/*
    pub fn pop(&mut self) -> Link<T> {
        let node = self.head;
        self.head = self.head.take().next;
        node
    }
*/

/*
    fn len(&self) -> usize {

    }
*/
    
/*

    pub fn insert(&mut self, data: T) {
        match self {
            Nil => {

            },
            _ => {
                let old_node = self.unwrap();
                let new_node = Box::new(
                    Node {
                        next: old_node.next,
                        data: data,
                    }
                );

                s.next = new_node;
            },
        }
    }
*/
    
/*
    fn erase(&mut self, data: T) -> {

    }
*/

/*
    fn print(&mut self) -> {

    }
*/
}
