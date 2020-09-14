pub fn exec(program: &str, args: &str) -> String {
    let split_args = args.split(" ");
    let vec = split_args.collect::<Vec<&str>>();
    let cmd = std::process::Command::new(program).args(&vec).stdout(std::process::Stdio::piped()).output().unwrap();
    let out = String::from_utf8(cmd.stdout).unwrap();
    return out.to_string();
}
