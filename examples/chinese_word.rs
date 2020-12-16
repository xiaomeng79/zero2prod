use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let s = "我是 chinese people ☺";
    let g = s.unicode_words().collect::<Vec<&str>>();
    for v in g.iter() {
        println!("{}", v)
    }
}
