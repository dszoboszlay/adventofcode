use std::option::Option;

// input is a 40 character long string slice, containing a 8x5 pixel sprite.
//
// The # character means an "on" pixel of the sprite and a space means an
// "off" pixel.
pub fn ocr_4x6(input: &str) -> Option<char> {
    match input {
        "#####   ### #   #   ####" => Some('E'),
        " ## #  ##   # ###  # ###" => Some('G'),
        "#  ##  ######  ##  ##  #" => Some('H'),
        "  ##   #   #   ##  # ## " => Some('J'),
        "#   #   #   #   #   ####" => Some('L'),
        "### #  ##  #### #   #   " => Some('P'),
        "####   #  #  #  #   ####" => Some('Z'),
        _ => None
    }
}