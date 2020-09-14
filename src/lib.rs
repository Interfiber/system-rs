pub fn exec(program: &str, args: &str) -> String {
	//! ```exec``` runs a command and returns its output
	//! 
	//!
	//! Example
	//! ```
	//! let cmd = system_rs::exec("ls", "/");
	//! println!("The output of ls / is: {}", cmd);
	//! ```
    let split_args = args.split(" ");
    let vec = split_args.collect::<Vec<&str>>();
    let cmd = std::process::Command::new(program).args(&vec).stdout(std::process::Stdio::piped()).output().unwrap();
    let out = String::from_utf8(cmd.stdout).unwrap();
    return out.to_string();
}
pub fn which_exists(program: &str) -> bool {
	//! ```which_exists``` checks if a program exists
	//! in the path
	//!
	//! 
	//! Example
	//! ```
	//! if system_rs::which_exists("rustc") {
	//!		println!("Rust is not installed!");
	//! }
	//! ```
	let which = exec("/usr/bin/which", program);
	if which == "" {
		return false
	}else {
		return true
	}
}
