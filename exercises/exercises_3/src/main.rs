/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: reben-ha <reben-ha@student.1337.ma>        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/04/25 17:49:25 by reben-ha          #+#    #+#             */
/*   Updated: 2023/04/25 20:17:46 by reben-ha         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */


/* Exercise : Shape */

use std::f32::consts::PI;

enum Shape
{
	Circle(f32),
	Square(f32),
	Triangle(f32, f32)
}

impl Shape
{
	fn	calculates_area(&self) -> f32
	{
		match self {
			Shape::Circle(radius) => PI * radius * radius,
			Shape::Square(len)=> len * len,
			Shape::Triangle(base, height)=> base * height * 0.5,
		}
	}
}

/* Exercise : Color */

enum Color
{
	RED,
	GREEN,
	BLUE,
	YELLOW,
}

impl Color
{
	fn rgb_value(&self) -> (u8, u8, u8)
	{
		match self
		{
			self::Color::BLUE => (0, 0, 255),
			self::Color::RED => (255, 0, 0),
			self::Color::YELLOW => (255, 255, 0),
			self::Color::GREEN => (0, 255, 0)
		}
	}
}

fn	main()
{
	// println!("\n\nExercise : Shape\n");
	
    // let circle = Shape::Circle(5.0);
    // let square = Shape::Square(10.0);
    // let triangle = Shape::Triangle(4.0, 6.0);
	
    // println!("Area of circle: {}", circle.calculates_area());
    // println!("Area of square: {}", square.calculates_area());
    // println!("Area of triangle: {}", triangle.calculates_area());
	// println!("\n\n");
	
	// println!("\n\n Exercise : Color\n");
	
    // let red = Color::RED;
    // let green = Color::GREEN;
    // let blue = Color::BLUE;
    // let yellow = Color::YELLOW;
	
    // println!("Area of RED: {:?}", red.rgb_value());
    // println!("Area of GREEN: {:?}", green.rgb_value());
    // println!("Area of BLUE: {:?}", blue.rgb_value());
    // println!("Area of YELLOW: {:?}", yellow.rgb_value());
	// println!("\n\n");

}
