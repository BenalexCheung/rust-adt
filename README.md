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

### 排序算法

### 动态规划

