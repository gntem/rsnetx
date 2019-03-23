#[macro_use]
extern crate serde_derive;
extern crate docopt;

use docopt::Docopt;
use dns_lookup::lookup_host;

const USAGE: &'static str = "
rsnetx

Usage:
  rsnetx lookup <hostname>

  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_hostname: Vec<String>,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    let arghostname = args.arg_hostname;
    let hostname = &arghostname[0];

    println!("Looking up [{}]", hostname);

    let _ips: Vec<std::net::IpAddr> = lookup_host(hostname).unwrap();

    for address in _ips.iter() {
        println!("{:?}", address);
    }
}
