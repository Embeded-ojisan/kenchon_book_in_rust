pub enum MyLinkedList<T>{
    Some(Node<T>),
    Nil,
}

pub struct Node<T> {
    next: Box<Node<T>>,
    data: T,
}

/*
impl MyLinkedList<T> {
    fn new() -> MyLinkedList {
        Nil
    }
}
*/