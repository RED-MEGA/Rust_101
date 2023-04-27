/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   utils.rs                                           :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: reben-ha <reben-ha@student.1337.ma>        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/04/25 21:32:28 by reben-ha          #+#    #+#             */
/*   Updated: 2023/04/25 21:53:40 by reben-ha         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod utils
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
		

		pub fn	calculates_area(shape : Shape) -> f32
		{
				match shape {
					Shape::Circle(radius) => PI * radius * radius,
					Shape::Square(len)=> len * len,
					Shape::Triangle(base, height)=> base * height * 0.5,
				}
		}
	}
}

pub use utils::*;