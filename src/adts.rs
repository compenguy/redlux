use std::ops::Range;

fn get_bits(byte: u16, range: Range<u16>) -> u16 {
  let shaved_left = byte << range.start - 1;
  let moved_back = shaved_left >> range.start - 1;
  let shave_right = moved_back >> 16 - range.end;
  return shave_right;
}

fn get_bits_u8(byte: u8, range: Range<u8>) -> u8 {
  let shaved_left = byte << range.start - 1;
  let moved_back = shaved_left >> range.start - 1;
  let shave_right = moved_back >> 8 - range.end;
  return shave_right;
}

pub fn construct_adts_header(track: &mp4::Mp4Track, sample: &mp4::Mp4Sample) -> Option<Vec<u8>> {
  // ADTS header wiki reference: https://wiki.multimedia.cx/index.php/ADTS#:~:text=Audio%20Data%20Transport%20Stream%20(ADTS,to%20stream%20audio%2C%20usually%20AAC.

  // byte7 and byte9 not included without CRC
  let adts_header_length = 7;

  // AAAA_AAAA
  let byte0 = 0b1111_1111;

  // AAAA_BCCD
  // D: Only support 1 (without CRC)
  let byte1 = 0b1111_0001;

  // EEFF_FFGH
  let mut byte2 = 0b0000_0000;
  let object_type = match track.audio_profile() {
    Ok(mp4::AudioObjectType::AacMain) => 1,
    Ok(mp4::AudioObjectType::AacLowComplexity) => 2,
    Ok(mp4::AudioObjectType::AacScalableSampleRate) => 3,
    Ok(mp4::AudioObjectType::AacLongTermPrediction) => 4,
    Err(_) => return None,
  };
  let adts_object_type = object_type - 1;
  byte2 = (byte2 << 2) | adts_object_type; // EE

  let sample_freq_index = match track.sample_freq_index() {
    Ok(mp4::SampleFreqIndex::Freq96000) => 0,
    Ok(mp4::SampleFreqIndex::Freq88200) => 1,
    Ok(mp4::SampleFreqIndex::Freq64000) => 2,
    Ok(mp4::SampleFreqIndex::Freq48000) => 3,
    Ok(mp4::SampleFreqIndex::Freq44100) => 4,
    Ok(mp4::SampleFreqIndex::Freq32000) => 5,
    Ok(mp4::SampleFreqIndex::Freq24000) => 6,
    Ok(mp4::SampleFreqIndex::Freq22050) => 7,
    Ok(mp4::SampleFreqIndex::Freq16000) => 8,
    Ok(mp4::SampleFreqIndex::Freq12000) => 9,
    Ok(mp4::SampleFreqIndex::Freq11025) => 10,
    Ok(mp4::SampleFreqIndex::Freq8000) => 11,
    Ok(mp4::SampleFreqIndex::Freq7350) => 12,
    // 13-14 = reserved
    // 15 = explicit frequency (forbidden in adts)
    Err(_) => return None,
  };
  byte2 = (byte2 << 4) | sample_freq_index; // FFFF
  byte2 = (byte2 << 1) | 0b1; // G

  let channel_config = match track.channel_config() {
    // 0 = for when channel config is sent via an inband PCE
    Ok(mp4::ChannelConfig::Mono) => 1,
    Ok(mp4::ChannelConfig::Stereo) => 2,
    Ok(mp4::ChannelConfig::Three) => 3,
    Ok(mp4::ChannelConfig::Four) => 4,
    Ok(mp4::ChannelConfig::Five) => 5,
    Ok(mp4::ChannelConfig::FiveOne) => 6,
    Ok(mp4::ChannelConfig::SevenOne) => 7,
    // 8-15 = reserved
    Err(_) => return None,
  };
  byte2 = (byte2 << 1) | get_bits_u8(channel_config, 6..6); // H

  // HHIJ_KLMM
  let mut byte3 = 0b0000_0000;
  byte3 = (byte3 << 2) | get_bits_u8(channel_config, 7..8); // HH
  byte3 = (byte3 << 4) | 0b1111; // IJKL

  let frame_length = adts_header_length + sample.bytes.len() as u16;
  byte3 = (byte3 << 2) | get_bits(frame_length, 3..5) as u8; // MM

  // MMMM_MMMM
  let byte4 = get_bits(frame_length, 6..13) as u8;

  // MMMO_OOOO
  let mut byte5 = 0b0000_0000;
  byte5 = (byte5 << 3) | get_bits(frame_length, 14..16) as u8;
  byte5 = (byte5 << 5) | 0b11111; // OOOOO

  // OOOO_OOPP
  let mut byte6 = 0b0000_0000;
  byte6 = (byte6 << 6) | 0b111111; // OOOOOO
  byte6 = (byte6 << 2) | 0b00; // PP

  return Some(vec![byte0, byte1, byte2, byte3, byte4, byte5, byte6]);
}