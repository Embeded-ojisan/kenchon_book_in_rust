
pub fn InsertionSort(a: &mut Vec<usize>)
{
    let mut N = a.len();
    for (i, val) in a.iter_mut().enumerate()
    {
        let v = val;
        let j = i;
        loop
        {
            if j <= 0
            {
                break;
            }

            if a[j - 1] > *val
            {
                let aj = a[j - 1];
                a[j] = aj;
            }
            else
            {
                break;
            }
        }
        a[j] = *v;
        j -= 1;
    }
}