use crate::UnionFind::UniteError::*;


pub struct UnionFind
{
    par: Vec<Option<usize>>,
    siz: Vec<usize>,
}

pub enum UniteError
{
    AlreadySameGroup,
    NoneGroup,

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

        if self.par[x] == None 
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
        if self.root(x).is_none() == true 
            || self.root(y).is_none() == true
        {
            return Err(NoneGroup);
        }

        let mut x = self.root(x).unwrap();
        let mut y = self.root(y).unwrap();

        if x == y
        {
            return Err(AlreadySameGroup);
        }

        if self.siz[x] < self.siz[y]
        {
            std::mem::replace(&mut x, y);
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
                Some(self.siz[sr])
            }
            None => {
                None
            }
        }
    }
}