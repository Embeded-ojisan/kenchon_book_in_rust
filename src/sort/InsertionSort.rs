
pub fn InsertionSort(a: &mut Vec<usize>)
{
    let mut N = a.len();
    let mut i = 0;
    for val in a.iter_mut()
    {
        let mut j = i;
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

        let v = val;

        a[j] = *v;
        j -= 1;
        i += 1;
    }
}