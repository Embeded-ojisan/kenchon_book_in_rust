/*
    「高効率言語Rust」のp.490のソースの写経とそれをベースにした自作メソッド
*/
use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub struct Node {
    data: isize,
    next: Option<Rc<RefCell<Node>>>,
    prev: Option<Weak<RefCell<Node>>>,
}

pub struct List {
    head: Option<Rc<RefCell<Node>>>,
    foot: Option<Rc<RefCell<Node>>>,
}

impl List {
    pub fn new() -> Self {
        Self {head: None, foot: None}
    }

    fn new_node(v: isize) -> Rc<RefCell<Node>> {
        Rc::new (
            RefCell::new (
                Node {
                    data: v,
                    next: None,
                    prev: None,
                }
            )
        )
    }

    // 末尾に値を追加
    pub fn push(&mut self, v: isize) {
        let n = List::new_node(v);
        match self.foot.take() {
            None => {
                self.foot = Some(Rc::clone(&n));
                self.head = Some(n);
            },
            Some(old_foot) => {
                self.foot = Some(Rc::clone(&n));
                n.borrow_mut().prev =
                    Some(Rc::downgrade(&old_foot));
                old_foot.borrow_mut().next = Some(n);
            }
        }
    }

    // 先頭に値を追加
    pub fn unshift(&mut self, v: isize) {
        let n = List::new_node(v);
        match self.head.take() {
            None => {
                self.foot = Some(Rc::clone(&n));
                self.head = Some(n);
            },
            Some(old_head) => {
                old_head.borrow_mut().prev = 
                    Some(Rc::downgrade(&old_head));
                n.borrow_mut().next = Some(old_head);
                self.head = Some(n);
            }
        }
    }

    pub fn erase(&mut self, v: isize) {
        match &self.head {
            None => {
                ;
            },
            Some(head) => {
                loop {
                    if head.borrow_mut().data == v {
                        let erase_point = head.borrow_mut();
                        match &erase_point.next {
                            None => { 
                                ;
                            },
                            Some(ep) => {
                                ep.borrow_mut().prev.unwrap() = erase_point.prev;
                            }
                        } 
                        break;
                    }

                    match &head.borrow_mut().next {
                        None => {
                            ;
                        },
                        Some(next) => {
                            let head = &head.borrow().next;
                        }
                    }
                }
            }
        }
    }

    pub fn iter(&mut self) -> ListIter {
        match &self.head {
            None => ListIter{cur: None},
            Some(head) => {
                let head = Rc::clone(&head);
                ListIter{cur: Some(head)}
            },
        }
    }
}

pub struct ListIter {
    pub cur: Option<Rc<RefCell<Node>>>,
}

impl Iterator for ListIter {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        match self.cur.take() {
            None => None,
            Some(cur) => {
                let cb = cur.borrow();
                let data = cb.data;
                match &cb.next {
                    None => self.cur = None,
                    Some(next) => {
                        self.cur = Some(Rc::clone(&next));
                    }
                }
                Some(data)
            }
        }
    }
}