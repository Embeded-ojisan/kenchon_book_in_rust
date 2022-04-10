use kenchon_book_in_rust::sort::MergeSort::*;
use kenchon_book_in_rust::utility::stdio_wrapper::*;

pub fn main()
{
    let N = read_single_usize();

    let mut a = Vec::new();

    for i in 0..N
    {
        a.push(read_single_usize());
    }

    println!("Display all the values");
    for i in 0..N
    {
        dbg!(a[i]);
    }

    MergeSort(&mut a, 0, N);

    for i in 0..N
    {
        dbg!(a[i]);
    }
}