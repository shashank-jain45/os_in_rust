#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_in_rust::test_runner)]
#![reexport_test_harness_main="test_main"]


use core::panic::PanicInfo;
use os_in_rust::println;
use bootloader::{BootInfo,entry_point};
use os_in_rust::memory;
extern crate alloc;
use alloc::{boxed::Box,vec,vec::Vec,rc::Rc};

entry_point!(kernel_main);

fn kernel_main(boot_info:&'static BootInfo) -> ! {
    use os_in_rust::memory::{self,BootInfoFrameAllocator};
    use x86_64::{structures::paging::Page,VirtAddr};
    use os_in_rust::allocator;

    println!("Hello World{}","!");
    os_in_rust::init();

    // let phys_mem_offset=VirtAddr::new(boot_info.physical_memory_offset);
    // let l4_table=unsafe{active_level_4_table(phys_mem_offset)};

    // for (i,entry) in l4_table.iter().enumerate(){
    //     use x86_64::structures::paging::PageTable;

    //     if !entry.is_unused(){
    //         println!("L4 Entry {}: {:?}",i,entry);
    //     }

    //     let phys=entry.frame().unwrap().start_address();
    //     let virt=phys.as_u64()+boot_info.physical_memory_offset;
    //     let ptr=VirtAddr::new(virt).as_mut_ptr();
    //     let l3_table: &PageTable=unsafe{&*ptr};

    //     for (i,entry) in l3_table.iter().enumerate(){
    //         if !entry.is_unused(){
    //             println!(" L3 Entry {}: {:?}",i,entry);
    //         }
    //     }
    // }


    let phys_mem_offset=VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper=unsafe{memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    let x=Box::new(41);
    println!("heap value at {:p}",x);
    // let page=Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};


    // #[cfg(test)]
    // test_main();
    let mut vec=Vec::new();
    for i in 0..500{
        vec.push(i);
    }
    println!("vec at {:p}",vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    println!("It did not crash!");
    os_in_rust::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    os_in_rust::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os_in_rust::test_panic_handler(info)
}

