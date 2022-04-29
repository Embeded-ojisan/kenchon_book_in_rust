/*
参考

https://blog-dry.com/entry/2020/01/30/004406
*/

use std::ops::Deref;

pub struct SimpleGraph {
    list: Vec<Vec<usize>>,
}

impl SimpleGraph {
    pub fn new() -> Self {
        SimpleGraph {
            list: vec![vec![]],
        }
    }

    pub fn add(&mut self, a: usize, b: usize) {
        match self.list.get_mut(a) {
            Some(vb) => {
                vb.push(b);
            },
            None => {
                self.list.insert(a, vec![b]);
            }
        };
    }

    pub fn get(&mut self, a: usize) -> Vec<usize> {
        match self.list.get(a){
            Some(vb) => {
                vb.to_vec()
            },
            None => {
                vec![]
            },
        }
    }

    pub fn get_not_mut(& self, a: usize) -> Vec<usize> {
        match self.list.get(a){
            Some(vb) => {
                vb.to_vec()
            },
            None => {
                vec![]
            },
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }
}

/*
impl Iterator for SimpleGraph {
    type Item = Vec<usize>;

    fn next(&mut self, v: usize) -> Option<Self::Item>
    {
        Some(
            (self.list[v].iter().next().unwrap().deref()).to_vec()
        )
//        temp.next()
    }
}
*/