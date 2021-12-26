# rust-adt
> 数据结构和算法实践 —— 《Rust编程：入门、实战与进阶》 朱春雷，机械工业出版社

## 数据结构
常用的数据结构包括：数组、栈、队列、哈希表、链表、树

### 数组
**数组**是一种线性表数据结构，用连续的内存空间来存储具有相同类型的数据。

**特点：**
- 连续的内存空间
- 相同类型

#### [移动零](src/move_zeroes)
#### [加一](src/plus_one)
#### [删除排序数组中的重复项](src/remove_duplicates_from_sorted_array)

### 栈与队列
**栈**是一种操作受限的线性表数据结构，所有的操作都在栈顶完成。

- 顺序栈：数组实现
- 链式栈：链表实现

**应用场景**：在解决某个问题时，只关心最近一次的操作，并且在操作完成之后需要向前查找到更前一次的操作。

**队列**也是一种操作受限的线性表数据结构，只允许在队列尾部压入数据，在队列头部弹出数据。

- 顺序队列：数组实现
- 链式队列：链表实现

**应用场景**：需要按照一定的顺序来处理数据，并且数据的数量在不断变化。

**双端队列**，一种特殊的队列，它允许在队列的头尾两端都能进行数据的插入和删除操作。

**应用场景**：实现一个长度动态变化的窗口或者连续区间

#### [最小栈](src/min_stack)
#### [有效的括号](src/valid_parentheses)
#### [滑动窗口最大值](src/sliding_window_maximum)

### 哈希表

**哈希表**（散列表）是根据键直接进行访问的数据结构。将键转化为数组索引的映射函数叫做哈希函数（散列函数），将哈希函数计算得到的值叫做哈希值（散列值）。

解决哈希碰撞的方法：

- 链表法
- 开放定址法
- 再哈希法
- 建立公共溢出区

#### [两数之和](src/two_sum)
#### [有效的字母异位词](src/valid_anagram)
#### [字母异位词分组](src/group_anagram)

### 链表

**链表**是由链表节点组成的。节点包含两个信息：一个是数据信息，用来存储数据，也叫数据域；另一个是地址信息，用来存储下一个节点地址，也叫做指针域。

- 头节点
- 尾节点
- 后继节点
- 前驱节点

#### [反转链表](src/reverse_linked_list)
#### [链表的中间节点](src/middle_of_the_linked_list)
#### [合并两个有序链表](src/merge_two_sorted_list)
#### [删除链表的倒数第n个节点](src/remove_nth_node_from_end_of_list)

### 树

**树**中的元素叫作节点，相邻节点的关系叫作父子关系，父节点相同的节点叫作兄弟节点，没有父节点的节点叫作根节点，没有子节点的节点叫作叶子节点。

- 二叉树：每个节点最多有两个子节点
- 满二叉树：一个层数为k的满二叉树的叶子结点个数为：2^(k-1)
- 完全二叉树：叶子节点都在最底下两层，最后一层的叶子节点都靠左排列，且从根节点到倒数第二层是一棵满二叉树

树的遍历：
- 前序遍历
- 中序遍历
- 后序遍历

**二叉搜索树**：在树中的任意一个节点，其左子树中的每个节点的值都要小于这个节点的值，而右子树节点的值都大于这个节点的值。

**堆**：是一棵完全二叉树，且堆中每个节点的值必须大于等于（或小于等于）其子树中每个节点的值。堆适合用数组来存储。

#### [二叉树的前序遍历](src/binary_tree_traversal)
#### [二叉树的中序遍历](src/binary_tree_traversal)
#### [二叉树的后序遍历](src/binary_tree_traversal)
#### [二叉树的层序遍历](src/binary_tree_traversal)
#### [二叉搜索树的插入操作](src/insert_into_a_binary_search_tree)

## 算法

常用的一些算法，如递归、分治、回溯、二分查找、深度与广度优先搜索、各种排序和动态规划等

### 递归、分治和回溯

#### 递归

递归算法能解决的问题，要求同时满足一下3个条件：
- 一个问题的解可以分解为几个子问题的解
- 这个问题与分解之后的子问题除了数据规模不同外，求解思路完全一样
- 存在递归终止条件

```Rust
fn recursion(level: T, param: T) {
    // 递归终止条件
    if level > MAX_LEVEL {
        process_result();
        return
    }

    // 处理当前层逻辑
    process(level, param);

    // 下探到下一层
    recursion(level + 1, new_param);

    // 清理当前层状态
    clear_state();
}
```

*“常见的处理递归问题的思维误区是试图弄清楚整个递归规程”*

#### 分治

分治算法是一种处理问题的思想，能解决的问题一般需要满足以下几个条件：
- 原问题与分解后的子问题具有相同的模式
- 原问题分解后的子问题可以独立求解，子问题之间没有相关性
- 具有分解终止条件，当问题足够小时，可以直接求解
- 可以将子问题合并成原问题，且合并操作的复杂度不能太高

```Rust
fn divide_conquer(problem: T, param1: T, param2: T, ...) {
    // 递归终止条件
    if 满足结束条件 {
        return 求解结果
    }

    // 处理当前层逻辑，将问题拆分为子问题
    data = prepare_data(problem);
    subproblems = split_problem(problem, data);

    // 下探到下一层，求解子问题
    subresult1 = divide_conquer(subproblems[0], p1, ...);
    subresult2 = divide_conquer(subproblems[1], p1, ...);
    subresult3 = divide_conquer(subproblems[2], p1, ...);
    ...

    // 将子问题的结果合并成原问题的解
    result = process_result(subresult1, subresult2, subresult3, ...);

    // 清理当前层状态
    clear_state();
}
```

#### 回溯

回溯算法是采用试错的思想，通常采用递归来实现，利用剪枝操作避免穷举所有可能的情况，提高回溯的效率。回溯算法的核心是，维护走过的路径和当前可以做的选择列表，在递归调用之前做选择，在递归调用之后撤销选择。当触发结束条件时，将路径记入结果集。

```Rust
let mut solution = vec![];
fn backtrack(路径， 选择列表) {
    // 递归终止条件
    if 满足结束条件 {
        solution.push(路径)；
        return;
    }

    for 选择 in 选择列表 {
        // 做选择
        路径.push(选择)；

        // 将该选择从选择列表移除后递归调用
        backtrack(路径， 选择列表);

        // 撤销选择，将该选择重新加入选择列表
        路径.remove(选择);
    }
}
```

#### [pow(x, n)](src/powx_n)
#### [爬楼梯](src/climbing_stairs)
#### [括号生成](src/generate_parentheses)
#### [子集](src/subsets)
#### [组合](src/combinations)
#### [N皇后](src/n_queens)

### 二分查找法

**二分查找法**是一种非常简单、易懂且高效的快速查找算法。场景要求如下：
- 二分查找依赖的是顺序表结构，即数组
- 二分查找针对的是一个有序的数据集合。针对动态变化的数据集合，二分查找将不再适用

```Rust
let mut left = 0; // 数组第一个元素的索引
let mut right = nums.len() - 1; // 数组最后一个元素的索引

while left <= right {
    // 选取数组的中间元素索引
    let mid = left + (right - left) / 2; // (left + right)/2 可能会导致溢出！
    if nums[mid] == target {
        return mid;
    } else if nums[mid] < target {
        // 中间元素小于目标值，将待查找的区间调整为右部分
        left = mid + 1;
    } else {
        // 中间元素大于目标值，将待查找的区间调整为左部分
        right = mid - 1;
    }
}
```

#### [搜索旋转排序数组)](src/search_in_rotated_sorted_array)
#### [寻找旋转排序数组中的最小值](src/find_minimum_in_rotated_sorted_array)
#### [有效的完全平方数](src/valid_perfect_square)

### 深度/广度优先搜索

**深度优先搜索**（Depth-First-Search, DFS）解决的是连通性的问题，判断是否有一条路径能从起点连接到终点。这里的起点和终点也可以是某种起始状态和最终状态。

#### 递归实现

```Rust
let mut visted = Vec::new();
fn dfs(node, visted) {
    // 终止条件：已经访问过当前节点
    if visited.contains(node) {return;}

    // 将当前节点加入visited
    visited.push(node);

    // 处理当前节点
    process(node);

    // 获得当前节点的子节点并递归执行
    let child_nodes = generate_child_nodes(node);
    for child_node in child_nodes {
        if !visited.contains(child_node) {
            dfs(child_node, visited);
        }
    }
}
```

#### 非递归实现

```Rust
fn dfs(root) {
    // 终止条件：已经访问过当前节点
    if root.is_none() {return;}

    let mut visited = Vec::new();
    let mut stack = Vec::new();
    stack.push(root);

    while !stack.is_empty() {
        let node = stack.pop();

        // 将当前节点加入visited
        visited.push(node);
        
        // 处理当前节点
        process(node);
    }

    // 获得当前节点的关联节点并加入stack
    let related_nodes = generate_related_nodes(node);
    for related_node in related_nodes {
        stack.push(related_node);
    }
}
```

**广度优先搜索**（Breadth-First-Search, BFS）是一种地毯式层层推进的搜索策略。

```Rust
fn bfs(graph, start, end) {
    let mut visited = Vec::new();
    let mut deque = VecDeque::new();
    deque.push_back(start);

    while !deque.is_empty() {
        let node = stack.pop_front();

        // 将当前节点加入visited
        visited.push(node);
        
        // 处理当前节点
        process(node);
    }

    // 获得当前节点的关联节点并加入stack
    let related_nodes = generate_related_nodes(node);
    for related_node in related_nodes {
        deque.push_back(related_node);
    }
}
```

#### [二叉树的最大深度](src/maximum_depth_of_binary_tree)
#### [二叉树的最小深度](src/mimimum_depth_of_binary_tree)
#### [二叉搜索树中的搜索](src/search_in_a_binary_search_tree)

### 排序算法

常见的排序算法可以分为两大类：

1）比较类排序：通过比较来决定元素间的相对次序，由于其时间复杂度不能突破O(n logn)，因此也称为非线性时间比较类排序
- 交换排序：冒泡排序、\*快速排序
- 插入排序：简单插入排序、\*希尔排序
- 选择排序：\*简单选择排序、\*堆排序
- 归并排序：二路归并排序、多路归并排序

2）非比较类排序：不通过比较来决定元素间的相对次序，可以突破基于比较排序的时间下界，以线性时间运行
- 计数排序
- 桶排序
- 基数排序

**冒泡排序**
```Rust
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
```

**插入排序**
```Rust
fn insertion_sort(mut nums: Vec<i32>) -> Vec<i32> {
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
```

**选择排序**
```Rust
fn selection_sort(mut nums: Vec<i32>) -> Vec<i32> {
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
```

**堆排序**
```Rust
fn heap_sort(mut nums: Vec<i32>) -> Vec<i32> {
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
```

**归并排序**
```Rust
fn merge_sort(mut nums: Vec<i32>) -> Vec<i32> {
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
```

**快速排序**
```Rust
fn quick_sort(mut nums: Vec<i32>) -> Vec<i32> {
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
```

#### [数组中的第k个最大元素](src/kth_largest_element_in_an_array)
#### [合并区间](src/merge_intervals)
#### [翻转对](src/reverse_pairs)

### 动态规划

