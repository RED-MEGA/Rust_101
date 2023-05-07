# Rust 101
Journey of learn rust from zero to hero

-> [Rust-introduction](https://github.com/Alien-MEGA/Rust-introduction)

Rust is a systems programming language that is fast, reliable, and safe. It is designed to be memory-safe and thread-safe, making it ideal for building high-performance programs on multiple platforms.

# ****Common Programming Concepts****

- Mutability
    
    In Rust, mutability refers to the ability to change or modify the value of a variable.
    
    Variables can be declared as mutable or immutable. Immutable variables are read-only, meaning their values cannot be changed once they are assigned. On the other hand, mutable variables can have their values modified or changed.
    
    To declare a variable as mutable in Rust, you need to use the **`mut`** keyword when declaring the variable, like so:
    
    ```rust
    bashCopy code
    let mut my_variable = 5;
    ```
    
    In the above example, the variable **`my_variable`** is declared as mutable by using the **`mut`** keyword. This means that the value of **`my_variable`** can be changed later in the code, like so:
    
    ```rust
    makefileCopy code
    my_variable = 10;
    ```
    
    Immutable variables, on the other hand, are declared without the **`mut`** keyword, like so:
    
    ```rust
    bashCopy code
    let my_immutable_variable = 5;
    ```
    
    Attempting to modify an immutable variable will result in a compiler error.
    
    The ability to declare variables as mutable or immutable is an important feature of Rust, as it allows for greater control over data and helps prevent unexpected changes to values.
    
- Data Types
    
    Primitive Types:
    
    ```rust
    // Integer Type
    let age: i32 = 27;
    
    // Floating-point Type
    let height: f64 = 1.75;
    
    // Boolean Type
    let is_student: bool = true;
    
    // Character Type
    let first_initial: char = 'J';
    ```
    
    Compound Types:
    
    ```rust
    // Array Type
    let numbers: [i32; 4] = [1, 2, 3, 4];
    
    // Tuple Type
    let person: (String, i32, f64) = ("John".to_string(), 27, 1.75);
    
    // Struct Type
    struct Person {
        name: String,
        age: i32,
        height: f64,
    }
    
    let student = Person {
        name: "John".to_string(),
        age: 27,
        height: 1.75,
    };
    ```
    
    In the examples above, **`age`** is an integer, **`height`** is a floating-point number, **`is_student`** is a boolean, and **`first_initial`** is a character. The **`numbers`** variable is an array of integers, the **`person`** variable is a tuple containing a string, an integer, and a floating-point number, and the **`student`** variable is a struct containing a string, an integer, and a floating-point number.
    
- Function
    
    A function **is a named block of code that can be reused multiple times in a program. It takes input parameters, performs some operations on them, and optionally returns a result.**
    
    ```rust
    fn add_numbers(x: i32, y: i32) -> i32 {
        let result = x + y;
        result
    }
    ```
    
    In this example, **`fn`** is the keyword used to define a function in Rust. **`add_numbers`** is the name of the function. The parameters **`x`** and **`y`** are defined with their types as **`i32`** integers. **`-> i32`** specifies that the function returns an integer value of type **`i32`**. The function then adds the parameters **`x`** and **`y`** together and stores the result in a new variable called **`result`**, and finally returns **`result`**.
    
- Control Flow
    
    Control flow in Rust refers to the way a program decides which blocks of code to execute based on certain conditions or events. Rust provides several control flow statements, including **`if`**, **`else if`**, **`else`**, **`while`**, **`loop`**, **`for`**, and **`match`**.
    
    Here is an example of a simple program that uses control flow statements to determine whether a number is positive, negative, or zero:
    
    ```rust
    fn check_number(num: i32) {
        if num > 0 {
            println!("The number is positive.");
        } else if num < 0 {
            println!("The number is negative.");
        } else {
            println!("The number is zero.");
        }
    }
    ```
    
    In this example, **`if`**, **`else if`**, and **`else`** statements are used to check the value of the parameter **`num`**. If **`num`** is greater than 0, the program prints "The number is positive." If **`num`** is less than 0, the program prints "The number is negative." If **`num`** is equal to 0, the program prints "The number is zero."
    

# Ownership

ownership is a fundamental concept in Rust that governs how memory is managed by the program. In Rust, all values are owned by a variable, and the ownership of the value determines how it can be used and when it will be deallocated.

```rust
fn main() {
    let s = String::from("hello"); // s owns the string "hello"
    let t = s; // t now owns the string "hello", and s is no longer valid
    println!("{}", t); // prints "hello"
}
```

In this example, we create a new **`String`** value containing the text "hello" and assign it to the variable **`s`**. At this point, **`s`** is said to own the string. We then create a new variable **`t`** and assign it the value of **`s`**. At this point, **`t`** becomes the owner of the string, and **`s`** is no longer valid.

This transfer of ownership is enforced by Rust's ownership system, which ensures that a value can only have one owner at a time. This helps prevent common programming errors such as use-after-free bugs and data races.

When a variable goes out of scope in Rust, its owned value is automatically deallocated. This ensures that memory is always cleaned up correctly and prevents memory leaks.

# Struct and ****Enums, Pattern Matching****

Structs and Enums are fundamental data structures in Rust that are used to represent and manipulate data in a program. Pattern matching is a powerful language feature in Rust that allows you to match values against patterns and perform different actions based on the match.

Structs:
A struct is a custom data type that allows you to group together related data with different types into a single unit. Here's an example of a struct definition in Rust:

```rust
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("{} is {} years old", person.name, person.age);
}
```

In this example, we define a struct **`Person`** that contains two fields: a **`name`** field of type **`String`**, and an **`age`** field of type **`u32`**. We then create a new **`Person`** instance and assign it to a variable named **`person`**, and use dot notation to access its fields.

Enums:
An enum is a custom data type that allows you to define a set of possible values that a variable can take. Here's an example of an enum definition in Rust:

```rust
enum Color {
    Red,
    Green,
    Blue,
}

fn main() {
    let color = Color::Green;
    match color {
        Color::Red => println!("The color is red!"),
        Color::Green => println!("The color is green!"),
        Color::Blue => println!("The color is blue!"),
    }
}
```

In this example, we define an enum **`Color`** that has three possible variants: **`Red`**, **`Green`**, and **`Blue`**. We then create a new **`Color`** instance and assign it to a variable named **`color`**, and use a **`match`** statement to pattern match on its value and print a message based on the match.

Pattern matching:
Pattern matching is a language feature in Rust that allows you to match values against patterns and perform different actions based on the match. Here's an example of pattern matching in Rust:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Dime;
    let value = value_in_cents(coin);
    println!("The value of {:?} is {} cents", coin, value);
}
```

In this example, we define an enum **`Coin`** that represents different types of coins. We then define a function **`value_in_cents`** that takes a **`Coin`** value and returns its value in cents. We use a **`match`** statement to pattern-match the **`Coin`** value and return the corresponding value in cents.

# Common Collections

Rust provides several data structures that are commonly used in programming, including vectors, strings, and hash maps.

1. Vectors: Vectors are Rust's dynamic arrays, which can grow or shrink in size as needed. They are declared using the **`Vec`** keyword and can hold any type of data. Here's an example:

```
rustCopy code
let mut v = Vec::new(); // create a new, empty vector
v.push(1); // add an element to the vector
v.push(2);
v.push(3);
println!("{:?}", v); // prints "[1, 2, 3]"

```

In this example, we create a new vector **`v`** and add three integers to it using the **`push`** method. We then print the contents of the vector using Rust's **`debug`** formatting syntax.

1. Strings: Strings are collections of UTF-8 encoded characters in Rust. They are represented by the **`String`** type and can be created from string literals or other strings using the **`from`** method. Here's an example:

```
vbnetCopy code
let s1 = String::from("hello"); // create a new string from a string literal
let s2 = String::from("world");
let s3 = s1 + &s2; // concatenate s1 and s2 into a new string s3
println!("{}", s3); // prints "helloworld"

```

In this example, we create two new strings **`s1`** and **`s2`** using string literals. We then concatenate them into a new string **`s3`** using Rust's overloaded **`+`** operator, which concatenates two strings. Finally, we print the contents of **`s3`**.

1. Hash maps: Hash maps are Rust's associative arrays, which map keys to values. They are declared using the **`HashMap`** type and can hold any type of data for both keys and values. Here's an example:

```
rustCopy code
use std::collections::HashMap; // import the HashMap type

let mut scores = HashMap::new(); // create a new, empty hash map
scores.insert(String::from("Alice"), 10); // add a key-value pair to the hash map
scores.insert(String::from("Bob"), 5);
scores.insert(String::from("Charlie"), 8);
let alice_score = scores.get("Alice"); // retrieve a value from the hash map
println!("{:?}", alice_score); // prints "Some(10)"

```

In this example, we create a new hash map **`scores`** and add three key-value pairs to it using the **`insert`** method. We then retrieve the value associated with the key "Alice" using the **`get`** method, which returns an **`Option`** type. Finally, we print the value using Rust's **`debug`** formatting syntax.

# ****Error Handling****

In Rust, the **`Result`** type is used to handle errors that may occur during program execution. It's a way of indicating that a function can return either a successful value or an error, and the caller is responsible for handling both cases.

```rust
rustCopy code
fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err(String::from("division by zero"));
    }
    Ok(x / y)
}

fn main() {
    let x = 10;
    let y = 2;

    match divide(x, y) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
```

In this example, we define a function **`divide`** that takes two **`i32`** values and returns a **`Result`** with either an **`i32`** result or a **`String`** error message. If the **`y`** value is zero, the function returns an **`Err`** variant with a message indicating a division by zero error. Otherwise, it returns an **`Ok`** variant with the result of the division.

In the **`main`** function, we call **`divide`** with values **`10`** and **`2`**. Since the division is valid, the **`match`** statement matches the **`Ok`** variant and prints the result. If the division had failed, the **`match`** statement would have matched the **`Err`** variant and printed the error message.

Now, let's talk about **`panic!`**. In Rust, **`panic!`** is a macro that can be used to indicate a critical error that cannot be recovered from. When a **`panic!`** occurs, the program will terminate immediately and print an error message to the console.

Here's an example of using **`panic!`**:

```rust
rustCopy code
fn main() {
    let v = vec![1, 2, 3];
    let third = v[2];
    println!("The third element is {}", third);
}

```

In this example, we create a **`vec`** of integers and try to access the third element using index **`2`**. However, since **`v`** only contains three elements, this will result in a **`panic!`** and the program will terminate with an error message.

It's important to note that **`panic!`** is intended to be used only for critical errors that cannot be recovered from. In most cases, **`Result`** should be used to handle errors gracefully and provide informative error messages to the user.
