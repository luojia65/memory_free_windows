# memory_free_windows
释放windows系统的内存（纯属娱乐）

# 原理
非常暴力，不断向系统申请内存，当RAM占用已满时，释放已申请的内存
获取系统内存占用部分使用了winapi crate
