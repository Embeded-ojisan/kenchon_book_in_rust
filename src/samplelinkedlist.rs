/*
以下を丸々コピー

http://www.nct9.ne.jp/m_hiroi/linux/rust04.html
*/

// ノードの定義
struct Node<T> {
    car: T,
    cdr: Option<Box<Node<T>>>
}

// 連結リストの定義
pub struct List<T> {
    head: Option<Box<Node<T>>>
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
            self.next = node.cdr.as_ref().map(|node| { &**node });
            &node.car
        })
    }
}

impl<'a, T> Iterator for IterMutList<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<&'a mut T> {
        self.next.take().map(|node| {
            self.next = node.cdr.as_mut().map(|node| { &mut **node });
            &mut node.car
        })
    }
}

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
        let new_node = Node { car: x, cdr: self.head.take() };
        self.head = Some(Box::new(new_node));
    }

    // データの取り出し
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|xs| {
            let xs = *xs;
            self.head = xs.cdr;
            xs.car
        })
    }

    // 先頭データへの参照
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|xs| {
            &xs.car
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