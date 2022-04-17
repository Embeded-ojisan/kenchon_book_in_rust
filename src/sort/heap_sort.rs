
pub fn heapify(
    a: &mut Vec<usize>
    ,i: usize
    ,N: usize
)
{
    let mut child1 = i*2 + 1;

    if child1 >= N
    {
        return;
    }

    if ((child1 + 1) < N)
        &&(a[child1 + 1] > a[child1])
    {
        child1 += 1;
    }

    if a[child1] <= a[i]
    {
        return;
    }

    unsafe
    {
        let p1: *mut usize = &mut a[i];
        let p2: *mut usize = &mut a[child1];

        p1.swap(p2);
    }

    heapify(a, child1, N);
}

pub fn heap_sort(
    a: &mut Vec<usize>
)
{
    let mut N = a.len();

    // N/2は範囲に含まれず
    for i in (0..(N/2)).rev()
    {
        heapify(a, i, N);
    }

    // Nは範囲に含まれず
    for i in (1..(N)).rev()
    {
        unsafe
        {
            let p1: *mut usize = &mut a[0];
            let p2: *mut usize = &mut a[i];

            p1.swap(p2);
        }

        heapify(a, 0, i);
    }
}