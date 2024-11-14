
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
            //Exit the loop when i is 7
            break;
        }
    }
}


fn main() 
{

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
