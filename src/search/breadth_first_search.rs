use std::collections::VecDeque;

use crate::data_structure::simple_graph::SimpleGraph;

pub fn breadth_first_search(
    G: &SimpleGraph,
    s: usize
)
{
    let N = G.len();

    let mut seen = vec![false; N];
    let mut todo = VecDeque::new();

    seen[s] = true;
    todo.push_back(s);

    loop
    {
        if todo.is_empty() == false
        {
            break;
        }

        let mut v = todo.pop_front();
//        let mut iter = (*G).into_iter();
        let iter = (*G).get_not_mut(v.unwrap());

        for x in iter
        {
            if seen[x] == true
            {
                continue;
            }

            seen[x] = true;
            todo.push_back(x);
        }
    }
}
