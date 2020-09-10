# System.rs
A Library for running shell commands in rust

## Description
System.rs allows you to run native system command from rust. But why not just use the Command
built into rust? Well, Imagine we have a super long ffmpeg command like
```console
ffmpeg -i input-video.avi -vn -acodec copy output-audio.aac
```
Lets see how we run that command with std::process::Command
```rust
use std::process::Command;
fn main(){
  let _cmd = Command::new("ffmpeg").args(&["-i", "input-video.avi", "-vn", "-acodec", "copy", "output-audio.aac"]).spawn();
}
```
Now system.rs
```rust
use system_rs::exec;
fn main(){
let _cmd = exec("ffmpeg", "-i input-video.avi -vn -acodec copy output-audio.aac");
}
```
The system.rs on looks alot cleaner and more readable, and by printing out the ```_cmd``` varible in system.rs you get all of the output of the command ran.

## Installation
If you use cargo you can add this to your projects ```Cargo.toml``` :
```toml
[dependencies]
system-rs = { git = "https://github.com/Interfiber/system-rs" }
```


## Example Usage

Execute a command and get its output:
```rust
use system_rs::exec;
fn main() {
  // Supply the command as the first argument, thhen all the arguments after it
  // exec("COMMAND", "ALL OF THE ARGUMENTS");
  let cmd = exec("ls", "-al /usr/lib");
  println!("Output of ls -al /usr/lib:\n{}", cmd);
}
```
The first 
