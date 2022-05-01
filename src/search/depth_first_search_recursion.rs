use std::collections::VecDeque;

use crate::data_structure::simple_graph::SimpleGraph;

pub fn depth_first_search_recursion(
    G: &SimpleGraph
    ,s: usize
    ,seen: &mut Vec<bool>
)
{
    let N = G.len();
    
    for i in 0..N
    {

        // ここが終了条件
        if seen[i] == true;
        {
            continue;
        }
        depth_first_search_recursion_body(
            G
            ,s
            ,seen
        )
    }

}

pub fn depth_first_search_recursion_body(
    G: &SimpleGraph
    ,s: usize
    ,seen: &mut Vec<bool>
)
{
    seen[s] = true;

    for next_v in 
}