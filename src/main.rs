
use std::net::{UdpSocket, SocketAddr};
use std::env;
use std::str::FromStr;
use rand::Rng;
use std::error::Error;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    // Check if the correct number of arguments is provided
    if args.len() != 3 {
        return Err(format!("Usage: {} <server_ip:port> <local_ip:port>", args[0]).into());
    }
    let client_socket = UdpSocket::bind(&args[2]).unwrap();
    let server_addr: SocketAddr = SocketAddr::from_str(&args[1]).unwrap();
    let server_socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let (client_tx, client_rx): (Sender<Option<SocketAddr>>, Receiver<Option<SocketAddr>>) = mpsc::channel();
    server_socket.connect(server_addr).unwrap();
    let mut rng = rand::thread_rng();

    // Generate a random length between 1 and 1024
    let len: usize = rng.gen_range(1..=1024);

    // Create a buffer with the random length
    let mut buf = vec![0; len];
    // Fill the buffer with random data
    rng.fill(&mut buf[..]);
    client_socket.recv_from(&mut buf).unwrap();
    server_socket.send_to(&buf, server_addr).unwrap();
    {
        let server_socket = server_socket.try_clone().unwrap();
        let client_socket = client_socket.try_clone().unwrap();
        thread::spawn(move || {
            let mut buf = [0; 128 * 1024];
            let mut client_addr: Option<SocketAddr> = None;
            loop {
                let amt = server_socket.recv(&mut buf).unwrap();
                println!("{}", amt);
                let t = client_rx.try_recv();
                if !(t.is_err()){
                    if t.unwrap() != client_addr{
                        client_addr = t.unwrap();
                    }
                }
                if client_addr.is_none(){

                } else {
                    client_socket.send_to(&buf[..amt], client_addr.unwrap()).unwrap();
                }


            }
        });
    }
    let mut buf = [0; 128 * 1024];
    loop {
        let (amt, src) = client_socket.recv_from(&mut buf).unwrap();
        client_tx.send(Some(src)).unwrap();
        server_socket.send(&buf[..amt]).unwrap();
    }
    Ok(())
}
