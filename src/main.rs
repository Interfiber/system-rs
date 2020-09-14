fn main() {
	let ls_command = system_rs::exec("ls", "/usr/local/bin");
	println!("/usr/local/bin contatins:\n{}", ls_command);
	let rust_exists = system_rs::which_exists("rustc");
	// which_exists takes in a &str(program) and returns a bool
	if !rust_exists {
		println!("Rust is not installed!");
	}else {
		println!("Rust exists!");
	}
}