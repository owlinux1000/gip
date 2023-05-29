use std::env;
#[macro_use] extern crate rocket;

use rocket_client_addr::ClientRealAddr;

static DEFAULT_PORT: u16 = 8080;

#[get("/")]
fn index(client_addr: &ClientRealAddr) -> String {
    match client_addr.get_ipv4_string() {
        Some(ipv4) => ipv4,
        None => client_addr.get_ipv6_string()
    }
}

fn get_port(port_key: &'static str) -> u16 {
    match env::var(port_key) {
        Ok(v) => match v.parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                println!(
                    "Invalid port number: {}",
                    v
                );
                DEFAULT_PORT
            }
        }, 
        Err(_) => {
            println!("${} is not defined.", port_key);
            DEFAULT_PORT
        }
    }
}

#[launch]
fn rocket() -> _ {
    let port_key = "PORT";
    let port = get_port(port_key);
    let figment = rocket::Config::figment()
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));
    rocket::custom(figment).mount("/", routes![index])
}
