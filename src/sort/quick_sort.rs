
pub fn QuickSort(
    a: &mut Vec<usize>
    ,left: usize
    ,right: usize
)
{
    if right - left <= 1
    {
        return;
    }

    let mut pivot_index = (left + right)/2;
    let mut pivot = a[pivot_index];

    unsafe
    {
        let p1: *mut usize = &mut a[pivot_index];
        let p2: *mut usize = &mut a[right - 1];

        p1.swap(p2);
    }

    let mut i = left;
    for j in left..(right - 1)
    {
        if a[j] < pivot
        {

            unsafe {
                let p1: *mut usize = &mut a[i];
                let p2: *mut usize = &mut a[j];

                p1.swap(p2);                
            }

            i += 1;
        }
        ;
    }

    unsafe
    {
        let p1: *mut usize = &mut a[i];
        let p2: *mut usize = &mut a[right - 1];

        p1.swap(p2);
    }

    QuickSort(a, left, i);
    QuickSort(a, i + 1, right);
}