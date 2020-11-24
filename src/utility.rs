pub fn roundplaces(num: f64, places: u32) -> f64 {
    let ten: i16 = 10;
    let toplaces = ten.pow(places) as f64;
    let mut rounded = num * toplaces;
    rounded = rounded.round() / toplaces;
    rounded
}