pub mod buttons;
pub mod channels;
pub mod colours;
pub mod compressor;
pub mod device;
pub mod encoders;
pub mod eq_frequencies;
pub mod faders;
pub mod gate;
pub mod interaction;
pub mod microphone;
pub mod routing;
pub mod scribbles;
pub mod states;
pub mod submix;
pub mod version;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
