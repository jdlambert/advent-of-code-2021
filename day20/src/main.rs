struct Image {
    pixels: Vec<u8>,
    width: usize,
    padding: u8,
}

fn index(image: &Image, i: usize, j: usize) -> u8 {
    image.pixels[i * image.width + j]
}

fn pixel_count(image: &Image) -> usize {
    image.pixels.iter().filter(|&&v| v == 1).count()
}

fn start_num(prev_image: &Image) -> usize {
    (0..9).fold(0, |num, _| num << 1 | prev_image.padding as usize)
}

fn down_num(image: &Image, prev: usize, i: usize) -> usize {
    let height = image.pixels.len() / image.width;
    let new = if i >= height - 1 {
        image.padding
    } else {
        image.pixels[i * image.width]
    };
    (prev & 0b111_111) << 3 // Lose the top three bits
        | (image.padding as usize) << 2
        | (image.padding as usize) << 1
        | new as usize
}

fn right_num(image: &Image, prev: usize, i: usize, j: usize) -> usize {
    let (new0, new1, new2) = if j >= image.width {
        (image.padding, image.padding, image.padding)
    } else {
        let n0 = if i > 1 {
            index(image, i - 2, j)
        } else {
            image.padding
        };
        let height = image.pixels.len() / image.width;
        let n1 = if i > 0 && i < height + 1 {
            index(image, i - 1, j)
        } else {
            image.padding
        };
        let n2 = if i < height {
            index(image, i, j)
        } else {
            image.padding
        };
        (n0, n1, n2)
    };
    (prev & 0b011_011_011) << 1 // Lose the leftmost three bits
        | (new0 as usize) << 6
        | (new1 as usize) << 3
        | (new2 as usize)
}

fn step(image: &Image, code: &Vec<u8>) -> Image {
    let padding = if image.padding == 0 {
        code[0]
    } else {
        code[511]
    };

    let new_width = image.width + 2;
    let new_height = image.pixels.len() / image.width + 2;

    let mut new_pixels = vec![0; new_width * new_height];
    let offset = new_width + 1;
    for (line, chunk) in image.pixels.chunks(image.width).enumerate() {
        let pos = line * new_width + offset;
        new_pixels[pos..pos + image.width].copy_from_slice(chunk);
    }

    let mut row_head_score = start_num(image);
    let mut cell_score;
    for i in 0..new_height {
        row_head_score = down_num(image, row_head_score, i);
        cell_score = row_head_score;
        new_pixels[i * new_width] = code[cell_score];
        for j in 1..new_width {
            cell_score = right_num(image, cell_score, i, j);
            new_pixels[i * new_width + j] = code[cell_score];
        }
    }

    Image {
        pixels: new_pixels,
        padding,
        width: image.width + 2,
    }
}

fn main() {
    let (code, image) = include_str!("../input.txt").split_once("\n\n").unwrap();
    let code: Vec<_> = code
        .chars()
        .filter(|&c| c == '#' || c == '.')
        .map(|bit| if bit == '#' { 1 } else { 0 })
        .collect();
    let pixels: Vec<_> = image
        .chars()
        .filter(|&c| c == '#' || c == '.')
        .map(|bit| if bit == '#' { 1 } else { 0 })
        .collect();
    let width = image.lines().next().unwrap().len();
    let mut image = Image {
        pixels,
        width,
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
