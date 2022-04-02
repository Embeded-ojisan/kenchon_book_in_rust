use kenchon_book_in_rust::DataStructure::UnionFind::*;

fn main()
{
    let mut uf = UnionFind::new(7);

    uf.unite(1,2);
    uf.unite(2,3);
    uf.unite(5,6);
    
    println!(
        "uf.issame(1,3) is {}",
        uf.issame(1,3)
    );
    println!(
        "uf.issame(2,5) is {}",
        uf.issame(2,5)
    );

    uf.unite(1,6);
    println!(
        "uf.issame(2,5) is {}",
        uf.issame(2,5)
    );
}