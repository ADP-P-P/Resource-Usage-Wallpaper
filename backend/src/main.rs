#![windows_subsystem = "windows"]
#![allow(unused)]
use std::{
    borrow::BorrowMut,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread::sleep,
    time::Duration,
    cell::Cell
};

use once_cell::unsync::OnceCell;
use sysinfo::System;

const KB: f64 = (1 << 10) as f64;
const MB: f64 = (1 << 20) as f64;
const GB: f64 = (1 << 30) as f64;
const HOST: &str = "localhost:26781";
static mut SYS: OnceCell<System> = OnceCell::new();
fn main() {
    unsafe { SYS.set(System::new_all()) };
    let listener = TcpListener::bind(HOST).unwrap();
    for stream in listener.incoming() {
        let tcp_stream = stream.unwrap();
        displash_request(tcp_stream);
    }
}

fn displash_request(mut stream: TcpStream) {
    let mut buf = [0; 512];
    stream.read(&mut buf).unwrap();
    let mut buffer = String::from_utf8_lossy(&buf);
    let mut i = 0;
    let mut j = 0;
    for chari in 0..buffer.as_bytes().len() {
        if buffer.as_bytes()[chari] == b'/' {
            i = chari;
        }
        if buffer.as_bytes()[chari] == b' ' && i != 0 {
            j = chari;
            break;
        }
    }
    let response = if j - i < 1 {
        "-1".to_string()
    } else {
        let req = buffer[i + 1..j].to_lowercase();
        let req = req.as_str();
        match req {
            "cpu" => {
                format!("{}", cpu_usage())
        }
            "memt" => {
                format!("{}", total_memory())
            }
            "mem" => {
                format!("{}", mem_usage())
            }
            _ => "-1".to_string(),
        }
    };

    let response = format!("{}{}", "HTTP/1.1 200 OK\r\n\r\n", response);
    stream.write(response.as_bytes());
    stream.flush().unwrap();
}

fn total_memory() -> u64 {
    let mut sys: &mut System = shared_sys();
    sys.refresh_all();
    sys.total_memory()
}

fn mem_usage() -> u64 {
    let mut sys: &mut System = shared_sys();
    sys.refresh_memory();
    sys.used_memory()
}

fn cpu_usage() -> f32 {
    let mut sys: &mut System = shared_sys();
    sys.refresh_cpu();
    sys.global_cpu_info().cpu_usage()
}

#[inline]
fn shared_sys() -> &'static mut System {
    unsafe { SYS.get_mut().unwrap() }
}

fn demo() {
    let mut sys = System::new_all();
    sys.refresh_all();
    let total_memory = sys.total_memory() as f64;
    loop {
        sleep(Duration::from_secs(1));
        sys.refresh_memory();
        let used_memory = sys.used_memory() as f64;
        let used_cpu = sys.global_cpu_info().cpu_usage();
        println!(
            "memory usage: {}, with rate: {:.2} %",
            transform_form(used_memory),
            (used_memory / total_memory) * 10.0
        );
        println!("cpu usage: {}", used_cpu);
    }
}
fn transform_form(num: f64) -> String {
    if num < KB {
        format!("{} byte", num)
    } else if num >= KB && num < MB {
        format!("{:.2} KB", num / KB)
    } else if num >= MB && num < GB {
        format!("{:.2} MB", num / MB)
    } else {
        format!("{:.2} GB", num / GB)
    }
}
