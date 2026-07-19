pub fn run() {
    //Match Arms
    //
    //math Value{
    //  Pattern => Expression,
    //  Pattern => Expression,
    //  Pattern => Expression,
    //  Pattern => Expression,
    //}
    //
    // match x {
    //     None => None,
    //     Some(i) => Some(i + 1),
    // }

    //let statements
    //
    //let x = 5;
    //let PATTERN = Expression
    //let (x, y, z) = (1, 2, 3);
    //
    //compile error if number of elements does not match number of elements in tuple
    //let (x, y) = (1, 2, 3);<error>
    //
    //Condtional if let Expression:

    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "35".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using pruple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    //while let Condtional Loops

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });
    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    //for Loops
    //In a for loop, the value that directly follows the keyboard for is a pattern. For exmple in
    //for x in y, the x is the pattern.

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
    //Function Parameters
    //function Parameters can also be patterns, the x part is a pattern As we did with let, we could
    //match a tuple in a functions arguments to the pattern.

    //irrefutable pattern
    //let x = 5;

    //refutable pattern
    //Some(x)
    //if let Some(x)= a_value;
    //
    //Matching literals
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anythings"),
    }
    //matching named variables

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Defualt case, x = {x:?}"),
    }

    println!("at the end x = {x:?}, y = {y}");
    //this prints:
    //Matched, y =5
    //at the end: x =Some(5), y = 10
    //
    //explanation: The pattern in the second match arm introduces a new variable named y that will
    //match any value inside a Some value. Because we're in a new scope inside the match expression,
    //this is a new y variable, not the y we declared at the beginning with the value 10... If x had
    //been a None value instead of Some(5), the patterns in the first two arms wouldnt have matched,
    //so the value would have matched to the underscore.
    //
    //
    //Matchign multiple patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //Matching Ranges of Values with ..=
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }
    //If x is 1, 2, 3, 4, or 5, the first arm will match. This syntax is more convenient for
    //multiple match values than using the | operator to express the same idea; if we were to use |,
    //we would have to specify 1 | 2 | 3 | 4 | 5 . Spcifying a range is much shorter, espcially if
    //we want to match, say, any number between 1 and 1,000!

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("somethign else"),
    }

    //Destructuring to Break Apart Values
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    //Destructuring

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    //explanation:
    //This Code creates the variables a and b that match the values of the x and y fields of the p
    //struct. This example shows that the names of the variables in the pattern dont have to match
    //the field names of the struct. However, it's common ot match the variable names to the field
    //names to make it easier to remember which variables came from which fields. Because of this
    //common usage and because writing let Point {x : x, y:y} = p; contains a lot of duplication,
    //Rust has a shorthand for patterns that match strct fields: You only need to list the numberame of
    //the struct field, and the variables created from the pattern will have the same names.

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y}");
        }
    }

    //this prints : On the y axis at 7
    //
    //Enums

    // let msg = Message::ChangeColor(0, 160, 255);
    // match msg {
    //     Message::Quit => {
    //         println!("The Quit varient has no data to destructure.");
    //     }
    //     Message::Move { x, y } => {
    //         println!("Move in the x direction {x} and in the y direction {y}");
    //     }
    //     Message::Write(text) => {
    //         println!("Text message: {text}");
    //     }
    //     Message::ChangeColor(r, g, b) => {
    //         println!("Change color to red {r}, green {g}, and blue {b}");
    //     }
    // }

    //this code will print:
    //Change color to red 0, green 160, and blue 255

    //Nested Structs and Enums

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }

    //Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    //We can mix match , and nest destructuring patterns in even more complex ways. The following
    //example shows a complicated destructure where we nest structs and tuples inside a tuple and
    //destructure all the primitive values out. This code lets us break complex types into their
    //components parts so that we can use the values we're interest in separately.

    //Ignoring Values in a Pattern
    //An Entire Value with _

    foo(3, 4);

    //Parts of a Value with a Nested _

    let mut seeting_value = Some(5);
    let new_setting_value = Some(10);

    match (seeting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwirte an existing customized value!");
        }
        _ => {
            seeting_value = new_setting_value;
        }
    }

    println!("setting is {seeting_value:?}");

    //this prints:
    //Can't overwrite an existing customized value
    //setting in Some(5)

    let numbers = (2, 4, 8, 16, 32);
    //clippy does not like this
    // match numbers {
    //     (first, _, third, _, fifth) => {
    //         println!("Some numbers: {first}, {third}, {fifth}");
    //     }
    // }
    //this will print:
    //Some numbers: 2, 8, 32

    //An Unused Variable by Starting its Name with _
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));
    //this panics:
    // if let (_s) = s {
    //     println!("found a string")
    // }
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{s:?}");

    //the second if statement workds because s does not get bind

    //Remaining Parts of a Value with ..
    let origin = ThreePoint { x: 0, y: 0, z: 0 };

    match origin {
        ThreePoint { x, .. } => println!("x is {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("SOme numbers: {first}, {last}");
        }
    }
    //this only works if the statement is unambiguous

    //Adding Conditionals with Match Guards
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
    //This will print:
    //The number 4 is even.

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");
    //This will print:
    //Default case, x = Some(5)
    //at the end: x = Some(5), y =10

    //another exmaple with the | operator
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"), //this is (4 | 5 | 6) && if y is true
        _ => println!("no"),
    }

    let msg = BindMessage::Hello { id: 5 };

    match msg {
        BindMessage::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        BindMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        BindMessage::Hello { id } => println!("Found some other id: {id}"),
    }

    //The example will print Found an id in range: 5. By specifying id @ before the range 3..=7,
    //we're capturing whatever value matched the range in a variable named id while also testing
    //thtat the value matched range pattern.
}

enum BindMessage {
    Hello { id: i32 },
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color), //changed this from (i32, i32, i32) to (Color)
}
struct Point {
    x: i32,
    y: i32,
}

struct ThreePoint {
    x: i32,
    y: i32,
    z: i32,
}
fn foo(x: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

// fn print_coordinates(&(x, y): &(i32, i32)) {
//     println!("Current location: ({x}, {y}");
// }
