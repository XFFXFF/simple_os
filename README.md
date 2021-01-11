# simple_os
This repository contains the source code for the _Writing an OS in Rust_ series at [os.phil-opp.com](https://os.phil-opp.com).  
Thanks for this awesome blog series.

## Try simple_os
Installing rust nightly  
`rustup toolchain install nightly`  

Seting the nightly toolchain in the project's directory.  
`rustup override set nightly`  

Installing and creating bootimage
`cargo install bootimage`  
`rustup component add llvm-tools-preview`  
`cargo bootimage`  

Booting it in [QEMU](https://www.qemu.org/)  
Download QEMU and set it to the environment path  
`cargo run` or `cargo test`
