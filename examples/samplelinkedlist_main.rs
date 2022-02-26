use kenchon_book_in_rust::samplelinkedlist::Deque;

fn main() {
    let mut deq = Deque::new();
    println!("{}", deq.len());
    println!("{}", deq.is_empty());
    println!("{}", deq.is_full());
    for i in 0 .. 4 {
        deq.push_back(i);
        deq.push_front(i + 10);
    }
    println!("{}", deq.len());
    println!("{}", deq.is_empty());
    println!("{}", deq.is_full());
    println!("{:?}", deq.back());
    println!("{:?}", deq.front());
    for _ in 0 .. 3 {
        println!("{:?}", deq.pop_back());
        println!("{:?}", deq.pop_front());
    }
    println!("{}", deq.len());
    println!("{}", deq.is_empty());
    println!("{}", deq.is_full());
    deq.clear();
    println!("{}", deq.len());
    println!("{}", deq.is_empty());
    println!("{}", deq.is_full());
}