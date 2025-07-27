pub mod barcode;
pub mod qr_code;
pub mod resistor_calculator;
pub mod tool;
pub mod tool_registry;
pub mod toola;

pub fn register_all_tools() {
    // toola::register();
    resistor_calculator::register();
    qr_code::register();
    barcode::register();
    //neat to try to recreate many different codes
    //try different hashes
    //ip address stuff
}

//wiki some random stuff and make it?
//if it a long undertaking, then its a project or blog? but post as a tool?
