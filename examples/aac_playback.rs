use redlux::Decoder;
use rodio::{DeviceSinkBuilder, Player};
use std::fs::File;
use std::io::BufReader;

fn main() {
  let path = "tests/samples/Simbai & Elke Bay - Energy.aac";
  let file = File::open(path).expect("Error opening file");
  let buf = BufReader::new(file);

  let decoder = Decoder::new_aac(buf);

  let handle = DeviceSinkBuilder::open_default_sink().expect("Error creating output stream");
  let player = Player::connect_new(&handle.mixer());

  player.append(decoder);
  player.set_volume(0.25);
  player.sleep_until_end();
}
