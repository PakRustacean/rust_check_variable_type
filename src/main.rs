use std::any::type_name;

fn type_is<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let x = "MCMXCIV".to_string();
    println!("{}", type_is(&x));
}
