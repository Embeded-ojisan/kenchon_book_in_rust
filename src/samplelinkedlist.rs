/*
以下を丸々コピー

http://www.nct9.ne.jp/m_hiroi/linux/rust09.html

※注釈付き
*/

use std::rc::Rc;
use std::cell::UnsafeCell;

// 双方向リスト
struct Node<T> {
    data: T,
    prev: Link<T>,
    next: Link<T>
}

pub type Link<T> = Option<Rc<UnsafeCell<Node<T>>>>;

// 両端キュー
pub struct Deque<T: Default> {
    pub head: Link<T>,
    pub size: usize
}

// メソッド
impl<T: Default> Deque<T> {
    pub fn new() -> Deque<T> {
        // ヘッダーセル (ダミー)
        let header = Rc::new(
            UnsafeCell::new(
                Node{ 
                    data: Default::default(),
                    prev: None,
                    next: None
                }
            )
        );

        // 
        unsafe {
            (*header.get()).prev = Some(header.clone());
            (*header.get()).next = Some(header.clone());
        };

        Deque { 
            head: Some(header),
            size: 0
        }
    }

    // 末尾にデータを追加する
    pub fn push_back(&mut self, x: T) {
        let new_node = Rc::new(
            UnsafeCell::new(
                Node {
                    data: x,
                    prev: None,
                    next: None
                }
            )
        );
        let p = new_node.get();
        self.head.as_mut().map(|header| unsafe {
            let q = header.get();
            (*q).prev.take().map(|tail| {
                // 末尾のNodeのnextを挿入対象のNodeに合わせる
                (*tail.get()).next = Some(new_node.clone());
                // 先頭のNodeのprevを挿入対象のNodeに合わせる
                (*q).prev = Some(new_node);
                (*p).prev = Some(tail);
                (*p).next = Some(header.clone());
            });
        });
        self.size += 1;
    }

    // 先頭データを取り出す
    pub fn pop_front(&mut self) -> Option<T> {
        if self.size == 0 { return None; }
        self.size -= 1;
        self.head.as_mut().map(|header| unsafe {
            let p = header.get();
            (*p).next.take().map(|node| {
                let q = node.get();
                (*q).next.take().map(|node1| {
                    (*node1.get()).prev = Some(header.clone());
                    (*p).next = Some(node1)
                });
                match Rc::try_unwrap(node) {
                    Ok(node) => node.into_inner().data,
                    Err(_) => panic!("pop_front error")
                }
            })
        }).unwrap()
    }

    // 先頭にデータを追加する
    pub fn push_front(&mut self, x: T) {
        let new_node = Rc::new(UnsafeCell::new(Node {
            data: x, prev: None, next: None
        }));
        let p = new_node.get();
        self.head.as_mut().map(|header| unsafe {
            let q = header.get();
            (*q).next.take().map(|top| {
                (*top.get()).prev = Some(new_node.clone());
                (*q).next = Some(new_node);
                (*p).next = Some(top);
                (*p).prev = Some(header.clone());
            });
        });
        self.size += 1;
    }

    // 末尾データを取り出す
    pub fn pop_back(&mut self) -> Option<T> {
        if self.size == 0 { return None; }
        self.size -= 1;
        self.head.as_mut().map(|header| unsafe {
            let p = header.get();
            (*p).prev.take().map(|node| {
                let q = node.get();
                (*q).prev.take().map(|node1| {
                    (*node1.get()).next = Some(header.clone());
                    (*p).prev = Some(node1)
                });
                match Rc::try_unwrap(node) {
                    Ok(node) => node.into_inner().data,
                    Err(_) => panic!("pop_back error")
                }
            })
        }).unwrap()
    }

    // 先頭データの参照を返す
    pub fn front(&self) -> Option<&T> {
        if self.size == 0 { 
            None
        } else {
            self.head.as_ref().map(|header| unsafe {
                (*header.get()).next.as_ref().map(|top| {
                    &((*top.get()).data)
                })
            }).unwrap()
        }
    }

    // 末尾データの参照を返す
    pub fn back(&self) -> Option<&T> {
        if self.size == 0 { 
            None
        } else {
            self.head.as_ref().map(|header| unsafe {
                (*header.get()).prev.as_ref().map(|tail| {
                    &((*tail.get()).data)
                })
            }).unwrap()
        }
    }

    // 要素数を返す
    pub fn len(&self) -> usize { self.size }

    // キューは空か？ 
    pub fn is_empty(&self) -> bool { self.size == 0 }

    // キューは満杯か？ (常に false を返す)
    pub fn is_full(&self) -> bool { false }

    // キューを空にする
    pub fn clear(&mut self) {
        while !self.is_empty() { self.pop_front(); }
    }
}

// Deque が drop されたときは双方向リストも drop する
impl<T: Default> Drop for Deque<T> {
    fn drop(&mut self) {
        self.clear();
        self.head.take().map(|node| unsafe {
            let p = node.get();
            (*p).prev = None;
            (*p).next = None;
        });
    }
}