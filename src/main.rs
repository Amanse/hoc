mod request;

use clap::Parser;
use request::Methods;

use crate::request::make_request;

#[derive(Parser)]
#[clap(version,about,long_about="High opinionated curl")]
struct  Args {
    #[clap(short,long,value_parser)]
    url: String,
    #[clap(short,long, value_parser)]
    method: Option<String>,
    #[clap(short, long, action)]
    pretty_json: bool,
    #[clap(short, long, action)]
    json_header: bool,
}

fn main() {
    let cli = Args::parse();

    let method_raw = cli.method.as_deref();
    let json_header = cli.json_header;
    let pretty_json = cli.pretty_json;

    let method: Methods = {
        match method_raw {
            Some("get") => Methods::Get,
            Some("post")=> Methods::Post,
            Some("put") => Methods::Put,
            Some("delete") => Methods::Delete,
            _ => Methods::Get,
        }
    };

    let resp = make_request(cli.url, method, json_header, pretty_json);
    println!("{}", resp);

}
