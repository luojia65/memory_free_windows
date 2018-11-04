fn get_total_and_available_memory_size() -> (usize, usize) {
    use winapi::um::winbase::GlobalMemoryStatus;
    use winapi::um::winbase::MEMORYSTATUS;
    unsafe {
        let status = Box::new(MEMORYSTATUS {
            dwLength: 0,
            dwMemoryLoad: 0,
            dwTotalPhys: 0,
            dwAvailPhys: 0,
            dwTotalPageFile: 0,
            dwAvailPageFile: 0,
            dwTotalVirtual: 0,
            dwAvailVirtual: 0,
        });
        let status_ptr = Box::into_raw(status);
        GlobalMemoryStatus(status_ptr);
        let status = Box::from_raw(status_ptr);
        (status.dwTotalPhys as usize, status.dwAvailPhys as usize)
    }
}

fn main() {
    println!("正在释放中！");
    let (total, before) = get_total_and_available_memory_size();
    let size = total / 4 * 3;
    let mut vec = Vec::with_capacity(size);
    for _i in 0..size {
        vec.push(0u8);
    }
    std::thread::sleep(std::time::Duration::new(3, 0));
    drop(vec);
    std::thread::sleep(std::time::Duration::new(1, 0));
    let (_, after) = get_total_and_available_memory_size();
    let freed = after - before;
    println!("恭喜您！您已释放了 {:.2}MB 的内存！", freed as f64 / 1048576.0);      
}
