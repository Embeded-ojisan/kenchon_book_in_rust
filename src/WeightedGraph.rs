use std::fmt;


#[derive(Copy, Clone, Debug)]
pub struct Edge {
    to: usize,
    w: usize,
}

pub struct WeightedGraph {
    list: Vec<Vec<Edge>>,
}

impl Edge {
    pub fn new(inpTo: usize, inpW: usize) -> Self {
        Edge {
            to: inpTo,
            w: inpW
        }
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.to, self.w)
    }
}

impl WeightedGraph {
    pub fn new() ->Self {
        WeightedGraph {
            list: vec![vec![]],
        }
    }

    pub fn add(
        &mut self,
        a: usize,
        b: usize,
        w: usize
    )
    {
        match self.list.get_mut(a) {
            Some(vb) => {
                vb.push(Edge::new(b, w));
            },
            None => {
                self.list.insert(a, vec![Edge::new(b, w)])
            }
        }
    }

    pub fn get(
        &mut self,
        a: usize
    ) -> Vec<Edge> {
        match self.list.get(a) {
            Some(vb) => {
                vb.to_vec()
            },
            None => {
                vec![]
            },
        }
    }
}

