
pub fn QuickSort(a: &mut Vec, left: usize, right: usize)
{
    if right - left <= 1
    {
        return;
    }

    let mut pivot_index = (left + right)/2;
    let mut pivot = a[pivot_index]:
    std::mem::replace(a[pivot_index], a[right - 1]);

    let mut i = left;
    for j in left..(right - 1)
    {
        if a[j] < pivot
        {
            std::mem::replace(a[i], a[j]);
            i += 1;
        }
        ;
    }
    std::mem::replace(a[i], a[right - 1]);

    QuickSort(a, left, i);
    QuickSort(a, i + 1, right);
}