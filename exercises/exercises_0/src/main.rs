/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: reben-ha <reben-ha@student.1337.ma>        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/04/23 14:11:36 by reben-ha          #+#    #+#             */
/*   Updated: 2023/04/23 14:42:18 by reben-ha         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[derive(Debug)]
enum Gender {
	Male,
	Female
}

struct Person
{
	name	: String,
	age		: i32,
	address	: String,
	gender	: Gender
}

impl  Person
{
	fn print_info(&self) {
		println!(
			"name : {}\nage : {}\naddress : {}",
			self.name, self.age, self.address
		);
	}

	fn init(name : String, age : i32, address : String, gender : Gender) -> Person
	{
		Person {
			name,
			age,
			address,
			gender
		}
	}
}

fn p_info(user : Person)
{
	println!(
		"name : {}\nage : {}\naddress : {}\ngender : {:#?}",
			user.name, user.age, user.address, user.gender
		);
}

fn	main()
{
	print!("\n\n\n\n");
	let user1 : Person = Person::init(String::from("Radouane Iben-hamou"), 18, String::from("Khouribga"), Gender::Male);

	p_info(user1);
}
