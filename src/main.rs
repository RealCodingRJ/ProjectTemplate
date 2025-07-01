use std::fs;
use std::io::{stdin, Write};


fn main() {

    let mut is_created = false;

    while !is_created {
        let mut dir = fs::create_dir("src");

        let mut command = String::new();

        stdin().read_line(&mut command).unwrap();

        if command.contains("Y") {

            get_file();

        }


        is_created = true
    }



}

fn get_file() {

    let mut file = fs::File::create("index.html").unwrap();

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


