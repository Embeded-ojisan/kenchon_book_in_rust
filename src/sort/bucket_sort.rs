

const MAX: usize = 100000;

pub fn bucket_sort(
    a: &mut Vec<usize>
)
{
    let mut N = a.len();

    let mut num = vec![0; MAX];

    for i in 0..N
    {
        num[a[i]] += 1;
    }

    let mut sum = vec![0; MAX];
    for v in 1..N
    {
        let sumv = sum[v - 1];
        sum[v] = sumv + num[v];
    }

    let mut a2 = vec![0; N];
    for i in (0..N).rev()
    {
        dbg!();
        let ai = a[i];
        sum[ai] -= 1;
        a2[sum[ai]] = ai;
    }
    *a = a2;
}