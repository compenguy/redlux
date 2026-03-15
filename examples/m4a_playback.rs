use redlux::Decoder;
use rodio::{DeviceSinkBuilder, Player};
use std::fs::File;
use std::io::BufReader;

fn main() {
  let path = "tests/samples/Simbai & Elke Bay - Energy.m4a";
  let file = File::open(path).expect("Error opening file");

  let metadata = file.metadata().expect("Error getting file metadata");
  let size = metadata.len();
  let buf = BufReader::new(file);

  let decoder = Decoder::new_mpeg4(buf, size).expect("Error creating M4aDecoder");

  let handle = DeviceSinkBuilder::open_default_sink().expect("Error creating output stream");
  let player = Player::connect_new(&handle.mixer());

  player.append(decoder);
  player.set_volume(0.25);
  player.sleep_until_end();
}
