use kenchon_book_in_rust::sort::heap_sort::*;
use kenchon_book_in_rust::utility::stdio_wrapper::*;

pub fn main()
{
    let N = read_single_usize();

    let mut a = Vec::new();

    for i in 0..N
    {
        a.push(read_single_usize());
    }

    heap_sort(&mut a);

    for i in 0..N
    {
        println!("{}", a[i]);
    }
}