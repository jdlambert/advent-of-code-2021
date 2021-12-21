struct Image {
    pixels: Vec<u8>,
    height: usize,
    width: usize,
    padding: u8,
}

fn index_or_pad(image: &Image, i: Option<usize>, j: usize) -> usize {
    if i.is_some() && i.unwrap() < image.height && j < image.width {
        image.pixels[i.unwrap() * image.width + j] as usize
    } else {
        image.padding as usize
    }
}

fn down_num(image: &Image, prev: usize, i: usize) -> usize {
    let corner = index_or_pad(image, Some(i), 0);
    (prev & 0b111_111) << 3 // Lose the top three bits
        | (image.padding as usize) << 2
        | (image.padding as usize) << 1
        | corner
}

fn right_num(image: &Image, prev: usize, i: usize, j: usize) -> usize {
    let top = index_or_pad(image, i.checked_sub(2), j);
    let middle = index_or_pad(image, i.checked_sub(1), j);
    let bottom = index_or_pad(image, Some(i), j);
    (prev & 0b011_011_011) << 1 // Lose the leftmost three bits
        | top << 6
        | middle << 3
        | bottom
}

fn step(image: &Image, code: &Vec<u8>) -> Image {
    let new_width = image.width + 2;
    let new_height = image.height + 2;

    let mut new_pixels = Vec::with_capacity(new_width * new_height);
    let mut row_head_score = if image.padding == 0 { 0 } else { 511 };
    let mut cell_score;
    for i in 0..new_height {
        row_head_score = down_num(image, row_head_score, i);
        cell_score = row_head_score;
        new_pixels.push(code[cell_score]);
        for j in 1..new_width {
            cell_score = right_num(image, cell_score, i, j);
            new_pixels.push(code[cell_score]);
        }
    }
    let padding = if image.padding == 0 {
        code[0]
    } else {
        code[511]
    };

    Image {
        pixels: new_pixels,
        padding,
        width: new_width,
        height: new_height,
    }
}

fn pixel_count(image: &Image) -> usize {
    image.pixels.iter().filter(|&&v| v == 1).count()
}

fn parse(s: &str) -> Vec<u8> {
    s.chars()
        .filter(|&c| c == '#' || c == '.')
        .map(|bit| if bit == '#' { 1 } else { 0 })
        .collect()
}

fn main() {
    let (code, image) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let code = parse(code);
    let pixels = parse(image);

    let width = image.lines().next().unwrap().len();
    let height = pixels.len() / width;
    let mut image = Image {
        pixels,
        width,
        height,
        padding: 0,
    };
    for _ in 0..2 {
        image = step(&image, &code);
    }
    println!("Part 1: {}", pixel_count(&image));
    for _ in 2..50 {
        image = step(&image, &code);
    }
    println!("Part 2: {}", pixel_count(&image));
}
