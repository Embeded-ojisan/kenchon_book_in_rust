use kenchon_book_in_rust::utility::stdio_wrapper::*;
use kenchon_book_in_rust::data_structure::weighted_graph::*;


fn main() {

    println!("Type vertex!");
    let vertex = read_single_usize();

    println!("Type edge number!");
    let edge = read_single_usize();

    let mut graph = WeightedGraph::new();

    for i in 0..edge {
        // 標準入力から値を受け取り
        let a = read_single_usize();
        let b = read_single_usize();
        let w = read_single_usize();

        // グラフへ値を追加
        graph.add(a, b, w);

        println!("{} {}", a, b);

    }
    
    for i in 0..edge {
        let v = graph.get(i);
        for j in v {
            println!("{}", j);
        }
    }
}