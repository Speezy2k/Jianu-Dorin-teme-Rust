fn nr_prim(a : i32) -> bool
{
    let mut i = 3;
    if a == 2 {return true;}
    if a <= 1 || a % 2 == 0 {return false;}
    while i * i <= a
    {
        if a % i == 0
        {
            return false;
        }
        i += 2;
    }
    return true;
}

fn nr_coprime(mut a : i32, mut b : i32) -> bool
{
    if a == 1 || b == 1 {return true;}
    while b != 0
    {
        let r = a % b;
        a = b;
        b = r;
    }
    return a == 1;
}

fn sing()
{
    for i in (1..=99).rev()
    {
        println!("{} bottles of beer on the wall,", i);
        println!("{} bottles of beer.", i);
        println!("Take one down, pass it around,");
        if i != 1
        {
            println!("{} bottles of beer on the wall,", i-1);
        }
        else 
        {
            println!("No bottles of beer on the wall.");
        }
        println!();
    }
}

fn main() 
{
    for i in 0..=100
    {
        if nr_prim(i)
        {
            print!("{} ", i);
        }
    }

    println!();
    // for i in 1..=100
    // {
    //     for j in 1..=100
    //     {
    //         if nr_coprime(i , j)
    //         {
    //             println!("{} {}", i, j);
    //         }
    //     }
    // }

    // println!();
    sing();
}
