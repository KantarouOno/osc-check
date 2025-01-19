use rosc::decoder::decode;
use rosc::OscPacket;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // バインドするUDPソケットのアドレスとポートを指定
    let socket = UdpSocket::bind("0.0.0.0:8000")?;
    println!("Listening for OSC messages on 0.0.0.0:8000");

    let mut buf = [0u8; 1024];

    loop {
        // OSCメッセージを受信
        let (size, addr) = socket.recv_from(&mut buf)?;

        println!("Received {} bytes from {}", size, addr);

        // OSCパケットをデコード
        match decode(&buf[..size]) {
            Ok(packet) => match packet {
                OscPacket::Message(msg) => {
                    println!("Received OSC Message: {}", msg.addr);

                    // 引数が空かどうか確認
                    if msg.args.is_empty() {
                        println!("No arguments in this OSC message.");
                    } else {
                        println!("Arguments: {:?}", msg.args);
                    }
                }
                OscPacket::Bundle(bundle) => {
                    println!("Received OSC Bundle: {:?}", bundle);
                }
            },
            Err(err) => {
                eprintln!("Failed to decode OSC packet: {}", err);
            }
        }
    }
}
