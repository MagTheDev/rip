use std::io::Write;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};


const URL: &str = "https://api.ipify.org?format=json";


#[tokio::main]
async fn main() {

    print_colored(Color::Blue, "\n\n  ::  ");
    print_colored(Color::Green, "Fetching your IP address... \n");

    let resp = match reqwest::get(URL).await {
        Ok(resp) => resp,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let body = match resp.text().await {
        Ok(body) => body,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let data = match serde_json::from_str::<serde_json::Value>(&body) {
        Ok(data) => data,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    print_colored(Color::White, "");
    print!("      Your IP address is: ");
    print_colored(Color::Magenta, &data["ip"].as_str().unwrap());
    print!("\n\n\n")


}

fn print_colored(color: Color, text: &str) {
    let mut buffwrt = BufferWriter::stdout(ColorChoice::Always);
    let mut buff = buffwrt.buffer();
    buff.set_color(ColorSpec::new().set_fg(Some(color))).unwrap();
    write!(buff, "{}", text).unwrap();
    buffwrt.print(&buff).unwrap();
}
