#![feature(linkage, naked_functions, asm, const_fn, proc_macro, used)]
//RUSTFLAGS="-C relocation-model=dynamic-no-pic -C code-model=kernel" RUST_BACKTRACE=1 cargo test --verbose --test kvm -- --nocapture

extern crate x86;
extern crate core;

#[macro_use]
extern crate klogger;

extern crate test_macros;
use test_macros::kvmattrs;


// Needed for code generated by `kvmattrs`
extern crate test;
use self::test::KvmTestMetaData;

#[test]
#[kvmattrs(identity_map, ram(0x30000000, 0x31000000))]
fn use_the_port() {
    log!("1");
    unsafe {
        asm!("inb $0, %al" :: "i"(0x01) :: "volatile");
    }
}

#[test]
#[kvmattrs(identity_map, ram(0x30000000, 0x31000000))]
fn io_example2() {
    assert!(1==1);
}
