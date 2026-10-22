# 实验功能总结

重新实现了 sys_get_time, sys_mmap 和 sys_munmap。根据 fork 和 exec 的实现完成了 sys_spawn。最后实现了 stride 调度算法。

# 简答题

stride 算法深入：
实际情况是 p2 先执行，因为 250 + 10 溢出后等于 4，反而小于 255。

当 $\text{priority} \ge 2$ 时，所有进程可以前进最多 $\frac{\text{BigStride}}{2}$。因为起始情况所有进程的 $\text{stride}=0$，而调度器选择 stride 最小的进程运行，因此各进程间 stride 之差不会超过 $\frac{\text{BigStride}}{2}$。

代码实现如下：

```rust
use core::cmp::Ordering;

struct Stride(u64);

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.abs_diff(other.0) > BIG_STRIDE / 2 {
            // 注意 BinaryHeap 是大根堆，应该让 stride 小的 Greater
            self.0.cmp(other.0)
        } else {
            other.0.cmp(self.0)
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

# 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 **以下各位** 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

   _无_

2. 此外，我也参考了 **以下资料** ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

   _无_

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。
   我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。
   我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。
   我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。
   我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
