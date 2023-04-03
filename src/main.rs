mod models;
mod processor;

use std::env;
use crate::processor::QueryProcessor;

#[tokio::main]
async fn main() {
    let mut debug_mode: bool = false;
    let args: Vec<String> = env::args().collect();

    for arg in args {
        if arg.contains("verbose") {
            debug_mode = true;
        }
    }

    let processor = QueryProcessor::new(debug_mode);
    processor.start().await;
}
