

struct Heap {
    heap: Vec<usize>,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            heap: vec![]
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


    pub fn get_max_value(&mut self) -> Option<usize> {
        if !(self.heap.is_empty()) {
            Some(self.heap[0])
        }
        else
        {
            None
        }
    }

    pub fn pop(&mut self) {
        if self.heap.is_empty() {
            return;
        }
        let x = self.heap.pop().unwrap();
    }
    

}