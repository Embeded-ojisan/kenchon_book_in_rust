

pub struct Graph {
    list: Vec<Vec<usize>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
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
}