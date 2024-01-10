fn main() {
  let x:&str = "Hello, world!"; 
  print_type_of(&x);
}

fn print_type_of<T>(_: &T) {
  // Get variable and use the standard library to print the type 
  println!("{}", std::any::type_name::<T>());
}
