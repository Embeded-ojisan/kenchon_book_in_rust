

pub struct Heap {
    heap: Vec<usize>,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            heap: vec![]
        }
    }

    pub fn print(&mut self) {
        for i in &self.heap {
            println!("{}", i);
        }
    }

    pub fn push(&mut self, inValue: usize) {
        self.heap.push(inValue);
        let mut i = self.heap.len() - 1;
        loop {
            if i <= 0 {
                break;
            }
            let p = (i - 1)/2;
            if self.heap[p] >= inValue {
                break;
            }
            self.heap[i] = self.heap[p];
            i = p;
        }
        self.heap[i] = inValue;
    }

    pub fn top(&mut self) -> Option<usize> {
        if !self.heap.is_empty()
        {
            return Some(self.heap[0]);
        }

        return None;

    }

    pub fn pop(&mut self) -> Option<usize> {
        if self.heap.is_empty()
        {
            return None;
        }
        let x = self.heap.pop().unwrap();
        self.heap.remove(0);
        let mut i = 0;
        loop
        {
            if (i * 2 + 1) > self.heap.len()
            {
                break;
            }

            let mut child1 = i * 2 + 1;
            let mut child2 = i * 2 + 2;

            if child2 < self.heap.len()
                && self.heap[child2] > self.heap[child1]
            {
                child1 = child2;
            }

            if self.heap[child1] <= x
            {
                break;
            }

            self.heap[i] = self.heap[child1];
            i = child1;
        }
        self.heap[i] = x;
        Some(x)
    }
    

}