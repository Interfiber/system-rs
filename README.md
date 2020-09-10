# System.rs
A Library for running shell command in rust

## Description
System.rs allows you to run native system command from rust. But why not just use the Command
built into rust? Well, Imagine we have a super long ffmpeg command like
```
ffmpeg -i input-video.avi -vn -acodec copy output-audio.aac
```
Lets see how we run that command with std::process::Command
```
use std::process::Command;
fn main(){
  let _cmd = Command::new("ffmpeg").args(&["-i", "input-video.avi", "-vn", "-acodec", "copy", "output-audio.aac"]).spawn();
}
```
Now system.rs
```
use system_rs::exec;
fn main(){
let _cmd = exec("ffmpeg", "-i input-video.avi -vn -acodec copy output-audio.aac");
}
```
The system.rs on looks alot cleaner and more readable, and by printing out the ```_cmd``` varible in system.rs you get all of the output of the command ran.

## Installation
If you use cargo you can add this to your projects ```Cargo.toml``` :
```
[dependencies]
system-rs = { git = "https://github.com/Interfiber/system-rs" }
```


## Example Usage

Execute a command and get its output:
```
use system_rs::exec;
fn main() {
  let cmd = exec("ls", "-al /usr/lib");
  println!("Output of ls -al /usr/lib:\n{}", cmd);
}
```
