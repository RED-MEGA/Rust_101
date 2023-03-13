/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: reben-ha <reben-ha@student.1337.ma>        +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/03/12 18:26:16 by reben-ha          #+#    #+#             */
/*   Updated: 2023/03/12 19:57:48 by reben-ha         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

fn  shadowing()
{
    let str = "hello";
    let str = str.len();
    /*
    You can use name of variable twice 
    and this can help you if you want to
    change type of variable 
    */
}

fn  mut_var()
{
    // You can't change number number in Immutable variable 
    let str;
    str = 2;
    str = 3;
    // You can change number number in mutable variable 
    let mut str;
    str = 2;
    str = 3;
}

fn main()
{
	shadowing();
	mut_var();
}