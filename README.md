# Schedule
### 6.26
刚开始学习Rust，预计2日解决语法  
### 6.27
凌晨装好了基于CLion的IDE  
解决了大部分rustlings，明天可以完工
### 6.28
写完了rustlings.  
总结是前面做的还不错，对rust基础语法都能掌握。但是后面做的比较磕绊，比如泛型，调用函数这些做的不够好。  
### 6.29
今天做编程题  
没有看懂《基于笨办法学...》是什么样的练习题，看起来要找15道题目，我选择洛谷的OJ，在开启O2优化（Rust在OJ上用时较长，会有TLE）下通过大部分测试样例即可，目前已解决2道。  
之后会补充到15道。  
接下来是自学RISC-V  
### 6.30
看了RISC-V的指令手册和PPT，不太清楚要做什么。所以需要看rCore tutorial代码
### 7.1
凌晨装好了实验环境，开始看Lab0
### 7.2
凌晨解决了Lab0。但是Lab1编译出现了问题，不知道什么原因。
现在我发现不如直接看已有代码来分析，也就是直接写阅读报告。在按照标准流程一步一步来时我发现有的代码无法执行，比如Lab1的println。可能原因是以上代码是根据已经完工的结果来拆分的。
### 7.3
今天看了提供的论文。  
在Lab2中提到了线段树，需要用rust解决一个类似的题目，以证明掌握这个东西
在Lab4中看到一些线程调度算法，我可以设计类似的算法，以证明掌握这个东西（但是在Lab6中对调度有一些修改，不知道我的随意修改是否会影响）  
现在需要做的是写对于Lab0到6的代码报告（写于report.txt），以及完成上面两个Lab的任务。
### 7.4
今天在忙别的事，写了1个rust例题，看了一遍线段树的实现。
### 7.5
很抱歉昨天忘记记录这个事情。昨天在写rust例题，我认为我对于rust的类型转换，字符串，Result和Option这块掌握的不够好，有必要写一写。  
另外，我还没有涉及到文件处理的题目，也需要在明天补上。
### 7.6
在为了学习rust而重写以前的题目。有点多，还没写完  
实际上应该学rust的语言特性，这些题目不太难  
### 7.7
解决了15题
今天暂时把时间放在复习OS上
### 7.8
今天看了linux页表相关的内容。以及一些hypervisor的内容。不过这个是偏ARM的，实际写操作系统应该用不到hypervisor？  
### 7.9
linux页表.md是最近关于页表的学习。ARM32bit是两级页表。
### 7.10
仍然是ARM的内容，更新见linux页表.md（实际上这个和Riscv关系不大）
### 7.14
更新一下，最近两天在研究ARM（一些其他的任务），现在在研究页面置换算法。这个是根据课本上来的，还没有确定能不能用
### 7.18
最近重新编译了lab0到6的代码，按照流程走了一遍（而不是只读就结束了），以及写了两个实验。线段树是仿照已有的版本来做的。  
lab1的time tick没有做出来，但是不影响后面的流程。在后面操作时这个任务被解决了。  
实际上理解不难，但是改bug的时间比较长，实验流程里面有些是很绕的，比如文件名错了，没有给import，少写了一些类等。  
我会考虑写线程调度的算法。以及写一个report。  
