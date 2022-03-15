/*
参考

https://qiita.com/penguinshunya/items/cd96803b74635aebefd6
*/
use std::io;

pub fn read_line_single() -> String {
    read::<String>()
}

pub fn read_usize_single() -> usize {
    read::<usize>()
}

fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .parse()
        .ok()
        .unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

fn read_vec2<T: std::str::FromStr>(n: u32) -> Vec<Vec<T>> {
    let mut v2 = Vec::new();
    for _ in 0..n {
        let mut s = String::new();
        std::io::stdin()
            .read_line(&mut s)
            .ok();
        let v = s.trim()
                    .split_whitespace()
                    .map(|e| e.parse().ok().unwrap())
                    .collect();
        v2.push(v);
    }
    v2
}

/*
pub fn read_usize_block() -> usize {
    ;
}
*/