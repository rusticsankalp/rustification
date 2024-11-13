

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
fn main() 
{
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
