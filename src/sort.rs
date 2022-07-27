// https://old.reddit.com/r/rust/comments/qw18oa/can_i_sort_an_array_at_compile_time/hl05kuj
pub const fn const_bubblesort<const N: usize>(mut array: [usize; N]) -> [usize; N] {
    loop {
        let mut swapped = false;

        let mut index = 1;
        while index < array.len() {
            if array[index - 1] > array[index] {
                let left = array[index - 1];
                let right = array[index];
                array[index - 1] = right;
                array[index] = left;
                swapped = true;
            }

            index += 1;
        }

        if !swapped {
            break;
        }
    }

    array
}
