fn main()
{
    //let mut firs_number = 0;
    //let mut second_number = 0;
    //println!("Podaj liczbę nr 1 :");
    //firs_number = std::io::stdin().read_line(&mut firs_number).unwrap();
    //read_line(&mut firs_number).expect("Failed to read from stdin");
    //second_number= std::io::stdin().read_line(&mut second_number).unwrap();
    //io::stdin::().read_line(&mut second_number).expect("Failed to read from stdin");

    let mut line = String::new();
    
    println!("Enter your name :");
    std::io::stdin().read_line(&mut line).unwrap();

    let wrote_number = line.to_string();

    let number: u32 = wrote_number.parse::<u32>().unwrap();

    println!("Hello , {}", line);
    println!("{}", number);


    //from_str::<u32>(my_str);
    
}