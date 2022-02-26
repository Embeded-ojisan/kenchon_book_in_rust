use kenchon_book_in_rust::samplelinkedlist::List;

fn main() {
    let mut a: List<i32> = List::new();
    for i in 0 .. 8 {
        a.push(i);
    }
    println!("{}", a.peek().unwrap());
    for x in a.iter() {
        print!("{} ", x)
    }
    println!("");
    for x in a.iter_mut() {
        *x += 10;
    }
    println!("{}", a.iter().nth(7).unwrap());
    *a.iter_mut().nth(7).unwrap() += 100;
    println!("{}", a.iter().nth(7).unwrap());
    while !a.is_empty() {
        println!("{}", a.pop().unwrap());
    }
    for i in 0 .. 8 {
        a.push(i)
    }
    let b: Vec<i32> = a.into_iter().collect();
    println!("{:?}", b);
}