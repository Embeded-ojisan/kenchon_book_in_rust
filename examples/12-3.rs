use kenchon_book_in_rust::sort::QucikSort::*;
use kenchon_book_in_rust::utility::stdio_wrapper::*;

pub fn main()
{
    let N = read_single_usize();

    let mut a = Vec::new();

    for i in 0..N
    {
        a[i] = read_single_usize();
    }

    QuickSort(a, 0, N);
}