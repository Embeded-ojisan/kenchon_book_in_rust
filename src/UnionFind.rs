use crate::UnionFind::UniteError::AlreadySameGroup;


pub struct UnionFind
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

    pub fn unite(&mut self, x: usize, y: usize) -> Result<bool,UniteError>
    {
        let x = self.root(x).unwrap_or(None);
        let y = self.root(y).unwrap_or(None);

        if x == y
        {
            return Err(AlreadySameGroup);
        }

        if self.siz[x] < self.siz[y]
        {
            std::mem::replace(x, y);
        }

        self.par[y] = Some(x);
        self.siz[x] += self.siz[y];

        return Ok(true);
    }

    pub fn size(&mut self, x: usize) -> Option<usize>
    {
        match self.root(x)
        {
            Some(sr) => {
                self.siz[self.root(x)]
            }
            None => {
                None
            }
        }
    }
}