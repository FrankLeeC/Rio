// use mem::StringKV;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::time;

fn main() {
    // let mut a = StringKV::new();
    // a.put(String::from("a"), String::from("b"));
    // let v = a.get("a");
    // println!("{}", v);

    std::thread::spawn(move || {
        server();
    });

    std::thread::sleep(time::Duration::from_millis(100));
    
    client("command a q", "this is a content with 中文");
    client("command a", "this is a content with 中文@");

   
}

fn client(c: &str, s: &str) {
    let mut stream = TcpStream::connect("127.0.0.1:9000")
    .expect("could not connect to server");
 
    // println!("中文 len: {}", "中文".len());
    // println!("中文 len: {}", "中文".chars().count());

    let len = s.as_bytes().len();
    let content = format!("{}\n{}\n{}", c, len, s);
    // let content = len.to_string() + String::from('\n').as_ref() + s;

    println!("send: {}", content);

    stream.write(content.as_bytes())
    .expect("faild to write to server");
    let mut reader = BufReader::new(&stream);

    let mut buffer: Vec<u8> = Vec::new();
    reader.read_until(b'\n', &mut buffer)
    .expect("Could not read into buffer");
    print!("{}", String::from_utf8(buffer)
    .expect("Could not write buffer as string"));

}

fn server() {
    let r = TcpListener::bind("127.0.0.1:9000");
    let tcp = match r {
        Ok(tcp) => tcp,
        Err(e) => {
            println!("bind error: {}", e);
            return
        },
    };
    
    for stream in tcp.incoming() {
        match stream {
            Ok(s) => process(s),
            Err(_) => return,
        }
       
    }
}

fn parse_len(s: String) -> usize {
    let r = s.trim().parse::<usize>();
    match r {
        Ok(i) => return i,
        Err(e) => {
            println!("parse_len err: {}", e);
            return 0
        },
    }
}

fn process(mut stream: TcpStream) {
    
    let response = "OK\n".as_bytes();
    let error = "ERR\n".as_bytes();

    let stream_clone = match stream.try_clone() {
        Ok(c) => c,
        Err(_) => {
            match stream.write(error) {
                Ok(_) => {
                    let _ = stream.flush();
                    return
                },
                Err(_) => {
                    println!("stream write error");
                    return
                }
            }
        },
    };
    let mut reader = BufReader::new(stream);
    let mut writer = BufWriter::new(stream_clone);

    let mut first_line = String::new();
    let command = match reader.read_line(&mut first_line) {
        Ok(_) => first_line,
        Err(_) => match writer.write(error) {
            Ok(_) => {
                let _ = writer.flush(); 
                return
            },
            Err(_) => {
                println!("stream write error");
                return
            }
        }
    };
    println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> server");

    let mut second_line = String::new();
    let len = match reader.read_line(&mut second_line) {
        Ok(_) => parse_len(second_line),
        Err(_) => match writer.write(error) {
            Ok(_) => {
                let _ = writer.flush(); 
                return
            },
            Err(_) => {
                println!("stream write error");
                return
            }
        }
    };


    if len <= 0 {
        match writer.write("INVALID COMMAND\n".as_bytes()) {
            Ok(_) => {
                let _ = writer.flush(); 
                return
            },
            Err(_) => {
                println!("stream write error");
                return
            }
        }
    }
    let mut buffer = vec![0; len];
    match reader.read(&mut buffer) {
        Ok(n) => {
            if n == len {
                println!("command: {}content: {}", command, String::from_utf8_lossy(&buffer));
                writer.write(response).unwrap();
                writer.flush().unwrap();
            }
        },
        Err(_) => match writer.write("READ ERR\n".as_bytes()) {
            Ok(_) => {
                let _ =writer.flush(); 
                return
            },
            Err(_) => {
                println!("stream write error");
                return
            }
        },
    }
    println!("<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<< server");
    
    
}
