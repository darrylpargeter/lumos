use std::u8;

pub struct Config {
   pub hex: HexString,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enought arguments!");
        }

        let hex = HexString::new(args[1].clone());

        Ok(Config { hex })
    }
}

#[derive(Debug)]
pub struct HexString {
    hex_string: String,
}

impl HexString {
    pub fn new(hex_string: String) -> HexString {

        HexString {
            hex_string,
        }
    }

    fn split_to_hex(&self) -> Vec<Hex> {
        let char_list: Vec<char> = self.hex_string.chars().collect();

        let r: Hex = Hex::new(char_list[0], char_list[1]);
        let g: Hex = Hex::new(char_list[2], char_list[3]);
        let b: Hex = Hex::new(char_list[4], char_list[5]);

        vec!(r,g,b)
    }

    pub fn to_rgb(&self) -> RGB {
        let mut dec_vec: Vec<u8> = Vec::with_capacity(3);

        for hex in self.split_to_hex() {
            dec_vec.push(hex.to_dec());
        }

        RGB::new(dec_vec)
    }

    pub fn print(&self) {
        println!("#{}", &self.hex_string.to_uppercase());
    }
}


#[derive(Debug)]
pub struct Hex {
    hex: String,
}

impl Hex {
    pub fn new(first: char, second: char) -> Hex {
        let mut hex = String::new();

        hex.push_str(first.to_string().as_str());
        hex.push_str(second.to_string().as_str());

        Hex { hex }
    }

    pub fn to_dec(&self) -> u8 {
        u8::from_str_radix(&self.hex.as_str(), 16).unwrap()
    }
}

#[derive(Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(decs: Vec<u8>) -> RGB {
        RGB {
            r: decs[0],
            g: decs[1],
            b: decs[2]
        }
    }

    pub fn print(&self) {
        println!("rgb({}, {}, {})", &self.r, &self.g, &self.b);
    }

    pub fn to_hls(&self) -> HLS {
        HLS::new(self.r, self.g, self.b)
    }
}

#[derive(Debug)]
pub struct HLS {
    h: f32,
    l: f32,
    s: f32,
}

impl HLS {
    pub fn new(r: u8, g: u8, b: u8) -> HLS {
        let max_colour_value: f32 = 255_f32;
        let red: f32 = f32::from(r) / max_colour_value;
        let green: f32 = f32::from(g) / max_colour_value;
        let blue: f32 = f32::from(b) / max_colour_value;

        let rgb = vec!(red, green, blue);
        let max = rgb.iter().cloned().fold(0./0., f32::max);
        let min = rgb.iter().cloned().fold(0./0., f32::min);

        let luminace = (max + min) / 2_f32;

        let mut saturation: f32 = 0_f32;

        if max != min {
            if luminace > 0.5_f32 {
                saturation = (max - min) / (2.0 - max - min);
            } else {
                saturation = (max - min) / (max + min);
            }
        }

        let mut hue: f32 = 0_f32;

        if red > blue && red > green {
            hue = (green - blue) / (max - min);
        }
        if green > red && green > blue {
            hue = 2.0 + (blue - red) / (max - min);
        }
        if blue > green && blue > red {
            hue = 4.0 + (red - green) / (max - min);
        }

        let hue_as_degree = hue * 60_f32;

        HLS {
            h: hue_as_degree,
            l: (luminace * 100_f32).round(),
            s: saturation * 100_f32,
        }
    }

    pub fn print(self) {
        println!("hsl({}, {}%, {}%)", self.h, self.s, self.l);
    }
}

pub fn run(config: Config) {
    let rgb = config.hex.to_rgb();
    let hsl = rgb.to_hls();
    config.hex.print();
    rgb.print();
    hsl.print();
}
