/*
   Copyright 2020 Tabacaru Eric

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/ 



use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;


const MAX: u16 = 65535;

#[allow(dead_code)]
struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16
}


impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("too many arguments");
        }


        let f = args[1].clone();



        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {flag: String::from(""), ipaddr, threads: 4});

        } else {
            let flag = args[1].clone();

            if flag.contains("-h") || flag.contains("-help") && args.len() == 2 {
                println!("Usage: -th to select how many threads you want to use
                            \r\n-h or -help to show this message.");
                
                return Err("help");

            } else if flag.contains("-h") || flag.contains("-help") {
                return Err("too many arguments");
            } else if flag.contains("-th") {

                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IPADDR; must be IPv4 or IPv6")
                };

                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("failed to parse thread number")
                };

                return Ok(Arguments{threads, flag, ipaddr});
            } else {
                return Err("invalid syntax");
            }
        }
    }
}


fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, thr_num: u16) {
    let mut port: u16 = start_port + 1;

    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");

                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= thr_num {
            break;
        }

        port += thr_num;

    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let arguments = Arguments::new(&args).unwrap_or_else(

        |err| {
            if err.contains("help") {
                process::exit(0);

            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(0);
            }
        }

    );

    let thr_num = arguments.threads;
    let addr = arguments.ipaddr;
    let ( tx, rx ) = channel();

    for i in 0..thr_num {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, thr_num)
        });
    }

    let mut out = vec![];

    drop(tx);

    for p in rx {
        out.push(p);
    }

    println!("");

    out.sort();

    for v in out {
        println!("{} is open!", v);
    }

}
