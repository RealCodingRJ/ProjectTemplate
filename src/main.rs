use std::{fs, thread};
use std::io::{stdin, Write};

fn main() {

    let mut is_created = false;

    while !is_created {
        let mut dir = fs::create_dir("src");

        let mut command = String::new();

        stdin().read_line(&mut command).unwrap();

        if command.len() < 1 {
            panic!("Length of Command Not More: {}", command)
        }

        if command.contains("Y") {

            threads();

            get_file();

        }


        is_created = true
    }


}

fn threads() {

    let x_sleep = 2000 / 2;

    let m =  thread::sleep(x_sleep.abs());
    return m;
}

fn is_empty(message: String) -> bool {

    return message == ""
}

fn get_file() {

    let mut file = fs::File::create("index.html").unwrap();

    if (is_empty(file)) {

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


