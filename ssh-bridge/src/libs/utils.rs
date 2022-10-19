use std::{
    io::{Error, Write, Read},
    process::{Command, Output, Stdio}, net::TcpStream,
};

use ssh2::{Session, Channel};

// Handles running the IPMI command, expects parameters/args
async fn ssh_channel(target: &String, username: &String, password: &String) -> Result<Channel, ssh2::Error> {
    // Build correct address
    let address: String;
    if target.contains(":") {
        address = target.clone();
    }
    else {
        address = format!("{}:22", target);
    }
    // Establish TCP stream and create session (And auth...)
    let tcp = TcpStream::connect(address).expect("Failed to create TCP stream, is the target correct?");
    let mut session = Session::new().expect("Failed to create session");
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    session.userauth_password(username, password).expect("Failed to authenticate with target, Invalid credentials?");
    // Create channel
    Ok(session.channel_session().unwrap())
}

// Stop a remote server
pub async fn ssh_stop(target: &String ,username: &String, password: &String, elevated: &bool) {
    let mut channel = ssh_channel(target, username, password).await.unwrap();
    if *elevated {
        channel.exec(format!("echo {password} | sudo -S poweroff").as_str()).unwrap();
    }
    else {
        channel.exec("poweroff").unwrap();
    }
    channel.wait_eof().unwrap();
}
