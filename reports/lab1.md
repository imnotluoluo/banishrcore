## 编程作业
1.在TaskControlBlock结构体中添加变量syscall_num，用于计数每一个系统调用被调用次数，并相应实现了TaskManager中的更新函数、查询函数和外部封装，在每一次进入syscall时，会先根据syscall_id更新syscall_num的值。

2.在TaskControlBlock结构体中添加变量first_calltime、have_becalled。分别表示第一次系统调用时间和第一次系统调用是否发生，并相应实现了TaskManager中的查询函数和外部封装。在syscall_num的更新函数update_syscall_num中实现了对first_calltime的赋值。

3.基于1、2中的查询函数实现了sys_task_info

## 简答作业
### 1.正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容 (运行 Rust 三个 bad 测例 (ch2b_bad_*.rs) ， 注意在编译时至少需要指定 LOG=ERROR 才能观察到内核的报错信息) ， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。
[rustsbi] RustSBI version 0.3.0-alpha.2, adapting to RISC-V SBI v1.0.0

报错如下：
~~~
[kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
[kernel] IllegalInstruction in application, kernel killed it.
~~~

问题：bad_address尝试向0地址执行写入，这不被运行；bad_instruction尝试在用户态执行sret，指令非法；bad_register尝试在用户态下使用csrr指令访问状态和控制寄存器，指令非法。


### 2.深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:

#### 2.2.1 L40：刚进入 __restore 时，a0 代表了什么值。请指出 __restore 的两种使用情景。


## 荣誉准则

### 1.在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

无

### 2.此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

博客：https://blog.csdn.net/qq_44044341/article/details/126878446 从中学习实验思路和细节，实验代码均为自己实现。

### 3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

### 4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。