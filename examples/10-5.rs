use kenchon_book_in_rust::Heap::Heap;

fn main() {
    let mut heap = Heap::new();

    heap.push(5);
    heap.push(3);
    heap.push(7);
    heap.push(1);

    println!("{}", heap.top().unwrap());
    heap.top();
    println!("{}", heap.top().unwrap());

    heap.push(11);
    println!("{}", heap.top().unwrap());
}