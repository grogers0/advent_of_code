use std::fmt::Display;
use std::io::{self, Read};

struct Image {
    width: usize,
    height: usize,
    layers: usize,
    data: Vec<u8>
}

impl Image {
    fn layer_data(&self, layer: usize) -> &[u8] {
        let first = layer * self.width * self.height;
        let last = (layer + 1) * self.width * self.height;
        &self.data[first..last]
    }

    fn count_digits(&self, layer: usize, digit: u8) -> usize {
        self.layer_data(layer).iter().filter(|d| **d == digit).count()
    }

    fn pixel_value(&self, x: usize, y: usize) -> u8 {
        let offset = y * self.width + x;
        for layer in 0..self.layers {
            let px = self.layer_data(layer)[offset];
            if px != 2 {
                return px;
            }
        }
        2
    }
}

fn parse(width: usize, height: usize, input: &str) -> Image {
    let len = input.trim().len();
    let layers = len / width / height;
    assert_eq!(len, layers * width * height);
    let data = input.trim().chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();
    Image {
        width: width,
        height: height,
        layers: layers,
        data: data
    }
}

fn part1(input: &str) -> impl Display {
    let img = parse(25, 6, input);
    let mut layers = (0..img.layers).collect::<Vec<_>>();
    layers.sort_by(|layer1, layer2| img.count_digits(*layer1, 0).cmp(&img.count_digits(*layer2, 0)));
    let layer = layers[0];
    img.count_digits(layer, 1) * img.count_digits(layer, 2)
}

fn part2(input: &str) -> impl Display {
    let img = parse(25, 6, input);
    let mut output = String::new();
    for y in 0..img.height {
        for x in 0..img.width {
            let px = match img.pixel_value(x, y) {
                0 => '.',
                1 => '@',
                _ => panic!()
            };
            output.push(px);
        }
        output.push('\n');
    }
    output
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}
