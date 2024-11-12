pub fn parse_hotkey(hotkey_str: String) -> Option<(u32, u32)> {
    let parts: Vec<&str> = hotkey_str.split('+').collect();
    if parts.len() < 2 {
        return None;
    }

    let mut modifier = 0u32;
    for part in &parts[..parts.len() - 1] {
        modifier |= match part.to_lowercase().as_str() {
            "ctrl" => 0x0002,  // MOD_CONTROL
            "alt" => 0x0001,   // MOD_ALT
            "shift" => 0x0004, // MOD_SHIFT
            _ => return None,
        };
    }

    let key = match parts.last()?.to_uppercase().as_str() {
        "A" => 0x41,
        "B" => 0x42,
        "C" => 0x43,
        "D" => 0x44,
        "E" => 0x45,
        "F" => 0x46,
        "G" => 0x47,
        "H" => 0x48,
        "I" => 0x49,
        "J" => 0x4A,
        "K" => 0x4B,
        "L" => 0x4C,
        "M" => 0x4D,
        "N" => 0x4E,
        "O" => 0x4F,
        "P" => 0x50,
        "Q" => 0x51,
        "R" => 0x52,
        "S" => 0x53,
        "T" => 0x54,
        "U" => 0x55,
        "V" => 0x56,
        "W" => 0x57,
        "X" => 0x58,
        "Y" => 0x59,
        "Z" => 0x5A,
        "0" => 0x30,
        "1" => 0x31,
        "2" => 0x32,
        "3" => 0x33,
        "4" => 0x34,
        "5" => 0x35,
        "6" => 0x36,
        "7" => 0x37,
        "8" => 0x38,
        "9" => 0x39,
        "F1" => 0x70,
        "F2" => 0x71,
        "F3" => 0x72,
        "F4" => 0x73,
        "F5" => 0x74,
        "F6" => 0x75,
        "F7" => 0x76,
        "F8" => 0x77,
        "F9" => 0x78,
        "F10" => 0x79,
        "F11" => 0x7A,
        "F12" => 0x7B,
        "NUMPAD0" => 0x60,
        "NUMPAD1" => 0x61,
        "NUMPAD2" => 0x62,
        "NUMPAD3" => 0x63,
        "NUMPAD4" => 0x64,
        "NUMPAD5" => 0x65,
        "NUMPAD6" => 0x66,
        "NUMPAD7" => 0x67,
        "NUMPAD8" => 0x68,
        "NUMPAD9" => 0x69,
        "NUMPADMULTIPLY" => 0x6A,
        "NUMPADADD" => 0x6B,
        "NUMPADSUBTRACT" => 0x6D,
        "NUMPADDIVIDE" => 0x6F,
        "NUMPADDECIMAL" => 0x6E,
        "NUMPADENTER" => 0x0D,
        "BACK" => 0x08,
        "TAB" => 0x09,
        "RETURN" => 0x0D,
        "ESCAPE" => 0x1B,
        "SPACE" => 0x20,
        "PRIOR" => 0x21,
        "NEXT" => 0x22,
        "END" => 0x23,
        "HOME" => 0x24,
        "LEFT" => 0x25,
        "UP" => 0x26,
        "RIGHT" => 0x27,
        "DOWN" => 0x28,
        "SELECT" => 0x29,
        "PRINT" => 0x2A,
        "EXECUTE" => 0x2B,
        "SNAPSHOT" => 0x2C,
        "INSERT" => 0x2D,
        "DELETE" => 0x2E,
        "HELP" => 0x2F,
        "SCROLL" => 0x91,
        "ENTER" => 0x0D,
        "ESC" => 0x1B,
        _ => return None,
    };
    Some((modifier, key))
}

#[cfg(test)]
mod tests {
    use crate::hotkey;

    #[test]
    fn it_works() {
        let hotkey_str = "Ctrl+Shift+Z".to_string();
        let (modifier, key) = hotkey::parse_hotkey(hotkey_str).unwrap();
        assert_eq!(modifier, 0x0006);
        assert_eq!(key, 0x5A);
    }

    #[test]
    fn it_works_2() {
        let hotkey_str = "Ctrl+Alt+s".to_string();
        let (modifier, key) = hotkey::parse_hotkey(hotkey_str).unwrap();
        assert_eq!(modifier, 0x0003);
        assert_eq!(key, 0x53);
    }
}
