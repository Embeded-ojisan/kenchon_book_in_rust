use kenchon_book_in_rust::stdio_wrapper::read_single_usize;
use kenchon_book_in_rust::SimpleGraph::*;

fn main() {

    println!("Type vertex!");
    let vertex = read_single_usize();

    println!("Type edge number!");
    let edge = read_single_usize();

    let mut graph = SimpleGraph::new();

    for i in 0..edge {
        // 標準入力から値を受け取り
        let a = read_single_usize();
        let b = read_single_usize();

        // グラフへ値を追加
        graph.add(a, b);

        println!("{} {}", a, b);

    }

    for i in 0..edge {
        let v = graph.get(i);
        for j in v {
            println!("{}", j);
        }
    }
}