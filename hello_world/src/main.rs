use std::os::windows::process;


fn scope()
{
    let x: i32 = 42;
    println!("x : {}",x);

    let x = "forty two";
    println!("x : {}",x);

    {
        let x = 42.5;
        println!("x :{}",x);
    }
}

fn control_flow()
{
    let proceed = true;
    if proceed {
        println!("proceed");
    } else {
        println!("stop");
    }

    let height = 200;
    if height < 150 {
        println!("short");
    } else if height < 170 {
        println!("average");
    } else {
        println!("tall");
    }
}

fn shadowing()
{
    let mut height = 200;

    height = height - 20;
    if height < 150 {
        println!("short");
    } else if height < 170 {
        println!("average");
    } else {
        println!("tall");
    }
}
/*
fn return_value()
{
    
    let mut height = 200;

    height = height - 20;
    let result = if height < 150 {
        "short";
    } else if height < 170 {
        "average";
    } else {
        "tall";
    };

    result
  
}
*/

fn loops()
{
    let mut x =1 ;
    loop {
        println!("x : {}",x);
        x += 1;
        if x == 5 {
            break;
        }
    }
}

fn conditional()
{
    let maybe_number = Some(42);

    if let Some(number) = maybe_number {
        println!("number : {}",number);
    }
    else {
        println!("not a number!");
    }
}

fn conditional2()
{
    let maybe_number: Option<Option<()>> = Some(None);

    if let Some(number) = maybe_number {
        println!("number : {:?}",number);
    }
    else {
        println!("not a number!");
    }
}


fn whileloop()
{
  let mut i =1;
  while i<5{
    println!("i = {}",i);
    i +=1;
  }
}

fn while2_input()
{
    let mut input = String::new();
    while input.trim() != "bye" {
        input.clear();
        println!("Please enter bye! to exit");
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        println!("You entered: {}",input);
    }
    println!("GoodBye!");

}

fn forloop()
{
    for i in (1..5).rev() {
        println!("i = {}",i);
    }
}


fn forloop_vector()
{
    let v= vec! {1,2,3};
    for n in v
    {
        println!("n = {}",n);
    }
}

fn break_continue()
{
    for i in 1..10{
        if i%2 == 0 {
            //Skip the even numbers
            continue;
        }
        println!("i = {}",i);

        if i >= 7 {
            //Exit the loop when i is 7 or more
            break;
        }
    }
}

fn match_control_flow()
{
    let name = "hello 2";

    match name{
        "good bye" => println!("Sorry to see you go!"),
        "hello" => println!("Hi, nice to meet you!"),
        _ => println!("I dont have a greeting for that")
    }

}

/////Unit functions wlice
fn process_numbers(numbers: &[i32])
{
    let mut sum = 0 ; 

    for number in numbers{
        sum += number;
    }

    println!("The sum of the numbers is :{}",sum);

    if(sum %2 == 0){
        println!("Sum is even");
        
    }
    else{
        println!("the sum is odd");
    }
}

// return vlaues
fn split_string(s:String, delimiter:char, field: usize) -> String {
    let parts :Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);

    //This would not complile
    //result.to_string()


    //This would compile but will panic
    //result.expect("oops! something went wrong").to_string();
     

    match result{
        Some(result) => result.to_string(),
        None=>("no result".to_string())
    }
}

fn sum (numbers :&[i32]) -> i32 {
    let mut result = 0;
    for number in numbers{
        result += number;
    } 

    result
}


//: what is this ?
#[derive(Debug)]
struct Person{
    first_name: String, 
    last_name: String,
    age: u8,
}

fn struct_example()
{
    let person = Person{
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 30,
    };

    println!("first name is :  {:?}",person.first_name);
}

#[derive(Debug)]
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User{
    fn new (username: &str, email: &str) -> Self{
        Self{
            username: username.to_string(),
            email: email.to_string(),
            sign_in_count: 0,
            active: true,
        }
    }

    fn deactivate(&mut self){
        self.active = false;
    }
}


fn main() 
{


    println!("struct_constructor_example"); 
    let mut user = User::new("johndoe","johndoe@john.com");
    user.deactivate();


    println!("struct_example"); 
    struct_example();

    println!("Person Struct {:?}",Person{
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 30,
    });

    println!("sum of numbers :: {}",sum(&[1,2,3]));
    println!("returning values :: {}", split_string("hello_world".to_string(),',',1));

    println!("Unit function slice sum");
    process_numbers(&[1,2,3,4,5]);

    println!("match_control_flow");
    match_control_flow();

    println!("break_continue");
    break_continue();

    println!("forloop vector");
    forloop_vector();

    println!("forloop");
    forloop();

    println!("while2 input");
   //while2_input();

    println!("while loop");
    whileloop();

    println!("conditional2");
    conditional2();

    println!("conditional");
    conditional();

    println!("loop");
    loops();
    //let result = return_value();
    //println!("result : {}",result);

    println!("shadowing");
    shadowing();
    control_flow();
    scope();
}
