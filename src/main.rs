use std::{fs, thread};
use std::io::{stdin, Write};
use std::path::Path;

fn create_error() {

    panic!("Error Has been Created");
}

fn main() {

    let mut is_created = false;

    while !is_created {

        println!("Enter Command: ");
        let mut command = String::new();

        
        stdin().read_line(&mut command).unwrap();
        
        if command.contains("Y") {

            thread();

            get_file();

        }
            
        else if command.contains("") {
            create_error();
        }

        is_created = true
    }


}

fn thread(){

   thread::spawn(||  {
       println!("Hello World");
    });

}

fn get_file() {

    let mut file = fs::File::create("index.html").unwrap();

    let path = Path::new("index.html");

    if path.exists() {

        fs::File::create( "main.css").unwrap();
        fs::File::create("main.ts").unwrap();


        file.write("<!DOCTYPE html>".as_ref()).expect("TODO: panic message");
        file.write("<html lang='en'>".as_ref()).expect("TODO: panic message");
        file.write("<title>".as_ref()).expect("TODO: panic message");
        file.write("</title>".as_ref()).expect("TODO: panic message");
        file.write("</html>".as_ref()).expect("TODO: panic message");
        file.write("<body>".as_ref()).expect("TODO: panic message");
        file.write("</body>".as_ref()).expect("TODO: panic message");
    }

}


