use stats_alloc::{INSTRUMENTED_SYSTEM, Region, StatsAlloc};
use std::alloc::System;
use std::collections::LinkedList as StdLinkedList;
use byte_unit::Byte;
use xor_ll::LinkedList as XorLinkedList;

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

fn main() {
    for n in [1000, 10_000, 100_000, 1_000_000, 10_000_000] {
        println!("\n Iters: {n}");
        // Benchmark XorLinkedList
        {
            let reg = Region::new(&GLOBAL);
            let mut xor_list = XorLinkedList::new();
            for i in 0..n {
                xor_list.push_back(i);
            }
            drop(xor_list);
            let xor_stats = reg.change();
            println!("XorLinkedList: {}", Byte::from_u64(xor_stats.bytes_allocated as u64).get_appropriate_unit(byte_unit::UnitType::Binary));
        }

        // Benchmark StdLinkedList
        {
            let reg = Region::new(&GLOBAL);
            let mut std_list = StdLinkedList::new();
            for i in 0..n {
                std_list.push_back(i);
            }
            drop(std_list);
            let std_stats = reg.change();

            println!("StdLinkedList: {}", Byte::from_u64(std_stats.bytes_allocated as u64).get_appropriate_unit(byte_unit::UnitType::Binary));
        }
    }
}
