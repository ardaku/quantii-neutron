use quantii_neutron::{QuantiiService, start_quantii_desktop};

fn main() {
    // launch quantii services
    let services: Vec<QuantiiService> = vec![];

    services.iter().for_each(|f| {
        f.run_service();
    });

    // launch quantii desktop
    start_quantii_desktop();
}
