/// This program is a TCP/IP server that transfers some random
/// pictures and text over the protocol.
///
use rand::{self, Rng};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    thread::{self, JoinHandle},
    time::Duration,
    sync::{Arc, Mutex}
};

/// This struct represents an image file.
/// It stores
///     1. File name of the picture
///     2. Content of the picture (bytes)
///
#[derive(Serialize, Deserialize)]
struct Image {
    file_name: String,
    data: Vec<u8>,
} // end struct Image

fn main() {
    // Create a listener over a particular IP address.
    let listener: TcpListener = TcpListener::bind("127.0.0.1:5533")
        .expect("Failed to establish a connection through the provided IP address");

    // Accept connections and process them serially.
    for stream in listener.incoming() {
        handle_stream(Arc::new(Mutex::new(stream.expect("Failed to get the data from an incoming connection"))));
    } // end for
} // end main()

/// This function handles a connection.
///
fn handle_stream(mut stream: Arc<Mutex<TcpStream>>) {
    // Get the request type from a client.
    // let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    // let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Create a thread that sends key words to a client.

    // Create a reference to the stream.

    let moved_stream: Arc<Mutex<TcpStream>> = Arc::clone(&stream);

    let thr: JoinHandle<()> = thread::spawn(move || {
        // A special variable that determines it the transfer of
        // data was successful.
        let mut success: bool = false;

        loop {
            // Try to get the access to the stream to transfer data.
            if let Ok(mut local_stream) = moved_stream.lock() {
                // Sent a key word.
                local_stream.write_all("Key word\r\n".as_bytes())
                    .expect("Failed to send the key word");

                // Debug
                println!("Key word sent!");

                // Mark the data transfer as successful.
                success = true;
            } // end if let

            // Sleep for some time in case the data transfer was successful.
            if success {
                success = false;
                thread::sleep(Duration::from_secs(1));
            } // end if
        } // end loop
    });

    // A special variable that determines if the transfer of data
    // was successful.
    let mut success: bool = false;

    // Continue sending some data to the client, while they are alive.
    loop {
        // Try to get an access to the stream to transfer data.
        if let Ok(mut local_stream) = stream.lock() {
            // Get the random image and its name (name, image).
            let (image_name, image_data): (String, Vec<u8>) = get_random_image();

            let img: Image = Image {
                file_name: image_name,
                data: image_data,
            }; // end image_data

            // Convert an image data structure to JSON.
            let json_img_data: String =
                serde_json::to_string(&img).expect("Failed to convert Image structure to JSON");

            // Format a proper response to the client.
            let response: String = format!("{}\r\n", json_img_data);

            // Debug
            println!("Image name: {}", img.file_name);

            // Transfer the content to the client.
            local_stream
                .write_all(response.as_bytes())
                .expect("Failed to transfer the data to a client");

            // Mark the data transfer as successful.
            success = true;
        } // end if let

        // If the data transfer was successful, then sleep
        // for some time.
        if success {
            success = false;
            // Sleep for some time before handling the next request.
            thread::sleep(Duration::from_millis(1500));
        } // end if
    } // end loop
} // end handle_stream()

/// This function gets a random picture from the database and
/// returns it.
///
fn get_random_image() -> (String, Vec<u8>) {
    // The path to the directory with images.
    const PATH_IMAGES: &str = "images/";

    // The vector where all the image names are stored.
    let mut image_names: Vec<String> = Vec::new();

    // The random index of the image to pick up.
    let rand_ind: usize;

    // File to read the information about the picture in.
    let mut file: File;

    // Picture that is ready to be transferred through TCP/IP protocol.
    let mut content: Vec<u8> = Vec::new();

    // Add all the names of the images to the special vector.

    // Check if the path was found successfully.
    if let Ok(entries) = fs::read_dir(PATH_IMAGES) {
        // For each entry in the directory.
        for entry in entries {
            // Check if the entry is valid.
            if let Ok(entry) = entry {
                image_names.push(entry.file_name().to_string_lossy().to_string());
            } // end if
        } // end for
    } // end for

    // Pick up a random image from the list of images.
    rand_ind = rand::thread_rng().gen_range(0..image_names.len());

    // Read the picture.

    // Open a file.
    file = File::open(format!("{PATH_IMAGES}/{}", image_names[rand_ind]))
        .expect("Failed to open a picture");

    // Read a picture.
    file.read_to_end(&mut content)
        .expect("Failed to read a picture");

    // Return the name of the file and the content itself.
    (image_names[rand_ind].clone(), content)
} // end get_random_image()
