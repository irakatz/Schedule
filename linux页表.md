### 为什么用虚拟内存？
为了解决进程之间内存隔离，提供了虚拟内存这个概念。
### 如何利用虚拟内存实现隔离？
MMU。进程访问虚拟内存，CPU执行时通过分页机制转换成物理内存访问，虚拟地址到物理内存的转换表称为页表，这个转换是个查表的过程。  
同一程序运行起来的两个进程，虚拟地址空间相同，但对应的物理空间是不相同的。OS需要给每个进程设置一份页表，在进程调度过程中，上下文切换阶段会做页表的切换。  
### 32bit的虚拟内存
ARM的CONFIG_PGTABLE_LEVELS=2，即两级页表，就是PGD->PTE
### 用多级页表是什么原因
如果想支持32位的操作系统下的4GB进程虚拟地址空间，假设页表大小为4K，则共有2的20次方页面。如果采用速度最快的1级页表，对应则需要2的20次方个页表项。一个页表项假如4字节，那么一个进程就需要（1048576*4=）4M的内存来存页表项。如果是采用2级页表，如图1，则创建进程时只需要有一个页目录就可以了，占用(1024*4)=4KB的内存。剩下的二级页表项只有用到的时候才会再去申请。  
（链接：https://zhuanlan.zhihu.com/p/79607142）  
### TLB（这个不是很重要）
内存访问可能会查很多次页表，那需要一个用于改进VA到PA转换速度的缓存。有了TLB之后，CPU访问某个虚拟内存地址的过程如下  
1.CPU产生一个虚拟地址  
2.MMU从TLB中获取页表，翻译成物理地址  
3.MMU把物理地址发送给L1/L2/L3/内存  
4.L1/L2/L3/内存将地址对应数据返回给CPU  

### ARM32页表映射
32bit虚拟地址的高12位作为PGD页表的index，也就是有4096个页表项，每个项指向一个PTE页表。虚拟地址的次8位是PTE页表的index，每个项可以找到20bit的物理页面地址。然后把这20bit和虚拟地址低12bit拼一块得到最后的32bit物理地址。  
在ARM中，以上是MMU干的。32位虚拟地址高位从TTBR0/1找1级页表，中间位是2级页表。而找到页表后small page的具体映射（https://www.cnblogs.com/arnoldlu/p/8087022.html）
### 页帧
内核把物理页作为内存管理的基本单位. 尽管处理器的最小可寻址单位通常是字, 但是, 内存管理单元MMU通常以页为单位进行处理. 因此，从虚拟内存的上来看，页就是最小单位.页帧代表了系统内存的最小单位。

### 再谈ioremap
驱动程序不能直接通过物理地址访问I/O内存资源，而必须将它们映射到核心虚地址空间内（通过页表），然后才能根据映射所得到的核心虚地址范围，通过访内指令访问这些I/O内存资源。 
ioremap用来将I/O内存资源的物理地址映射到核心虚地址空间（3GB－4GB）中（这里是内核空间） 
ioremap(cookie,size)  
void __iomem * __ioremap(phys_addr, size, flags); 这个flags是0，没有用  
void __iomem * (*arch_ioremap_caller)(phys_addr_t, size_t,unsigned int, void *) =__arm_ioremap_caller;  
然后是__arm_ioremap_pfn_caller(pfn, offset, size, mtype,caller); 其中pfn是页帧。 
这里使用了err = ioremap_page_range(addr, addr + size, paddr,__pgprot(type->prot_pte));  
ioremap_page_range为一段物理地址建立映射页表，若成功则返回0   
使用ioremap将它映射到内核虚拟空间，同时又用remap_page_range映射到用户虚拟空间，这样一来，内核和用户都能访问  
pgd_offset_k用于将虚拟地址转换，得到pmd的指针  
然后有err = ioremap_p4d_range(pgd, addr, next, phys_addr, prot);这样的东西。这里p4d可以换成pud,pmd,pte。pte是最后一级  
这个就是ioremap_pte_range的详述https://zhuanlan.zhihu.com/p/44944071  
https://blog.csdn.net/julie0107/article/details/46126231  
注意到set_pte_at在不同架构下实现方式不同，在arm下  
define set_pte_ext(ptep,pte,ext) cpu_set_pte_ext(ptep,pte,ext)  
define cpu_set_pte_ext PROC_TABLE(set_pte_ext)  
define PROC_TABLE(f) processor.f
### VA转PA?
实际上就是页表怎么做，这个在mmu.c的create_mapping中有说

