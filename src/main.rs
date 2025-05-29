use clap::{Arg, Command};
mod capture;
mod transmit;
mod receive;
mod api;
mod config;
mod metrics;

fn main() {
    let matches = Command::new("SecureOT Insight Agent")
        .version("1.0")
        .author("SecureOT Developer")
        .about("Captura y transmite tráfico OT")
        .arg(Arg::new("send").long("send").help("Activar modo emisor"))
        .arg(Arg::new("receive").long("receive").help("Activar modo receptor"))
        .arg(Arg::new("dual").long("dual").help("Modo dual: transmisión y recepción simultánea"))
        .arg(Arg::new("api").long("api").help("Activar API REST"))
        .get_matches();

    if matches.get_flag("send") {
        transmit::start_transmission();
    }
    if matches.get_flag("receive") {
        receive::start_reception();
    }
    if matches.get_flag("dual") {
        transmit::start_transmission();
        receive::start_reception();
    }
    if matches.get_flag("api") {
        api::start_server();
    }
}
// Actualizacion de codigo 2025-06-11
// Actualizacion de codigo 2025-06-09
// Actualizacion de codigo 2025-05-31
// Actualizacion de codigo 2025-05-21
// Actualizacion de codigo 2025-06-08
// Actualizacion de codigo 2025-05-29
