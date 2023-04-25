/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: reben-ha <reben-ha@student.1337.ma>        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/04/25 17:49:25 by reben-ha          #+#    #+#             */
/*   Updated: 2023/04/25 18:21:57 by reben-ha         ###   ########.fr       */
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

fn	main()
{
	println!("\n\n/* Exercise : Shape */\n");
	
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(10.0);
    let triangle = Shape::Triangle(4.0, 6.0);

    println!("Area of circle: {}", circle.calculates_area());
    println!("Area of square: {}", square.calculates_area());
    println!("Area of triangle: {}", triangle.calculates_area());
	println!("\n\n");
}
