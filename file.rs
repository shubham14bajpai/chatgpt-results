use std::env;
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

fn main() {
    // Get the command-line arguments as a vector of strings
    let args: Vec<String> = env::args().collect();

    // Check if at least two arguments were provided (the file name and path)
    if args.len() > 2 {
        // Get the file name and path from the arguments
        let file_name = &args[1];
        let path = &args[2];

        // Create a channel to receive the results from the threads
        let (tx, rx) = mpsc::channel();

        // Read the contents of the specified directory
        let entries = fs::read_dir(path).unwrap();

        // Create a shared counter for the number of threads
        let counter = Arc::new(Mutex::new(0));

        // Iterate over the entries
        for entry in entries {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let file_name = file_name.clone();
            let tx = tx.clone();
            let counter = counter.clone();

            // Check if the number of threads is less than 5
            if counter.lock().unwrap() < 5 {
                // Increment the number of threads
                *counter.lock().unwrap() += 1;

                // Spawn a thread
                thread::spawn(move || {
                    // Check if the entry is a file and if its name contains the file name
                    if entry_path.is_file() && entry_path.to_str().unwrap().contains(file_name) {
                        // Send the result to the channel
                        tx.send(entry_path.to_str().unwrap().to_owned()).unwrap();
                    }

                    // Decrement the number of threads
                    *counter.lock().unwrap() -= 1;
                });
            }
        }

        // Print the results from the threads
        for result in rx {
            println!("{}", result);
        }
    } else {
        // Print an error message if not enough arguments were provided
        println!("Error: Not enough arguments were provided");
    }
}
