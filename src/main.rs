mod task_01;

mod utils {
    pub mod util;
}



fn main() {
    println!("Hello, world!");

    let test = false;
    println!("Mode: {:}",  if test {"test"} else {"data"});
    //task_01::task_01::first(test);
    task_01::task_01::second(test);
}
