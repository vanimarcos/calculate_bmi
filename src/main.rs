use std::io;
use calculate_ideal_weight::calculate_bmi::calculate_bmi;
use calculate_ideal_weight::height_gender::HeightGender;

fn main() {
    println!("This program calculate your BMI(Body Mass Index)");
    println!("Insert your gender(M for Male and F for Female):");

    let mut gender: String = String::new();
    io::stdin()
    .read_line(&mut gender)
    .expect("O genero não foi inserido");

    let gender: char = gender.trim().chars().nth(0).expect("Não foi possivel converter o genero inserido");
 
    println!("Insert your height:");

    let mut height: String = String::new();
    io::stdin()
    .read_line(&mut height)
    .expect("a altura não foi inserida");

    let height = height.trim().parse::<f32>().expect("Não foi possivel converter a altura inserida");

    let height_gender = HeightGender {
        gender: gender,
        height: height
    };

    let result:f32 = calculate_bmi(height_gender);

    println!("Your BMI is: {}", result);
}

