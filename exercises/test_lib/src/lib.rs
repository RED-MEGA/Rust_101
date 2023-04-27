pub mod test_lib
{
	pub mod color_utils
	{
		pub enum Color
		{
			RED,
			GREEN,
			BLUE,
			YELLOW,
		}
		
		pub fn rgb_value(red : Color) -> (u8, u8, u8)
		{
			match red
			{
				Color::BLUE => (0, 0, 255),
				Color::RED => (255, 0, 0),
				Color::YELLOW => (255, 255, 0),
				Color::GREEN => (0, 255, 0)
			}
		}
	}
	
	pub mod shape_utils
	{
		use std::f32::consts::PI;
		pub enum Shape
		{
			Circle(f32),
			Square(f32),
			Triangle(f32, f32)
		}
		
        impl Shape {
            
            pub fn	calculates_area(&self) -> f32
            {
                    match self {
                        Shape::Circle(radius) => PI * radius * radius,
                        Shape::Square(len)=> len * len,
                        Shape::Triangle(base, height)=> base * height * 0.5,
                    }
            }
        }

	}
}

// pub use utils::*;