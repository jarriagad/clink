use rdev::{listen, Event, EventType};
use enigo::{Enigo, KeyboardControllable};
use std::env;
use std::thread;
use std::io::{self, Read};
use std::time::Duration;

fn main(){
    // collect args
    let args: Vec<String> = env::args().collect();
    let mut input = String::new();

    if args.len() > 1 {
        // If there is a command-line argument, use it as the input string
        input = args[1].clone();
    } else {
        // Otherwise, read from standard input (for piped data)
        io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    }

    // pew
    if let Err(error) = listen(move |event| callback(event, &input)) {
        println!("Error: {:?}", error);
    }
}

// Listens for click, waits 2 secods, then types string
// Make sure to use single quotes if using special chars
// Also note that this will only work with single line files
fn callback(event: Event, input_string: &str) {
    match event.event_type {
        EventType::ButtonPress(_) => {

            println!(
                "Mouse clicked!"
            );

            thread::sleep(Duration::from_secs(2));

            println!("Typing...");

            let mut enigo = Enigo::new();
            enigo.key_sequence(input_string);

            println!("Done typing");

            std::process::exit(0);
        }
        _ => {}
    }
}

