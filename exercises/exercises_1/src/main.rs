/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: reben-ha <reben-ha@student.1337.ma>        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/04/23 14:44:55 by reben-ha          #+#    #+#             */
/*   Updated: 2023/04/23 15:31:03 by reben-ha         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::f32::consts::PI;
enum Shape
{
	Circle(f32),
	Rectangle(f32, f32),
}

fn calcule_area(shape : Shape) -> f32
{
	match shape {
		Shape::Circle(r) => PI * r * r,
    	Shape::Rectangle(w, h) => w * h,
	}
}

fn main()
{
    println!("\n\n\nProgram Output :\n\n");

	println!("Rectangle : {:.3}", calcule_area(Shape::Rectangle(5.41, 9.60)));
	println!("Circle : {:.3}", calcule_area(Shape::Circle(23.3)));	
}
