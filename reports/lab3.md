# 实验三报告

# 完成内容

1. 在`task`中添加`spawn`功能，加载`elf`添加任务
  
2. 完善`sys_call`
  
3. 完善调度程序
  

# 问答作业

1. stride 算法原理非常简单，但是有一个比较大的问题。例如两个 stride = 10 的进程，使用 8bit 无符号整形储存 pass， p1.pass = 255, p2.pass = 250，在 p2 执行一个时间片后，理论上下一次应该 p1 执行。
  
  - 实际情况是轮到 p1 执行吗？为什么？
    答：不是 `p2`的 pass 移除变为 `4`，成为下一个选择
2. 我们之前要求进程优先级 >= 2 其实就是为了解决这个问题。可以证明， **在不考虑溢出的情况下** , 在进程优先级全部 >= 2 的情况下，如果严格按照算法执行，那么 PASS_MAX – PASS_MIN <= BigStride / 2。
  
  - 为什么？尝试简单说明（不要求严格证明）。
    答：在执行一段时间后 PASS_MAX - PASS_MIN < BigStride / 2 + n。0 < A <= stride。所以减去一个stride，所以PASS_MAX - PASS_MIN <= BigStride / 2.
    
  - 已知以上结论，**考虑溢出的情况下**，可以为 pass 设计特别的比较器，让 BinaryHeap<Pass> 的 pop 方法能返回真正最小的 Pass。补全下列代码中的 `partial_cmp` 函数，假设两个 Pass 永远不会相等。
    

```rust
use core::cmp::Ordering;

struct Pass(u64);

impl PartialOrd for Pass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // ...
    }
}

impl PartialEq for Pass {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```