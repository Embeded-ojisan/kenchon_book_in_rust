use kenchon_book_in_rust::sort::InsertionSort::*;
use kenchon_book_in_rust::utility::stdio_wrapper::*;

pub fn main()
{
    let mut v = Vec::new();
 
    println!("Type a number of elements");
    let N = read_single_usize();

    for i in 0..N
    {
        let a = read_single_usize();
        println!("The {}st element is {}", i, a);
        v.push(a);
    }


    let n = v.len();
    for i in &v
    {
        println!("The array is {}", i);
    }


    InsertionSort(&mut v);

    let n = v.len();
    for i in &v
    {
        println!("The sorted array is {}", i);
    }
}