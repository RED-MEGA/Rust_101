/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: reben-ha <reben-ha@student.1337.ma>        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/04/25 17:49:25 by reben-ha          #+#    #+#             */
/*   Updated: 2023/04/25 21:54:46 by reben-ha         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

/* Exercise : Shape */


/* Exercise : Color */


// use utils::color_utils::*;

// use test_lib::utils::color_utils::*;
// use test_lib::utils::shape_utils::*;

use test_lib::test_lib::color_utils::*;
use test_lib::test_lib::shape_utils::*;

fn	main()
{
	println!("\n\nExercise : Shape\n");
	
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(10.0);
    let triangle = Shape::Triangle(4.0, 6.0);
	
    println!("Area of circle: {}", circle.calculates_area());
    println!("Area of square: {}", square.calculates_area());
    println!("Area of triangle: {}", triangle.calculates_area());
	println!("\n\n");
	
	println!("\n\n Exercise : Color\n");
	
    let red = Color::RED;
    let green = Color::GREEN;
    let blue = Color::BLUE;
    let yellow = Color::YELLOW;
	
    println!("Area of RED: {:?}", rgb_value(red));
    println!("Area of GREEN: {:?}", rgb_value(green));
    println!("Area of BLUE: {:?}", rgb_value(blue));
    println!("Area of YELLOW: {:?}", rgb_value(yellow));
	println!("\n\n");

}
