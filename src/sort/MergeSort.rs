
pub fn MergeSort(a: &mut Vec<usize>, left: usize, right: usize)
{

    if (right - left) == 1
    {
        dbg!();
        return;
    }

    let mid = left + (right - left)/2;
    
    MergeSort(a, left, mid);

    MergeSort(a, mid, right);

    let mut buf = Vec::new();

    for i in left..mid
    {
        buf.push(a[i]);
    }

    for i in (mid..right).rev()
    {
        buf.push(a[i]);
    }

    let mut index_left = 0;
    let mut index_right = buf.len() - 1;

    for i in left..right
    {
        if buf[index_left] <= buf[index_right]
        {
            a[i] = buf[index_left];
            index_left += 1;
        }
        else
        {
            a[i] = buf[index_right];
            index_right -= 1;
        }
    }
}