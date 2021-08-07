use std::cmp::PartialOrd;
use std::marker::Sized;

pub fn bubble_sort<E>(arr: &mut [E])
where
    E: PartialOrd + Sized,
{
    let len = arr.len();
    let mut i = 0;
    while i < len {
        let mut swapped_count = 0;
        let mut j = 0;
        while j < len - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped_count += 1;
            }
            j = j + 1;
        }

        if swapped_count == 0 {
            break;
        }
        i = i + 1;
    }
}
