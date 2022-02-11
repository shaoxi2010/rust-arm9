# 项目目标
由于目前rust官方没有提供arm9的裸机环境支持，但是本身又不想放弃f1c200这个便宜又实在的平台。
这里就单纯的抛砖引玉作用了，吸引大家一起来研究下arm9的rust开发。

代码参考来源xboot，大佬真强。
# 使用rust与c开发的优缺点
1. cargo的存在使得rust不用在额外学习makefile等，简化项目管理统一编译
2. rust本身提供了很多基础实现，可以很方便的使用
3. rust提供的抽象可以很直观的理解代码含义，自身命名空间有效解决隔离问题
4. rust的抽象成本非常低，异常逻辑完善，debug起来很直观
5. rust的格式化字符串消耗较大，但灵活度高，在小内存比较难控制
6. rust的内联汇编与GCC内联汇编有些许的不一样，需要一定的学习成本
# 实现原理
核心思想与gcc编译内核一致，只要需要rust生成代码时，去除掉允许库相关依赖，程序严格
上来讲是可以允许在裸机环境。这时我们在采用arm-none-eabi-gcc的链接功能，将程序链接
成为一个完整环境即可，即可以同时将C与RUST一同编译链接实现。
# 在编写程序中遇到的问题
1. 首先裸机开发需要link.lds链接文件，需要将text,data,rodata(不链接没有字符串)，bss段
2. 在start的启动汇编上需要设置sp地址否则会发现程序异常跑飞，qemu反复验证
3. 启动汇编需要清理bss段，否则static变量会出问题，qemu反复验证
4. rust内连汇编切记，会引起寄存器变化一定要使用inout实现，否则变量可能会被认为不变而不会对寄存器赋值直接跑飞，反汇编
5. 对于cpu操作应当详细参考arm手册，操作cp异常也可能会导致程序卡死，cache_flush卡死，qemu联调发现
6. rust的目标armv5te-unknown-linux-gnueabi目标会使用动态链接器，如Scrt1.o、libc，需要添加-no-pie和-no-stdlib避免链接
7. unwind在裸机环境并不支持，需要关闭栈回溯功能使用profile下panic = "abort"
8. __aeabi_unwind_cpp_pr0和eh_personality方法依赖于libgcc，也是栈回溯的一部分，空实现就行。
9. 关闭mmu并不会对数据进行flush操作，需要手动flush相关内容，不知道是不是ARM9特有
10. asm与gloabl_asm均会对{}进行解析处理，arm指令内带{}需要使用{{}}包装，或者使用raw模式，参见rust内联汇编文档
11. mmu的页彪需要对齐，否则会卡死在打开mmu上，qemu联调
# 程序运行与程序验证
1. 首先使用rustup target add armv5te-unknown-linux-gnueabi添加rust的arm9支持 (nightly)
2. 使用apt install arm-none-eabi-gcc安装arm工具链
3. 使用cargo build --release 生成二进制
4. 使用 arm-none-eabi-objcopy --binray target/armv5te-unknown-linux-gnueabi/release/rust-arm9 arm9生成裸机运行二进制
5. 下载目标机器
# QEMU虚拟化运行和gdb联调
在实际环境下可能没有设备可以调试，使用QEMU虚拟机调试是很有效的办法。
## QMUE 平台
1. 当前代码平台适配versatilepb即ARM Versatile/PB (ARM926EJ-S)
2. 使用qemu-system-arm -M versatilepb -serial stdio -display none -kernel target/armv5te-unknown-linux-gnueabi/release/rust-arm9运行
3. 即可看到hello world 输出
## GDB联合调试
1. 添加QEMU调试参数-s -S运行即qemu-system-arm -s -S -M versatilepb -serial stdio -display none -kernel target/armv5te-unknown-linux-gnueabi/release/rust-arm9
2. 使用gdb-multiarch target/armv5te-unknown-linux-gnueabi/release/rust-arm9加载GDB
3. 进入gdb后使用target remote :1234，即可看到程序停在入口_start
4. 使用break等命令添加断点，或者反汇编等操作，一步步监视运行，调试程序bug
