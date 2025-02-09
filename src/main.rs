use std::sync::{Arc, Mutex};
use std::io::{stdin, stdout, Write};
use midir::{MidiInput, Ignore};
use opencv::{highgui, imgcodecs, prelude::*, Result};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let current_index = Arc::new(Mutex::new(0));
    let current_index_clone = Arc::clone(&current_index); // ✅ クロージャ用にクローン

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
            match msg(message) {
                None => {},
                Some(_) => {
                    let mut index = current_index_clone.lock().unwrap();
                    *index = (*index + 1) % 13; // ✅ 画像をローテーション
                    println!("Updated current_index: {}", *index);
                },
            }
        },
        (),
    )?;

    // 画像パス
    let image_paths = [
        "assets/11.png",
        "assets/12.png",
        "assets/13.png",
        "assets/14.png",
        "assets/15.png",
        "assets/16.png",
        "assets/17.png",
        "assets/18.png",
        "assets/19.png",
        "assets/20.png",
        "assets/21.png",
        "assets/22.png",
        "assets/23.png",
    ];

    // 画像をロード
    let images: Vec<Mat> = image_paths
        .iter()
        .map(|path| imgcodecs::imread(path, imgcodecs::IMREAD_COLOR))
        .collect::<Result<Vec<_>>>()?;

    if images.is_empty() {
        println!("No images loaded!");
        return Ok(());
    }

    // ウィンドウを作成し、全画面モードに設定
    let window_name = "Image Viewer";
    highgui::named_window(window_name, highgui::WINDOW_NORMAL)?;
    highgui::set_window_property(
        window_name,
        highgui::WND_PROP_FULLSCREEN,
        highgui::WINDOW_FULLSCREEN as f64,
    )?;

    println!("Starting image display loop...");

    loop {
        // `current_index` のロックを取得し、`usize` を取り出す
        let index = *current_index.lock().unwrap();
        println!("Displaying image at index: {}", index); // ✅ デバッグ出力

        if index >= images.len() {
            println!("Invalid index: {}", index);
            continue;
        }

        highgui::imshow(window_name, &images[index])?; // ✅ 画像を表示
        let key = highgui::wait_key(30)?; // ✅ `wait_key()` を追加し、ウィンドウを更新
        if key == 27 { // ESCキーで終了
            break;
        }
    }

    println!("Exiting...");
    Ok(())
}

fn msg(message: &[u8]) -> Option<u8> {
    if message.len() < 3 {
        return None;
    }
    
    println!("{:?}",message);

    if message[2] == 127 && message[1] != 0 {
        return Some(60);
    }

    None
}