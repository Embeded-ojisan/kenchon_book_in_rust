

struct UnionFind
{
    par: Vec<Option<usize>>,
    siz: Vec<usize>,
}

pub enum UniteError
{
    AlreadySameGroup,
}

impl UnionFind
{
    pub fn new(n: usize) -> Self
    {
        UnionFind
        {
            par: vec![None; n],
            siz: vec![1; n],
        }
    }

    pub fn root(&mut self, x: usize) -> Option<usize>
    {
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

    pub fn issame(&mut self, x: usize, y: usize) -> bool
    {
        // Option型がEqトレートに対応しているため
        return self.root(x) == self.root(y);     
    }

    pub fn unite(&mut self, x: usize, y: usize) -> Result<,UniteError>
    {
        let x = self.root(x);
        let y = self.root(y);

        if x == y
        {
            return Error(UnionError);
        }

        return Ok();
    }
}