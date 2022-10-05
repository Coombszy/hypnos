use std::{
    io::Error,
    process::{exit, Command, Output},
    str::from_utf8,
};

// Handles running the IPMI command, expects parameters/args
fn ipmi_cmd(params: String) -> Result<Output, Error> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!(" ipmitool {params} "))
        .output();

    output
}

// Wraps around the ipmi_cmd function to add parameters for power control
fn ipmi_power(username: &String, password: &String, target: &String, state: &str) {
    let command = format!("-I lanplus -U {username} -P '{password}' -H {target} power {state}");
    match ipmi_cmd(command) {
        Ok(o) => match o.status.success() {
            true => (),
            false => {
                println!(
                    "ipmitool command failed: {:?}",
                    from_utf8(&o.stderr).unwrap()
                );
            }
        },
        Err(e) => {
            println!("Failed to execute ipmitool commands, is it installed?\n {e}");
            exit(1);
        }
    }
}

// Starts a remote server
pub fn ipmi_start(username: &String, password: &String, target: &String) {
    ipmi_power(username, password, target, "on");
}

// Soft stops a remote server
pub fn ipmi_soft_stop(username: &String, password: &String, target: &String) {
    ipmi_power(username, password, target, "soft");
}
