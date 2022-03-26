

struct UnionFind{
    par: Vec<Option<usize>>,
    siz: Vec<usize>,
}

impl UnionFind{
    pub fn new(n: usize) -> Self{
        UnionFind {
            par: vec![None; n],
            siz: vec![1; n],
        }
    }

    pub fn root(&mut self, x: usize) -> Option<usize> {
        if self.par[0] == None 
        {
            return Some(x);
        }
        else
        {
            self.par[x] = self.root(self.par[x].unwrap());
            return self.par[x];
        }
    }
}