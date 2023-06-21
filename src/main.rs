use std::env;

fn hello(name: &str) {
    let mut output = format!("\nHello, {} :)\n", name);
    // add the date and time to the output
    output.push_str(&format!(
        "\nThe date and time is: {}\n",
        chrono::offset::Local::now()
    ));
    // print the output to the terminal
    println!("{}", &output);
    // write the output to a file
    std::fs::write("output.txt", output);
    println!("output.txt created or overwritten");
}

fn main() {
    if let Some(name) = env::args().nth(1) {
        hello(&name);
    } else {
        println!("Please provide an argument");
    }
}
