use app_example::app::service::call::TaskBootstrap;

fn main() {
    println!("Hello, App!");
    let boot = TaskBootstrap {};
    boot.call();
}
