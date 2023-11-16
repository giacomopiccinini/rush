use std::fs::rename;

// Import the argument struct
use crate::MvArgs; 

pub fn execute(args: MvArgs) {

    // Move
    rename(&args.source, &args.target).expect("Move failed");
}

