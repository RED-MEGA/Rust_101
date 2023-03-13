/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: reben-ha <reben-ha@student.1337.ma>        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/03/13 04:05:09 by reben-ha          #+#    #+#             */
/*   Updated: 2023/03/13 04:14:44 by reben-ha         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// if Expressions

fn main()
{
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}

// Using if in a let Statement

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

// Repetition with Loops

fn main()
{
    loop {
        println!("again!");
    }
}

// Returning Values from Loops

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// Loop Labels to Disambiguate Between Multiple Loops

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

// Loop Labels to Disambiguate Between Multiple Loops

fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// Conditional Loops with while

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}


fn main() {
	let a = [10, 20, 30, 40, 50];
    let mut index = 0;
	
    while index < 5 {
		println!("the value is: {}", a[index]);
		
        index += 1;
    }
}

// Looping Through a Collection with for

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

// Summary
// You made it! This was a sizable chapter: you learned about variables, scalar and compound data types, functions, comments, if expressions, and loops! To practice with the concepts discussed in this chapter, try building programs to do the following:

// Convert temperatures between Fahrenheit and Celsius.
// Generate the nth Fibonacci number.
// Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” taking advantage of the repetition in the song.