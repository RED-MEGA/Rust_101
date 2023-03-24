/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: reben-ha <reben-ha@student.1337.ma>        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/03/23 23:53:05 by reben-ha          #+#    #+#             */
/*   Updated: 2023/03/24 00:01:21 by reben-ha         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use rand::Rng;

fn main() 
{

	let mut rng = rand::thread_rng();
	let random_number = rng.gen_range(1, 100);


	println!("Guess the number!");
	println!("number is {random_number}");
	// loop {
		
	// 	// take input from user
		
	// 	// compare input with random number
		
		
	// 	// if input is equal to random number, print "You win" using match statement
		
		
		
	// }
}
