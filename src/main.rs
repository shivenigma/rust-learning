fn main() {
    test_module::print_hello();
    test_module::print_world();
}
pub mod test_module {
    pub fn print_hello() {
        println!("Hello,");
    }
    pub fn print_world() {
        println!(" World!");
    }
}
