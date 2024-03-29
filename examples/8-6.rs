use std::collections::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    list.push_back("yamamoto");
    list.push_back("watanabe");
    list.push_back("ito");
    list.push_back("takahashi");
    list.push_back("suzuki");
    list.push_back("sato");
/*
    let list = LinkedList::from(
        "yamamoto",
        "watanabe",
        "ito",
        "takahashi",
        "suzuki",
        "sato"
    );
*/

    for v in list.iter() {
        println!("{}", v);
    }

    let erase_string = "watanabe".to_string();
    let mut count = 0;

    for v in list.iter() {
        match &*v.to_string() {
            "watanabe" => {
                break;
            }
            _ => {
                count += 1;
            }
        }
    }

/*
    list.remove(count);

    for v in list.iter() {
        println!("{}", v);
    }    
*/
}