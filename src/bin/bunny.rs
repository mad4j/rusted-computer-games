/*
 * Original Basic program had the data in DATA format.
 * We're importing all the data into an array for ease of processing.
 * Format of data is
 * characters 0-4 is the letters that will be used in the output. 64 + the value represents the ASCII character
 * ASCII code 65 = A, 66 = B, etc.  so 2+64=66 (B), 21+64=85 (U) and so on.
 * Then we next have pairs of numbers.
 * Looking at the this data
 * 1,2,-1,0,2,45,50,-1
 * That reads as
 * 1,2 = draw characters - in this case BU
 * -1 = go to a new line
 * 0,2 = DRAW BUN
 * 45,50 = DRAW BUNNYB starting at position 45
 * and so on.
 * 4096 is EOF
 */

// values from original BUNNY DATA statements
#[rustfmt::skip]
const DATA: [i32; 233] = [
    2,21,14,14,25,
    1,2,-1,0,2,45,50,-1,0,5,43,52,-1,0,7,41,52,-1,
    1,9,37,50,-1,2,11,36,50,-1,3,13,34,49,-1,4,14,32,48,-1,
    5,15,31,47,-1,6,16,30,45,-1,7,17,29,44,-1,8,19,28,43,-1,
    9,20,27,41,-1,10,21,26,40,-1,11,22,25,38,-1,12,22,24,36,-1,
    13,34,-1,14,33,-1,15,31,-1,17,29,-1,18,27,-1,
    19,26,-1,16,28,-1,13,30,-1,11,31,-1,10,32,-1,
    8,33,-1,7,34,-1,6,13,16,34,-1,5,12,16,35,-1,
    4,12,16,35,-1,3,12,15,35,-1,2,35,-1,1,35,-1,
    2,34,-1,3,34,-1,4,33,-1,6,33,-1,10,32,34,34,-1,
    14,17,19,25,28,31,35,35,-1,15,19,23,30,36,36,-1,
    14,18,21,21,24,30,37,37,-1,13,18,23,29,33,38,-1,
    12,29,31,33,-1,11,13,17,17,19,19,22,22,24,31,-1,
    10,11,17,18,22,22,24,24,29,29,-1,
    22,23,26,29,-1,27,29,-1,28,29,-1,4096,
];

fn main() {
    // retrieve word letters from DATA array
    let mut bunny: [char; 5] = [' '; 5];
    for i in 0..5 {
        bunny[i] = char::from_u32(64 + DATA[i] as u32).unwrap_or_default();
    }

    // shadow with an immutable declaration
    let bunny: [char; 5] = bunny;

    // current cursor position
    let mut pos = 0;

    // number of processed lines
    let mut lines = 0;

    // current index of DATA value to be processed
    // first five values to be skipped
    let mut index = 5;

    // loop until end of data reached
    while DATA[index] < 64 {

        // current value to be processed
        let x = DATA[index];

        // verify if current value is an end of line
        if x < 0 {

            // print an end of line
            println!();

            // update current position
            pos = 0;

            // update number of processed lines
            lines += 1;

        // otherwise draw bunny shape
        } else {

            // verify if a letter needed to be rendered
            if (index - lines) % 2 == 0 {
                while pos <= x {
                    print!("{}", bunny[(pos % 5) as usize]);
                    pos += 1;
                }
            // otherwise render white spaces
            } else {
                print!("{}", " ".repeat((x - pos) as usize));
                pos = x;
            };
        }

        // update current index of DATA values
        index += 1;
    }
}
