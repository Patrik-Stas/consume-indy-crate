use crate::libindy_async::run_async;

mod libindy_async;

fn main() {
    println!("starting");
    run_async();
    println!("finished");
}