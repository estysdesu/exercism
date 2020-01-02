use unicode_segmentation::UnicodeSegmentation;

// pub fn reverse(input: &str) -> String {
//     let g = input.graphemes(true).rev();
//     let mut rev_g: String = String::new();
//     for u in g {
//         rev_g.push_str(u);
//     }
//     rev_g
// }

pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect::<Vec<&str>>().join("")
}
