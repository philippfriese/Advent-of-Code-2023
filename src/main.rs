mod task_01;
mod task_02;

mod utils {
    pub mod util;
}



fn main() {
    let test = false;
    println!("Mode: {:}",  if test {"test"} else {"data"});
    task_02::task_02::first(test);
    task_02::task_02::second(test);
}
