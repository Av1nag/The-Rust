struct RectangleDimensions {
    width: u32,
    height: u32,
}

pub fn main() {
    let dimensions_struct = RectangleDimensions {
        width: 30,
        height: 10,
    };

   print!("{}", area_of_rectangle(&dimensions_struct));
}

fn area_of_rectangle(rectangle_dimensions: &RectangleDimensions) -> u32 {
    rectangle_dimensions.width * rectangle_dimensions.height
}
