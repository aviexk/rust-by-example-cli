use std::fs;
use std::io::{self, Write};
use std::process::Command;

#[derive(Debug, Clone)]
struct Exercise {
    title: String,
    description: String,
    initial_code: String,
    expected_output: Option<String>,
    hint: Option<String>,
    solution: Option<String>,
    instructions: Vec<String>,
}

#[derive(Debug, Clone)]
struct Chapter {
    title: String,
    description: String,
    exercises: Vec<Exercise>,
}

struct Tutorial {
    chapters: Vec<Chapter>,
    current_chapter: usize,
    current_exercise: usize,
    temp_file: String,
    current_code: String,
}

impl Tutorial {
    fn new() -> Self {
        let chapters = Self::create_chapters();
        let current_code = if !chapters.is_empty() && !chapters[0].exercises.is_empty() {
            chapters[0].exercises[0].initial_code.clone()
        } else {
            String::new()
        };
        
        Self {
            chapters,
            current_chapter: 0,
            current_exercise: 0,
            temp_file: "rusttutor_temp.rs".to_string(),
            current_code,
        }
    }

    fn create_chapters() -> Vec<Chapter> {
        vec![
            Chapter {
                title: "Hello World".to_string(),
                description: "Start with a traditional Hello World program.".to_string(),
                exercises: vec![
                    Exercise {
                        title: "Basic Hello World".to_string(),
                        description: "Write your first Rust program that prints 'Hello, World!' to the screen.".to_string(),
                        initial_code: r#"// This is a comment, and is ignored by the compiler
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter" shortcut

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function
fn main() {
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
}
"#.to_string(),
                        expected_output: Some("Hello World!".to_string()),
                        hint: Some("Use println! macro to print to console. Don't forget the exclamation mark!".to_string()),
                        solution: Some(r#"fn main() {
    println!("Hello World!");
}"#.to_string()),
                        instructions: vec![
                            "1. Look at the code above - it's a basic Rust program".to_string(),
                            "2. The main() function is where your program starts".to_string(),
                            "3. println! is a macro (note the !) that prints to the console".to_string(),
                            "4. Try changing the text inside the quotes".to_string(),
                            "5. Press 'r' to run the code and see the output".to_string(),
                        ],
                    },
                    Exercise {
                        title: "Comments and Documentation".to_string(),
                        description: "Learn about different types of comments in Rust.".to_string(),
                        initial_code: r#"fn main() {
    // This is a line comment
    /* This is a block comment */
    
    /// This is a documentation comment
    /// Add your own comment here and print something
    
    // TODO: Add a println! statement below
    
}"#.to_string(),
                        expected_output: Some("I can comment!".to_string()),
                        hint: Some("Add println!(\"I can comment!\"); in the main function".to_string()),
                        solution: Some(r#"fn main() {
    // This is a line comment
    /* This is a block comment */
    
    /// This is a documentation comment
    /// Add your own comment here and print something
    
    // TODO: Add a println! statement below
    println!("I can comment!");
}"#.to_string()),
                        instructions: vec![
                            "1. Notice the different comment styles in Rust".to_string(),
                            "2. // for single line comments".to_string(),
                            "3. /* */ for block comments".to_string(),
                            "4. /// for documentation comments".to_string(),
                            "5. Add a println! statement to make the program output 'I can comment!'".to_string(),
                        ],
                    },
                ],
            },
            Chapter {
                title: "Primitives".to_string(),
                description: "Learn about signed integers, unsigned integers and other primitives.".to_string(),
                exercises: vec![
                    Exercise {
                        title: "Scalar Types".to_string(),
                        description: "Explore Rust's scalar types: integers, floats, booleans, and characters.".to_string(),
                        initial_code: r#"fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
    // mutable = true;
    
    // Variables can be overwritten with shadowing.
    let mutable = true;
    
    // TODO: Print all the variables above
    
}"#.to_string(),
                        expected_output: Some("logical: true\na_float: 1\nan_integer: 5\ndefault_float: 3\ndefault_integer: 7\ninferred_type: 4294967296\nmutable: 21\nmutable: true".to_string()),
                        hint: Some("Use println! with {} placeholders or println!(\"variable: {}\", variable)".to_string()),
                        solution: Some(r#"fn main() {
    // Variables can be type annotated.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // Regular annotation
    let an_integer   = 5i32; // Suffix annotation

    // Or a default will be used.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`
    
    // A type can also be inferred from context 
    let mut inferred_type = 12; // Type i64 is inferred from another line
    inferred_type = 4294967296i64;
    
    // A mutable variable's value can be changed.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    
    // Error! The type of a variable can't be changed.
    // mutable = true;
    
    // Variables can be overwritten with shadowing.
    let mutable = true;
    
    // Print all the variables above
    println!("logical: {}", logical);
    println!("a_float: {}", a_float);
    println!("an_integer: {}", an_integer);
    println!("default_float: {}", default_float);
    println!("default_integer: {}", default_integer);
    println!("inferred_type: {}", inferred_type);
    println!("mutable: {}", mutable);
}"#.to_string()),
                        instructions: vec![
                            "1. Rust has several primitive types".to_string(),
                            "2. Variables are immutable by default - use 'mut' to make them mutable".to_string(),
                            "3. Type annotations can be explicit or inferred".to_string(),
                            "4. Shadowing allows reusing variable names".to_string(),
                            "5. Add println! statements to print all the variables".to_string(),
                        ],
                    },
                ],
            },
            Chapter {
                title: "Custom Types".to_string(),
                description: "Learn about structs and enums.".to_string(),
                exercises: vec![
                    Exercise {
                        title: "Structures".to_string(),
                        description: "Define and use custom structs.".to_string(),
                        initial_code: r#"// A struct with named fields
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);
    
    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

// TODO: Add #[derive(Debug)] to Person struct to make it printable
"#.to_string(),
                        expected_output: Some("Person { name: \"Peter\", age: 27 }\npair contains 1 and 0.1\npair contains 1 and 0.1".to_string()),
                        hint: Some("Add #[derive(Debug)] above the Person struct definition".to_string()),
                        solution: Some(r#"// A struct with named fields
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);
    
    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}"#.to_string()),
                        instructions: vec![
                            "1. Structs are custom data types that group related data".to_string(),
                            "2. There are three types: classic C-style structs, tuple structs, and unit structs".to_string(),
                            "3. {:?} is used for debug printing".to_string(),
                            "4. #[derive(Debug)] allows a struct to be printed with {:?}".to_string(),
                            "5. Add #[derive(Debug)] above the Person struct to make it printable".to_string(),
                        ],
                    },
                ],
            },
            Chapter {
                title: "Variable Bindings".to_string(),
                description: "Learn about mutable bindings, scope, and shadowing.".to_string(),
                exercises: vec![
                    Exercise {
                        title: "Mutability".to_string(),
                        description: "Understand mutable and immutable bindings.".to_string(),
                        initial_code: r#"fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error! Cannot assign a new value to an immutable variable
    // _immutable_binding += 1;
    
    // TODO: Create a mutable variable called 'counter' with initial value 0
    // TODO: Increment it by 5 and print the result
    
}"#.to_string(),
                        expected_output: Some("Before mutation: 1\nAfter mutation: 2\nCounter: 5".to_string()),
                        hint: Some("Use let mut counter = 0; then counter += 5; then println!(\"Counter: {}\", counter);".to_string()),
                        solution: Some(r#"fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Error! Cannot assign a new value to an immutable variable
    // _immutable_binding += 1;
    
    // Create a mutable variable called 'counter' with initial value 0
    let mut counter = 0;
    // Increment it by 5 and print the result
    counter += 5;
    println!("Counter: {}", counter);
}"#.to_string()),
                        instructions: vec![
                            "1. Variables are immutable by default in Rust".to_string(),
                            "2. Use 'mut' keyword to make variables mutable".to_string(),
                            "3. Immutable variables cannot be changed after assignment".to_string(),
                            "4. Mutable variables can be modified".to_string(),
                            "5. Create a mutable counter variable and increment it by 5".to_string(),
                        ],
                    },
                ],
            },
            Chapter {
                title: "Functions".to_string(),
                description: "Learn about functions, methods, and closures.".to_string(),
                exercises: vec![
                    Exercise {
                        title: "Basic Functions".to_string(),
                        description: "Define and call functions with parameters and return values.".to_string(),
                        initial_code: r#"// Unlike C/C++, there's no restriction on the order of function definitions
fn main() {
    // We can use this function here, and define it somewhere later
    fizzbuzz_to(100);
}

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the last expression is returned
    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the signature
fn fizzbuzz_to(n: u32) {
    for i in 1..=n {
        fizzbuzz(i);
    }
}

// TODO: Create a function called 'add' that takes two i32 parameters and returns their sum
// TODO: Call this function in main with values 5 and 3, and print the result
"#.to_string(),
                        expected_output: Some("1\n2\nfizz\n4\nbuzz\nfizz\n7\n8\nfizz\nbuzz\n11\nfizz\n13\n14\nfizzbuzz\n16\n17\nfizz\n19\nbuzz\nfizz\n22\n23\nfizz\nbuzz\n26\nfizz\n28\n29\nfizzbuzz\n31\n32\nfizz\n34\nbuzz\nfizz\n37\n38\nfizz\nbuzz\n41\nfizz\n43\n44\nfizzbuzz\n46\n47\nfizz\n49\nbuzz\nfizz\n52\n53\nfizz\nbuzz\n56\nfizz\n58\n59\nfizzbuzz\n61\n62\nfizz\n64\nbuzz\nfizz\n67\n68\nfizz\nbuzz\n71\nfizz\n73\n74\nfizzbuzz\n76\n77\nfizz\n79\nbuzz\nfizz\n82\n83\nfizz\nbuzz\n86\nfizz\n88\n89\nfizzbuzz\n91\n92\nfizz\n94\nbuzz\nfizz\n97\n98\nfizz\nbuzz\nSum: 8".to_string()),
                        hint: Some("Create fn add(a: i32, b: i32) -> i32 { a + b } and call it in main".to_string()),
                        solution: Some(r#"// Unlike C/C++, there's no restriction on the order of function definitions
fn main() {
    // We can use this function here, and define it somewhere later
    fizzbuzz_to(100);
    
    // Call the add function and print the result
    let result = add(5, 3);
    println!("Sum: {}", result);
}

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the last expression is returned
    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the signature
fn fizzbuzz_to(n: u32) {
    for i in 1..=n {
        fizzbuzz(i);
    }
}

// Function that takes two i32 parameters and returns their sum
fn add(a: i32, b: i32) -> i32 {
    a + b
}"#.to_string()),
                        instructions: vec![
                            "1. Functions are defined with the 'fn' keyword".to_string(),
                            "2. Parameters are specified with name: type".to_string(),
                            "3. Return type is specified with -> type".to_string(),
                            "4. The last expression in a function is returned (no semicolon)".to_string(),
                            "5. Create an 'add' function that takes two i32s and returns their sum".to_string(),
                        ],
                    },
                ],
            },
        ]
    }

    fn run(&mut self) {
        println!("\x1b[2J\x1b[H"); // Clear screen
        println!("🦀 Welcome to RustTutor - Interactive Rust Learning!");
        println!("=================================================");
        println!("Based on Rust by Example (https://doc.rust-lang.org/rust-by-example/)");
        println!("Inspired by vimtutor\n");
        
        println!("This is an interactive tutorial. You'll be guided through each exercise step by step.");
        println!("Press Enter to start your first exercise, or type 'help' for commands.\n");
        
        loop {
            // Interactive exercise flow
            self.interactive_exercise_flow();
            
            // Command mode
            print!("\nrusttutor> ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            
            match input {
                "q" | "quit" => {
                    println!("Happy coding with Rust! 🦀");
                    break;
                }
                "h" | "help" => self.print_help(),
                "l" | "list" => self.list_chapters(),
                "r" | "run" => self.run_current_exercise(),
                "s" | "solution" => self.show_solution(),
                "hint" => self.show_hint(),
                "n" | "next" => self.next_exercise(),
                "p" | "prev" => self.prev_exercise(),
                "edit" => self.edit_exercise(),
                "reset" => self.reset_exercise(),
                "restart" => self.restart_exercise(),
                cmd if cmd.starts_with("goto ") => {
                    let parts: Vec<&str> = cmd.split_whitespace().collect();
                    if parts.len() == 3 {
                        if let (Ok(chapter), Ok(exercise)) = (parts[1].parse::<usize>(), parts[2].parse::<usize>()) {
                            self.goto_exercise(chapter, exercise);
                        } else {
                            println!("Invalid chapter or exercise number!");
                        }
                    } else {
                        println!("Usage: goto <chapter> <exercise>");
                    }
                }
                "" => continue, // Just pressed enter, restart the interactive flow
                _ => println!("Unknown command. Type 'h' for help."),
            }
        }
    }

    fn print_help(&self) {
        println!("\n📖 RustTutor Commands:");
        println!("  h, help      - Show this help");
        println!("  l, list      - List all chapters and exercises");
        println!("  r, run       - Run the current exercise");
        println!("  s, solution  - Show the solution");
        println!("  hint         - Show a hint");
        println!("  n, next      - Go to next exercise");
        println!("  p, prev      - Go to previous exercise");
        println!("  edit         - Edit the current exercise code in nvim");
        println!("  reset        - Reset exercise to initial state");
        println!("  restart      - Restart the interactive flow for current exercise");
        println!("  goto <c> <e> - Go to chapter c, exercise e");
        println!("  q, quit      - Exit rusttutor");
        println!("\n💡 Tip: Just press Enter to go through the interactive exercise flow!");
    }

    fn list_chapters(&self) {
        println!("\n📚 Available Chapters:");
        for (i, chapter) in self.chapters.iter().enumerate() {
            let current_marker = if i == self.current_chapter { "👉" } else { "  " };
            println!("{} Chapter {}: {}", current_marker, i + 1, chapter.title);
            for (j, exercise) in chapter.exercises.iter().enumerate() {
                let ex_marker = if i == self.current_chapter && j == self.current_exercise { "  👉" } else { "    " };
                println!("{}  {}.{} {}", ex_marker, i + 1, j + 1, exercise.title);
            }
        }
    }

    fn interactive_exercise_flow(&mut self) {
        // Step 1: Display chapter and exercise title
        self.display_chapter_and_exercise();
        self.wait_for_enter("Press Enter to see the description and instructions...");
        
        // Step 2: Show description and instructions
        self.display_description_and_instructions();
        self.wait_for_enter("Press Enter to open the code in nvim...");
        
        // Step 3: Open code in nvim
        self.edit_exercise();
        
        // Step 4: Ask what to do next
        self.post_edit_options();
    }
    
    fn display_chapter_and_exercise(&self) {
        println!("\x1b[2J\x1b[H"); // Clear screen
        let chapter = &self.chapters[self.current_chapter];
        let exercise = &chapter.exercises[self.current_exercise];
        
        println!("{}", "=".repeat(80));
        println!("📖 Chapter {}: {}", self.current_chapter + 1, chapter.title);
        println!("📝 Exercise {}.{}: {}", self.current_chapter + 1, self.current_exercise + 1, exercise.title);
        println!("{}", "=".repeat(80));
        
        // Show progress
        let total_exercises: usize = self.chapters.iter().map(|c| c.exercises.len()).sum();
        let current_exercise_num = self.chapters[..self.current_chapter].iter().map(|c| c.exercises.len()).sum::<usize>() + self.current_exercise + 1;
        println!("\n📊 Progress: Exercise {} of {}", current_exercise_num, total_exercises);
        
        // Show chapter description
        println!("\n📚 Chapter Overview: {}", chapter.description);
    }
    
    fn display_description_and_instructions(&self) {
        println!("\x1b[2J\x1b[H"); // Clear screen
        let chapter = &self.chapters[self.current_chapter];
        let exercise = &chapter.exercises[self.current_exercise];
        
        println!("{}", "=".repeat(80));
        println!("📝 Exercise {}.{}: {}", self.current_chapter + 1, self.current_exercise + 1, exercise.title);
        println!("{}", "=".repeat(80));
        
        println!("\n📋 Description:");
        println!("{}", exercise.description);
        
        if !exercise.instructions.is_empty() {
            println!("\n📋 Instructions:");
            for (i, instruction) in exercise.instructions.iter().enumerate() {
                println!("  {}. {}", i + 1, instruction.trim_start_matches(|c: char| c.is_ascii_digit() || c == '.' || c == ' '));
            }
        }
        
        if let Some(expected) = &exercise.expected_output {
            println!("\n🎯 Expected Output:");
            println!("{}", "─".repeat(30));
            println!("{}", expected);
            println!("{}", "─".repeat(30));
        }
        
        if exercise.hint.is_some() {
            println!("\n💭 Hint available - type 'hint' after editing to see it");
        }
    }
    
    fn wait_for_enter(&self, message: &str) {
        println!("\n{}", message);
        print!("👉 ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
    
    fn post_edit_options(&self) {
        println!("\n🔧 What would you like to do next?");
        println!("  r, run     - Run your code");
        println!("  hint       - Get a hint");
        println!("  s, solution - Show the solution");
        println!("  edit       - Edit the code again");
        println!("  n, next    - Move to next exercise");
        println!("  help       - Show all commands");
        println!("Or just press Enter to run your code!");
    }

    fn run_current_exercise(&self) {
        // Write current code to temporary file
        if let Err(e) = fs::write(&self.temp_file, &self.current_code) {
            println!("❌ Error writing to temporary file: {}", e);
            return;
        }
        
        println!("\n🔧 Compiling and running your code...");
        
        // Compile the code
        let compile_output = Command::new("rustc")
            .arg(&self.temp_file)
            .arg("-o")
            .arg("rusttutor_temp")
            .output();
        
        match compile_output {
            Ok(output) => {
                if output.status.success() {
                    println!("✅ Compilation successful!");
                    
                    // Run the compiled binary
                    let run_output = Command::new("./rusttutor_temp").output();
                    
                    match run_output {
                        Ok(run_result) => {
                            println!("\n📤 Your Output:");
                            println!("{}", "─".repeat(40));
                            let output_str = String::from_utf8_lossy(&run_result.stdout);
                            println!("{}", output_str);
                            println!("{}", "─".repeat(40));
                            
                            // Check against expected output
                            let chapter = &self.chapters[self.current_chapter];
                            let exercise = &chapter.exercises[self.current_exercise];
                            
                            if let Some(expected) = &exercise.expected_output {
                                let actual = output_str.trim().to_string();
                                if actual == *expected {
                                    println!("🎉 Perfect! Your output matches exactly!");
                                    println!("💡 You can now move to the next exercise with 'n' or 'next'");
                                } else {
                                    println!("🤔 Expected Output:");
                                    println!("{}", "─".repeat(40));
                                    println!("{}", expected);
                                    println!("{}", "─".repeat(40));
                                    println!("💭 Try again! Use 'edit' to modify your code or 'hint' for help.");
                                }
                            } else {
                                println!("✨ Code executed successfully!");
                            }
                        }
                        Err(e) => println!("❌ Error running program: {}", e),
                    }
                } else {
                    println!("❌ Compilation failed:");
                    println!("{}", String::from_utf8_lossy(&output.stderr));
                    println!("💭 Use 'edit' to fix the issues or 'hint' for help.");
                }
            }
            Err(e) => println!("❌ Error compiling: {}", e),
        }
        
        // Cleanup
        let _ = fs::remove_file("rusttutor_temp");
    }

    fn show_solution(&self) {
        let chapter = &self.chapters[self.current_chapter];
        let exercise = &chapter.exercises[self.current_exercise];
        
        if let Some(solution) = &exercise.solution {
            println!("\n💡 Solution:");
            println!("{}", "─".repeat(50));
            println!("{}", solution);
            println!("{}", "─".repeat(50));
        } else {
            println!("❌ No solution available for this exercise.");
        }
    }

    fn show_hint(&self) {
        let chapter = &self.chapters[self.current_chapter];
        let exercise = &chapter.exercises[self.current_exercise];
        
        if let Some(hint) = &exercise.hint {
            println!("\n💭 Hint: {}", hint);
        } else {
            println!("❌ No hint available for this exercise.");
        }
    }

    fn next_exercise(&mut self) {
        if self.current_exercise < self.chapters[self.current_chapter].exercises.len() - 1 {
            self.current_exercise += 1;
        } else if self.current_chapter < self.chapters.len() - 1 {
            self.current_chapter += 1;
            self.current_exercise = 0;
        } else {
            println!("🎓 Congratulations! You've completed all exercises!");
            return;
        }
        
        // Reset code to the new exercise's initial code
        let chapter = &self.chapters[self.current_chapter];
        let exercise = &chapter.exercises[self.current_exercise];
        self.current_code = exercise.initial_code.clone();
        
        println!("📖 Moved to next exercise!");
    }

    fn prev_exercise(&mut self) {
        if self.current_exercise > 0 {
            self.current_exercise -= 1;
        } else if self.current_chapter > 0 {
            self.current_chapter -= 1;
            self.current_exercise = self.chapters[self.current_chapter].exercises.len() - 1;
        } else {
            println!("📚 You're at the first exercise!");
            return;
        }
        
        // Reset code to the new exercise's initial code
        let chapter = &self.chapters[self.current_chapter];
        let exercise = &chapter.exercises[self.current_exercise];
        self.current_code = exercise.initial_code.clone();
        
        println!("📖 Moved to previous exercise!");
    }

    fn goto_exercise(&mut self, chapter: usize, exercise: usize) {
        if chapter == 0 || chapter > self.chapters.len() {
            println!("❌ Invalid chapter number. Use 1-{}", self.chapters.len());
            return;
        }
        
        let chapter_idx = chapter - 1;
        if exercise == 0 || exercise > self.chapters[chapter_idx].exercises.len() {
            println!("❌ Invalid exercise number. Use 1-{}", self.chapters[chapter_idx].exercises.len());
            return;
        }
        
        self.current_chapter = chapter_idx;
        self.current_exercise = exercise - 1;
        
        // Reset code to the new exercise's initial code
        let chapter = &self.chapters[self.current_chapter];
        let exercise = &chapter.exercises[self.current_exercise];
        self.current_code = exercise.initial_code.clone();
        
        println!("📍 Jumped to Chapter {:?}, Exercise {:?}", chapter, exercise);
    }

    fn edit_exercise(&mut self) {
        // Write current code to temporary file
        if let Err(e) = fs::write(&self.temp_file, &self.current_code) {
            println!("❌ Error writing to temporary file: {}", e);
            return;
        }
        
        println!("🔧 Opening nvim... Save and quit (:wq) when you're done editing.");
        println!("💡 The file will be automatically loaded with your code.");
        
        // Open nvim with the temporary file
        let nvim_result = Command::new("nvim")
            .arg(&self.temp_file)
            .status();
        
        match nvim_result {
            Ok(status) => {
                if status.success() {
                    // Read the modified code back
                    match fs::read_to_string(&self.temp_file) {
                        Ok(modified_code) => {
                            self.current_code = modified_code;
                            println!("✅ Code updated successfully!");
                            
                            // Show a preview of the changes
                            println!("\n📝 Your Current Code:");
                            println!("{}", "─".repeat(50));
                            println!("{}", self.current_code);
                            println!("{}", "─".repeat(50));
                        }
                        Err(e) => println!("❌ Error reading modified file: {}", e),
                    }
                } else {
                    println!("❌ nvim was closed without saving properly.");
                }
            }
            Err(e) => {
                println!("❌ Error opening nvim: {}", e);
                println!("💡 Make sure nvim is installed and in your PATH.");
                println!("    You can install it with: brew install neovim (macOS) or your package manager");
            }
        }
        
        // Clean up temp file
        let _ = fs::remove_file(&self.temp_file);
    }

    fn reset_exercise(&mut self) {
        let chapter = &self.chapters[self.current_chapter];
        let exercise = &chapter.exercises[self.current_exercise];
        self.current_code = exercise.initial_code.clone();
        println!("🔄 Exercise reset to initial state.");
    }
    
    fn restart_exercise(&mut self) {
        self.reset_exercise();
        println!("🔄 Restarting interactive flow for current exercise...");
    }
}

fn main() {
    // Check if Rust is installed
    if Command::new("rustc").arg("--version").output().is_err() {
        println!("❌ Rust compiler (rustc) not found!");
        println!("Please install Rust from https://rustup.rs/");
        return;
    }
    
    let mut tutorial = Tutorial::new();
    tutorial.run();
}
