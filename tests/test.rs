use redlux::Decoder;
use rodio::{DeviceSinkBuilder, Player};
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

#[test]
fn play_m4a() {
  let path = "tests/samples/Simbai & Elke Bay - Energy.m4a";
  let file = File::open(path).expect("Error opening file");

  let metadata = file.metadata().expect("Error getting file metadata");
  let size = metadata.len();
  let buf = BufReader::new(file);

  let decoder = Decoder::new_mpeg4(buf, size).expect("Error creating M4aDecoder");

  let handle = DeviceSinkBuilder::open_default_sink().expect("Error creating output stream");
  let player = Player::connect_new(&handle.mixer());

  player.append(decoder);
  player.set_volume(0.0);
  // play audio for 200ms at 0.0 volume
  thread::sleep(Duration::from_millis(200));
}

#[test]
fn play_aac() {
  let path = "tests/samples/Simbai & Elke Bay - Energy.aac";
  let file = File::open(path).expect("Error opening file");
  let buf = BufReader::new(file);
  let decoder = Decoder::new_aac(buf);

  let handle = DeviceSinkBuilder::open_default_sink().expect("Error creating output stream");
  let player = Player::connect_new(&handle.mixer());

  player.append(decoder);
  player.set_volume(0.0);
  // play audio for 200ms at 0.0 volume
  thread::sleep(Duration::from_millis(200));
}
