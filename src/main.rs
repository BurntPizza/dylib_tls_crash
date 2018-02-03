
extern crate libloading;

fn main() {
    func();
    println!("Success: No thread");
    std::thread::spawn(func).join();
    println!("Success: Thread");
}

fn func() {
    let mut path = env!("CARGO_MANIFEST_DIR").to_string();
    path += "/target/debug/libtest_lib.so";
    let lib = libloading::Library::new(path).unwrap();
    println!("Dropping lib");
    if true {
        drop(lib);
    } else {
        std::mem::forget(lib);
    }
    println!("Lib is dropped");
}
