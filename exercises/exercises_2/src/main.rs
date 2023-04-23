/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: reben-ha <reben-ha@student.1337.ma>        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/04/23 15:34:15 by reben-ha          #+#    #+#             */
/*   Updated: 2023/04/23 17:46:04 by reben-ha         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io::Read;
use std::io;
use std::process::exit;
use std::str::FromStr;

#[derive(Debug)]
struct DatabaseConnection 
{
	hostname : String,
	username : String,
	password : String,
	port : i32,
}

impl DatabaseConnection
{
	fn init(hostname : String, username : String) -> DatabaseConnection
	{
		DatabaseConnection
		{
			hostname,
			username,
			password : "".to_string(),
			port : 0,
		}	
	}
}

fn read_value<T: FromStr>(prompt: &str) -> T
{
	println!("{}", prompt);
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read from stdin");
	match input.trim().parse() {
		Ok(parsed) => parsed,
		Err(_) => {
			println!("Invalid input. Please enter an integer.");
			exit(1);
		}
	}
}

fn main()
{
    println!("\n\nProgram Start :\n\n");

	let mut connection_DB = DatabaseConnection::init
	(
		String::from("reben-ha42"),
		String::from("reben-ha"),
	);
	
	connection_DB.password = read_value("Enter an password : ");
	connection_DB.port = read_value("Enter an Port : ");
	println!("{:#?}", connection_DB);
} 
