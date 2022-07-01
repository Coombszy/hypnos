use actix_web::{web, App, HttpServer};
use log::info;

use hypnos_library::draw_start_screen;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    startup();

    // Start Web
    let host: String = "0.0.0.0".to_string();
    let port: u16 = 4050;
    info!("Starting web server, listenting on {host}:{port}");
    HttpServer::new(move || {
        App::new()
    })
    .bind((host, port))?
    .run()
    .await

    // Testing
    /*
    let mac_address: [u8; 6] = [0x00, 0x23, 0x24, 0x82, 0x10, 0x46];
    
    let magic_packet = wake_on_lan::MagicPacket::new(&mac_address);

    magic_packet.send();

    println!("Sent!");
    */
}


fn startup() {
    draw_start_screen();
}

