
pub fn InsertionSort(a: &mut Vec<usize>)
{
    let mut N = a.len();
    let mut i = 1;
    loop
    {
        if i >= N
        {
            break;
        }

        let val = a[i];

        let mut j = i;

        loop
        {
            if j <= 0
            {
                break;
            }

            if a[j - 1] > val
            {
                let aj = a[j - 1];
                a[j] = aj;
            }
            else
            {
                break;
            }
            j -= 1;
        }

        let v = val;

        a[j] = v;
        i += 1;
    }
}