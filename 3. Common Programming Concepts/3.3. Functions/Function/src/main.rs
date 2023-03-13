/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: reben-ha <reben-ha@student.1337.ma>        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/03/13 03:31:57 by reben-ha          #+#    #+#             */
/*   Updated: 2023/03/13 04:01:44 by reben-ha         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn another_function()
{
	print!("hi\n");
}

fn main()
{
    another_function();
}

// add Parameters

fn main() 
{
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) 
{
    println!("The measurement is: {value}{unit_label}");
}

// Statements and Expressions

fn main() 
{
    let y = 6;
}

// OR 

fn main()
{
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

// Functions with Return Values

fn five() -> i32
{
    5
}

fn main()
{
    let x = five();

    println!("The value of x is: {x}");
}

// OR 

fn main()
{
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32
{
    x + 1 // You need't semicolons
}
