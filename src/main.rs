mod sort_lib {
    /// 冒泡排序
    /// 1. 比较相邻的两个元素，如果第一个元素比第二个元素大，就交换位置
    /// 2. 对每一对相邻元素执行步骤1，这样最后的元素是最大的数
    /// 3. 重复步骤1~2，直到排序完成
    pub fn bubble_sort(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        for i in 0..nums.len() {
            // 标记每轮遍历中是否发生元素交换
            let mut flag = false;

            // 比较相邻元素，同时标记有交换发生
            for j in 0..nums.len() - i - 1 {
                if nums[j] > nums[j + 1] {
                    let tmp = nums[j];
                    nums[j] = nums[j + 1];
                    nums[j + 1] = tmp;
                    flag = true;
                }
            }

            println!("{:?}", nums);

            // 判断是否有数据交换，若没有则提前推出
            if !flag {
                break;
            }
        }
        nums
    }

    /// 插入排序
    /// 1. 第一个元素被视为已排序
    /// 2. 取下一个元素，在已排序序列中从后向前扫描
    /// 3. 如果已排序元素大于新元素，将该已排序元素右移一个位置
    /// 4. 重复步骤3，直到找到已排序元素小于或者等于新元素的元素，将新元素插入到该元素之后
    /// 5. 重复步骤2~4，直到未排序序列中的元素为空
    pub fn insertion_sort(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        for i in 1..nums.len() {
            // 开始外循环，用current保存当前i指向的未排序元素
            let current = nums[i];
            // 开始内循环，用当前j指向的已排序元素和current比较
            let mut j = (i - 1) as i32;
            while j >= 0 {
                // 若已排序元素大于未排序元素，则将已排序元素右移一位
                if nums[j as usize] > current {
                    // 移动元素
                    nums[(j + 1) as usize] = nums[j as usize];
                } else {
                    // 结束内循环，j+1指向的位置就是current应该插入的位置
                    break;
                }
                j -= 1;
            }

            nums[(j + 1) as usize] = current;
            println!("{:?}", nums);
        }
        nums
    }

    /// 选择排序
    /// 1. 数组nums初始状态：已排序序列为空，未排序序列区间为[0,n)
    /// 2. 在未排序序列区间[0,n)中找到最小元素nums[min_index]，将其与nums[0]交换元素
    /// 3. 重复步骤2，在未排序序列区间[i,n)中找到最小元素nums[min_index]，将其与nums[i]交换元素
    /// 4. 此时，数组nums的已排序序列区间为[0,i]，未排序序列区间为[i+1,n)
    /// 5. 直到i=n-1，排序结束，此时已排序序列区间为[0,n)，未排序序列为空
    pub fn selection_sort(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }

        for i in 0..nums.len() - 1 {
            let mut min_index = i;

            for j in i + 1..nums.len() {
                if nums[j] < nums[min_index] {
                    min_index = j;
                }
            }

            if i != min_index {
                nums.swap(i, min_index);
            }
            println!("{:?}", nums);
        }
        nums
    }

    /// 堆排序
    /// 1. 将待排序数组nums构建成大顶堆
    /// 2. 将堆顶元素nums[0]与最后一个元素nums[n-1]交换
    /// 此时数组分为两个序列，一个是无序序列[0,n-2]，一个是有序序列[n-1]，且nums[n-1]为当前数组最大值
    /// 3. 上述操作完成后，数组已大不满足大顶堆的特性，需要将区间为[0,n-2]的无序序列重新构建成堆
    /// 然后将nums[0]与无序序列最后一个元素nums[n-2]交换，得到新的无序序列[0,n-3]和新的有序序列[n-2,n-1]
    /// 4. 重复步骤3，直到最后堆中只剩下索引为0的元素，排序完成
    pub fn heap_sort(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return vec![];
        }
        build_heap(&mut nums);
        for i in (0..nums.len()).rev() {
            nums.swap(0, i);
            heapify(&mut nums, 0, i);
            println!("{:?}", nums);
        }
        nums
    }

    // 建立大顶堆
    fn build_heap(nums: &mut Vec<i32>) {
        let len = nums.len();
        for i in (0..len / 2).rev() {
            heapify(nums, i, len);
        }
    }

    fn heapify(nums: &mut Vec<i32>, idx: usize, len: usize) {
        let mut idx = idx;
        loop {
            let mut max_pos = idx;
            if 2 * idx + 1 < len && nums[idx] < nums[2 * idx + 1] {
                max_pos = 2 * idx + 1;
            }
            if 2 * idx + 2 < len && nums[max_pos] < nums[2 * idx + 2] {
                max_pos = 2 * idx + 2;
            }
            if max_pos == idx {
                break;
            }
            nums.swap(idx, max_pos);
            idx = max_pos;
        }
    }

    /// 归并排序
    /// 1. 把长度为n的待排序元素拆分成两个长度为n/2的子序列
    /// 2. 针对这两个子序列继续重复步骤1，直到拆分成的子序列中只包含一个元素
    /// 3. 开始排序，排序的方法是按照大小顺序合并两个元素
    /// 4. 重复步骤3，按照大小顺序不断地合并排好序的子序列，直到最终将两个排好序的子序列合并成一个完全有序的序列
    pub fn merge_sort(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return nums;
        }

        let n = nums.len() - 1;
        merge_sort_recursion(&mut nums, 0, n);
        nums
    }

    fn merge_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
        // 判断是否只剩下最后一个元素
        if left >= right {
            return;
        }

        // 从中间将数组分成两个序列
        let middle = left + (right - left) / 2;

        // 分别以递归方式将左右两个序列排好序
        merge_sort_recursion(nums, left, middle);
        merge_sort_recursion(nums, middle + 1, right);

        // 将已有序的两个序列合并
        merge(nums, left, middle, right);
    }

    fn merge(nums: &mut Vec<i32>, left: usize, middle: usize, right: usize) {
        // 定义索引i表示左序列的起始位置
        let mut i = left;

        // 定义索引j表示右序列的起始位置
        let mut j = middle + 1;

        // 定义索引k表示开始排序原数组的位置
        let mut k = left;

        // 定义用于排序过程中临时存放元素的数组
        let mut tmp = vec![];

        while k <= right {
            if i > middle {
                // 左序列元素处理完毕，剩下右序列元素，将右序列元素逐个添加
                tmp.push(nums[j]);
                j += 1;
                k += 1;
            } else if j > right {
                // 右序列元素处理完毕，剩下左序列元素，将左序列元素逐个添加
                tmp.push(nums[i]);
                i += 1;
                k += 1;
            } else if nums[i] < nums[j] {
                // 左序列元素小于右序列元素，将左序列元素添加，索引i往前移动一位
                tmp.push(nums[i]);
                i += 1;
                k += 1;
            } else {
                // 左序列元素大于等于有序列元素，将右序列元素添加，索引j往前移动一位
                tmp.push(nums[j]);
                j += 1;
                k += 1;
            }
        }

        for i in 0..=(right - left) {
            nums[left + i] = tmp[i];
        }

        println!("{:?}", nums);
    }

    /// 快速排序
    /// 1. 在待排序的序列nums的区间[0,n)中挑出一个元素nums[pivot]作为基准值，pivot即分区点
    /// 2. 所有比基准值小的元素放在区间[0,pivot)中，所有比基准值大的元素放在区间[pivot+1,n)中
    /// 3. 分别在区间[0,pivot)与区间[pivot+1,n)中重复步骤1~2，直到区间只剩一个元素，所有元素排序完成
    pub fn quick_sort(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.is_empty() {
            return nums;
        }
        let len = nums.len();
        quick_sort_recursion(&mut nums, 0, len - 1);
        nums
    }

    fn quick_sort_recursion(nums: &mut Vec<i32>, left: usize, right: usize) {
        // 判断是否只剩下一个元素
        if left >= right {
            return;
        }

        // 使用partition函数找到分区点
        let pivot = partition(nums, left, right);

        // 对分区点左子数组和右子数组进行递归操作
        if pivot != 0 {
            quick_sort_recursion(nums, left, pivot - 1);
        }
        quick_sort_recursion(nums, pivot + 1, right);
    }

    fn partition(nums: &mut Vec<i32>, left: usize, right: usize) -> usize {
        // 设定基准值
        let pivot = right;

        // 遍历数组，每个数都与基准值进行比较，小于基准值的放到索引i指向的位置
        // 遍历完成后，索引i位置之前的所有书都小于基准值
        let mut i = left;
        for j in left..right {
            if nums[j] < nums[pivot] {
                nums.swap(i, j);
                i += 1;
            }
        }

        // 将末尾的基准值交换到索引i的位置，此索引位置之后的所有数都大于基准值
        nums.swap(i, right);
        println!("{:?}", nums);
        // 返回i作为分区点
        i
    }
}

fn main() {
    let nums = vec![7, 6, 12, 11, 6, 3];
    println!("冒泡排序：");
    sort_lib::bubble_sort(nums.clone());
    println!("插入排序：");
    sort_lib::insertion_sort(nums.clone());
    println!("选择排序：");
    sort_lib::selection_sort(nums.clone());
    println!("堆排序：");
    sort_lib::heap_sort(nums.clone());
    println!("归并排序：");
    sort_lib::merge_sort(nums.clone());
    println!("快速排序：");
    sort_lib::quick_sort(nums.clone());
}
