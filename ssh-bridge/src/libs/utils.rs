use std::{
    io::{Error, Write},
    process::{Command, Output, Stdio},
};

// TODO: REMOVE THE PUBNESS OF THIS FUNCTION
// Handles running the IPMI command, expects parameters/args
pub fn ssh_cmd(target: &String, username: &String, password: &String, command: &String) -> Result<Output, Error> {
    
}

// // Starts a remote server
// pub fn ipmi_start(username: &String, password: &String, target: &String) {
//     ipmi_power(username, password, target, "on");
// }

// // Soft stops a remote server
// pub fn ipmi_soft_stop(username: &String, password: &String, target: &String) {
//     ipmi_power(username, password, target, "soft");
// }
