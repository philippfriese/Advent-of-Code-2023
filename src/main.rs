mod task_01;
mod task_02;
mod task_03;

mod utils {
    pub mod util;
}



fn main() {
    let test = false;
    println!("Mode: {:}",  if test {"test"} else {"data"});
    task_03::task_03::first(test);
    task_03::task_03::second(test);
}
