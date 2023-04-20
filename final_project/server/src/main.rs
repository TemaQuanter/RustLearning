/// This program is a TCP/IP server that transfers some random
/// pictures and text over the protocol.
///
use rand::{self, Rng};
use std::{
    fs::{self, File},
    io::{Read, Write, BufReader, BufRead},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

/// This struct represents an image file.
/// It stores
///     1. File name of the picture
///     2. Content of the picture (bytes)
/// 
struct Image {
    file_name: String,
    data: Vec<u8>
} // end struct Image

fn main() {
    // Create a listener over a particular IP address.
    let listener: TcpListener = TcpListener::bind("127.0.0.1:5533")
        .expect("Failed to establish a connection through the provided IP address");

    // Accept connections and process them serially.
    for stream in listener.incoming() {
        handle_stream(stream.expect("Failed to get the data from an incoming connection"));
    } // end for
} // end main()

/// This function handles a connection.
/// 
fn handle_stream(mut stream: TcpStream) {
    // Get the request type from a client.
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // Create a response for the client.
    let response: &str = match &request_line[..] {
        "GET / HTTP/1.1" => "HTTP/1.1 200 OK\r\n\r\n",
        _ => "HTTP/1.1 404 NOT FOUND\r\n\r\n"
    }; // end match

    // Respond to the client.
    stream.write_all(response.as_bytes())
        .expect("Failed to establish a connection with a client");

    // Check if the client tried to access a non-existing page,
    // finish session.
    if response == "HTTP/1.1 404 NOT FOUND" {
        return;
    } // end if

    println!("{response}");

    // Continue sending some data to the client, while they are alive.
    loop {
        // Get the random image and its name (name, image).
        let (image_name, image_data) : (String, Vec<u8>) = get_random_image();

        let img: Image = Image {
            file_name: image_name,
            data: image_data
        }; // end image_data

        // Debug
        println!("Image name: {}", img.file_name);

        // Transfer the content to the client.
        stream.write_all(&img.data[..])
            .expect("Failed to transfer the data to a client");

        break;

        // Sleep for some time before handling the next request.
        thread::sleep(Duration::from_millis(1500));
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
