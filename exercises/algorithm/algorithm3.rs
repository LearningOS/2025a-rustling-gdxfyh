/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

fn sort<T>(array: &mut [T])
	//TODO
where
    T: Ord,
{
    let n = array.len();
    if n <= 1 {
        return; // 空数组或只有一个元素的数组已经是排序好的
    }

    // 冒泡排序的核心逻辑
    for i in 0..n {
        // 标记本轮是否发生了交换，如果没有交换，说明数组已经有序
        let mut swapped = false;
        
        // 每轮排序都会将最大的元素"浮"到末尾，所以每轮可以少比较一个元素
        for j in 0..n - i - 1 {
            if array[j] > array[j + 1] {
                // 交换元素
                array.swap(j, j + 1);
                swapped = true;
            }
        }
        
        // 如果没有发生交换，说明数组已经排序完成，可以提前退出
        if !swapped {
            break;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}