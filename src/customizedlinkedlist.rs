/*
以下を丸々コピーしたうえでカスタマイズ

http://www.nct9.ne.jp/m_hiroi/linux/rust04.html
*/

use std::rc::Rc;
use std::cell::RefCell;

// ノードの定義
struct Node<T> {
    data: T,
    prev: Option<Rc<RefCell<Node<T>>>>,
    next: Option<Rc<RefCell<Node<T>>>>
}

// 連結リストの定義
pub struct List<T> {
    head: Option<Rc<RefCell<Node<T>>>>
}

// イテレータの定義
pub struct IterList<'a, T: 'a> {
    next: Option<&'a Node<T>>
}

pub struct IterMutList<'a, T: 'a> {
    next: Option<&'a mut Node<T>>
}

pub struct IterIntoList<T> {
    head: List<T>
}

impl<'a, T> Iterator for IterList<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| { &**node });
            &node.data
        })
    }
}

/*
impl<'a, T> Iterator for IterMutList<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| { &mut **node });
            &mut node.data
        })
    }
}
*/

impl<T> Iterator for IterIntoList<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        self.head.pop()
    }
}

// メソッドの定義
impl<T> List<T> {
    // リストの生成
    pub fn new() -> List<T> {
        List { head: None }
    }

    // データの追加
    pub fn push(&mut self, x: T) {

        let ne = self.head.as_ref();
        match self.head.is_some() {
            true => {
                let new_node = Node { 
                    data: x,
                    prev: None,
                    // 現状の根元の値へ参照させる
                    next: Some(
                        Rc::clone(
                            ne.unwrap()
                        )
                    )
                };
                
                // pushする値をリストの根元へ
                self.head = Some(Rc::new(RefCell::new(new_node)));
//                let reserve = std::mem::replace(&mut self.head, self.head);
//                self.head.as_mut().unwrap().next.as_mut().unwrap().prev = self.head.take();
                if self.head.as_ref().expect("Error1").next.as_ref().is_some()
                {
                    self.head
                    .as_ref()
                    .expect("Error2")
                    .next
                    .as_ref()
                    .expect("Erro3")
                    .prev = self.head.take();
                }
                else
                {
                    ;
                }
            },
            false => {
                let new_node = Node { 
                    data: x,
                    prev: None,
                    next: None 
                };
                self.head = Some(Rc::new(new_node));
            }
        }
    }

    // データの取り出し
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|xs| {
            let xs = *xs;
            self.head = xs.next;
            xs.data
        })
    }

    // 先頭データへの参照
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|xs| {
            &xs.data
        })
    }

    // リストは空か？
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    // イテレータの生成
    pub fn iter(&self) -> IterList<T> {
        IterList { next: self.head.as_ref().map(|node| &**node )}
    }
    pub fn iter_mut(&mut self) -> IterMutList<T> {
        IterMutList { next: self.head.as_mut().map(|node| &mut **node )}
    }
    pub fn into_iter(self) -> IterIntoList<T> {
        IterIntoList { head: self }
    }
}

impl<T: std::fmt::Display> List<T> {

    // リストの中身の出力
    pub fn print(&self) {
        for x in self.iter() {
            println!("{} ", x);
        }
    }
}

impl<T: std::cmp::PartialEq> List<T> {

    // 指定したデータの削除
    pub fn erase(&mut self, erase_value: &T) {
        for x in self.iter() {
            if *erase_value == *x
            {
                ;
            }
        }    
    }
}
