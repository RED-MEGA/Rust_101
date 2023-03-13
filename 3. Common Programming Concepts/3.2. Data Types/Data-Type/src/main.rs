/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: reben-ha <reben-ha@student.1337.ma>        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/03/13 03:14:43 by reben-ha          #+#    #+#             */
/*   Updated: 2023/03/13 03:16:47 by reben-ha         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn main()
{
	// Floating-Point Types

	let x = 2.0; // f64
	let y: f32 = 3.0; // f32

	// Numeric Operations
	
	// addition
	let sum = 5 + 10;

	// subtraction
	let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;
	
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
	
    // remainder
    let remainder = 43 % 5;
	
	// The Boolean Type
	let t = true;
	let f: bool = false; // with explicit type annotation

	// The Character Type
	let c = 'z';
	let z: char = 'ℤ'; // with explicit type annotation
	let heart_eyed_cat = '😻';

	// The Tuple Type

	let tup: (i32, f64, u8) = (500, 6.4, 1);
	let tup = (500, 6.4, 1);
	let (x, y, z) = tup;
	let five_hundred = x.0;
	let six_point_four = x.1;
	let one = x.2;

	// The Array Type
	let a = [1, 2, 3, 4, 5];

	let months = ["January", "February", "March", "April", "May", "June", "July",
	"August", "September", "October", "November", "December"];

	let a: [i32; 5] = [1, 2, 3, 4, 5];

	let a = [3; 5];

	// Accessing Array Elements

	let a = [1, 2, 3, 4, 5];
	let first = a[0];
	let second = a[1];
    let a = [1, 2, 3, 4, 5];
}
