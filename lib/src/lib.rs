// Draws start screen containing app version and ascii
pub fn draw_start_screen() {
    let ascii_name = r#"     _   _                             
    | | | |_   _ _ __  _ __   ___  ___ 
    | |_| | | | | '_ \| '_ \ / _ \/ __|
    |  _  | |_| | |_) | | | | (_) \__ \
    |_| |_|\__, | .__/|_| |_|\___/|___/
            |___/|_|                    "#;

    println!("{} v{}", &ascii_name, &env!("CARGO_PKG_VERSION"));
    println!("\n   Created by {}", &env!("CARGO_PKG_AUTHORS"));
    println!("==================================================")
}

