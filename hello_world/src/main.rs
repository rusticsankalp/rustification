

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

fn main() 
{
    scope();
}
