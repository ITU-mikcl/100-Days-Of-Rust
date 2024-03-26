use std::io;

fn main() {
    let lines = io::stdin().lines();
    
    let mut content_for_line;

    let mut skewer = 0;
    let mut vegetarian_skewer = 0;

    for line in lines {
        content_for_line = line.unwrap();

        if content_for_line.contains('x') {
            skewer += 1;
        } else if content_for_line.contains('o') {
            vegetarian_skewer += 1;
        }

        if content_for_line.contains(']') || content_for_line.contains(')') {
            println!("[{vegetarian_skewer}, {skewer}]");
            break;
        }
    }
}