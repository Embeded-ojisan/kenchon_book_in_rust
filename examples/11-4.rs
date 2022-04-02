use kenchon_book_in_rust::utility::stdio_wrapper::read_single_usize;
use kenchon_book_in_rust::DataStructure::UnionFind::*;

fn main()
{
    println!(" Type a number of union!");
    let N = read_single_usize();

    println!(" Type a number of vertex!");
    let M = read_single_usize();

    let mut uf = UnionFind::new(N);

    for i in 0..M
    {
        println!("Type a xvalue");
        let a = read_single_usize();

        println!("Type a yvalue");
        let b = read_single_usize();

        uf.unite(a, b);
    }

    let mut res = 0;
    for x in 0..N
    {
        if uf.root(x).is_none() != true
            && uf.root(x).unwrap() == x
            {
                res+= 1;
            }
    }

    println!("{}", res);
}