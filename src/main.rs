use midir::{MidiInput, Ignore};
use std::error::Error;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // MIDI入力インスタンスを作成
    let mut midi_in = MidiInput::new("Renoise MIDI Receiver")?;
    midi_in.ignore(Ignore::None); // 全てのメッセージを受信

    // 利用可能なMIDI入力ポートを取得
    let in_ports = midi_in.ports();
    if in_ports.is_empty() {
        println!("No MIDI input ports available.");
        return Ok(());
    }

    println!("Available input ports:");
    for (i, port) in in_ports.iter().enumerate() {
        println!("{}: {}", i, midi_in.port_name(port)?);
    }

    // 使用するポートを選択
    print!("Select Renoise MIDI output port: ");
    stdout().flush()?;
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    let port_index: usize = input.trim().parse()?;
    let in_port = &in_ports[port_index];

    println!("Listening to MIDI messages from port: {}", midi_in.port_name(in_port)?);

    // MIDIメッセージをリアルタイムで処理するクロージャ
    let _conn_in = midi_in.connect(
        in_port,
        "Renoise MIDI Listener",
        move |_, message, _| {
          // println!("Received MIDI message: {:?}", message.len());
          //  println!("{:?}", msg(message));
           match msg(message) {
              None => {},
              Some(value) => println!("Message length: {}", &value),
           }
        },
        (),
    )?;

    println!("Press Enter to exit.");
    let mut exit = String::new();
    stdin().read_line(&mut exit)?;

    println!("Exiting...");
    Ok(())
}

fn msg(message: &[u8]) -> Option<u8> {
    if message.len() < 3 { // ✅ 長さが3未満なら安全に `None` を返す
        return None;
    }

    let first = message[0];
    let second = message[1];
    let third = message[2];



    Some(second) // ✅ `message.len()` を返す
}