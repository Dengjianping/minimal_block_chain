#![allow(unused_imports, unused)] 

mod block_types;
mod server;
mod utils;
mod views;
mod sha256;
mod cuda_ffi;

use crate::block_types::*;
use crate::sha256::*;
use crate::cuda_ffi::*;


fn main() {
    //digest();
    let s = String::from("jdeng");
    let c = cuda_sha256("jdeng");
    println!("hash: {}", c);
    pre_process();
    println!("Hello, world!");
    trial();
    server::block_chain_server();
}
