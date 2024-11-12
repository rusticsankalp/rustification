

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

fn main() 
{
    control_flow();
    scope();
}
