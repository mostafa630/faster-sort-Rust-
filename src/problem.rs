/// <summary>
/// Sort the given array in ascending order
/// At least, should beat the default sorting algorithm of the Rust sort
/// </summary>
/// <param name="arr"> array to be sorted in ascending order </param>
/// <param name="size"> array size </param>
pub fn giga_sort(arr: &mut Vec<f32>, size: i32) {
    QuickSort(arr, size, 0, (size - 1) as usize);
}

const THRESHOLD: usize = 170;
fn QuickSort(arr: &mut Vec<f32>, size: i32, first: usize, last: usize) {
    if first < last {
        let split = Partition(arr, size, first, last);
        if last - first > THRESHOLD {
            QuickSort(arr, size, first, (split - 1) as usize);
            QuickSort(arr, size, (split + 1) as usize, last);
        } else {
            let mut key: f32 = 0.0;
            let mut j: i32 = 0;
            for i in first + 1..last + 1 {
                key = arr[i];
                j = (i - 1) as i32;
                while j >= first as i32 && arr[j as usize] > key {
                    arr[(j + 1) as usize] = arr[j as usize];
                    j -= 1;
                }
                arr[(j + 1) as usize] = key;
            }
        }
    }
}
fn Partition(arr: &mut Vec<f32>, size: i32, first: usize, last: usize) -> i32 {
    let middle = (first + last) / 2;

    if arr[first] > arr[middle] {
        let temp: f32 = arr[first];
        arr[first] = arr[middle];
        arr[middle] = temp;
    }
    if arr[first] > arr[last] {
        let temp: f32 = arr[first];
        arr[first] = arr[last];
        arr[last] = temp;
    }
    if arr[middle] > arr[last] {
        let temp: f32 = arr[middle];
        arr[middle] = arr[last];
        arr[last] = temp;
    }

    let pivot_value: f32 = arr[middle];
    arr[middle] = arr[first];
    arr[first] = pivot_value;
    let mut left_mark: usize = first + 1;
    let mut right_mark: usize = last;

    while left_mark <= right_mark {
        while left_mark <= right_mark && arr[left_mark] <= pivot_value {
            left_mark += 1;
        }
        while right_mark >= left_mark && arr[right_mark] >= pivot_value {
            right_mark -= 1;
        }
        if left_mark <= right_mark {
            (arr[right_mark], arr[left_mark]) = (arr[left_mark], arr[right_mark]);
        }
    }
    let temp1 = arr[first];
    arr[first] = arr[right_mark];
    arr[right_mark] = temp1;

    return right_mark as i32;
}
