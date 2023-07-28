use byteorder::{ByteOrder, LittleEndian};
use goxlr_shared::colours::{Colour, ColourScheme, FaderColour, OneColour, ThreeColour, TwoColour};

/// This is an extension of the ColourScheme structure, that builds the byte array that the
/// GoXLR expects, based on the configuration of the colours.
trait ColourStruct {
    fn build_colour_map(&self) -> Vec<u8>;
}

trait ColourConfiguration {
    fn get_bytes(&self) -> Vec<u8>;
}

trait LegacyColourConfiguration {
    fn get_bytes(&self, is_legacy: bool) -> Vec<u8>;
}

trait ColourBytes {
    fn packed(&self) -> u32;
    fn get_bytes(&self) -> Vec<u8>;
}

impl ColourStruct for ColourScheme {
    fn build_colour_map(&self) -> Vec<u8> {
        let mut map = vec![];
        for button in &self.scribbles {
            map.append(&mut button.get_bytes());
        }

        for button in &self.mood {
            map.append(&mut button.get_bytes());
        }

        for button in &self.mutes {
            map.append(&mut button.get_bytes());
        }

        for fader in &self.faders {
            map.append(&mut fader.get_bytes(self.is_legacy));
        }

        for dummy in &self.dummy1 {
            map.append(&mut dummy.get_bytes());
        }

        for button in &self.presets {
            map.append(&mut button.get_bytes());
        }

        for encoder in &self.encoders {
            map.append(&mut encoder.get_bytes());
        }

        for dummy in &self.dummy2 {
            map.append(&mut dummy.get_bytes());
        }

        for button in &self.sample_banks {
            map.append(&mut button.get_bytes());
        }

        for button in &self.sample_buttons {
            map.append(&mut button.get_bytes());
        }

        for button in &self.fx_buttons {
            map.append(&mut button.get_bytes());
        }

        for button in &self.mic_buttons {
            map.append(&mut button.get_bytes());
        }

        for button in &self.dummy3 {
            map.append(&mut button.get_bytes());
        }

        map
    }
}

/// OneColour is generally used either as a spacer, or for situations where a feature was
/// supposed to be present, but no longer is. A couple of these appear in the struct. Technically
/// we can just spam 4 empty bytes, but we'll implement it properly for now
impl ColourConfiguration for OneColour {
    fn get_bytes(&self) -> Vec<u8> {
        self.colour1.get_bytes()
    }
}

impl ColourConfiguration for TwoColour {
    fn get_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(&mut self.colour1.get_bytes());
        result.append(&mut self.colour2.get_bytes());

        result
    }
}

impl LegacyColourConfiguration for FaderColour {
    fn get_bytes(&self, is_legacy: bool) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(&mut self.colour1.get_bytes());
        result.append(&mut self.colour2.get_bytes());

        // When GoXLR animations were added, the size of a fader Colour Set increased from 2 to
        // 14. From what I can tell during testing, none of the additionally added colours are used
        // for anything, so we'll just pad our result out with zeros. In future, is_legacy will be
        // replaced by a 'FEATURE_SET' map which we can use here to check.
        if !is_legacy {
            let mut extension = vec![0; 12 * 4];
            result.append(&mut extension);
        }

        result
    }
}

impl ColourConfiguration for ThreeColour {
    fn get_bytes(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.append(&mut self.left.get_bytes());
        result.append(&mut self.right.get_bytes());
        result.append(&mut self.knob.get_bytes());

        result
    }
}

impl ColourBytes for Colour {
    fn packed(&self) -> u32 {
        ((self.red) << 16) | ((self.green) << 8) | (self.blue)
    }

    fn get_bytes(&self) -> Vec<u8> {
        let mut value = [0; 4];
        LittleEndian::write_u32(&mut value, self.packed());

        Vec::from(value)
    }
}
