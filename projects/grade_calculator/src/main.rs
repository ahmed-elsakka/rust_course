use std;

const PASSING_GRADE: f32 = 60.0;
fn main() {
    println!("Enter your name: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");

    name = name.trim().to_string();

    let mut scores: [f32; 3] = [0.0, 0.0, 0.0];
    for i in 1..=3 {
        println!("Enter the grade of subject {}: ", i);
        let mut score_str = String::new();
        std::io::stdin().read_line(&mut score_str).expect("Failed to read line");

        let score: f32 = score_str.trim().parse().expect("Please enter a valid number");
        scores[i - 1] = score;
    }

    let average = calc_average(scores);
    let grade_char = calc_grade(average);

    println!("Name: {}", name);
    println!("Average score: {}", average);
    println!("Grade: {}", grade_char);
}

fn calc_average(scores: [f32; 3]) -> f32 {
    let mut result: f32 = 0.0;
    for i in 0..=2 {
        result += scores[i];
    }
    result / 3.0
}

fn calc_grade(grade: f32) -> char {
    if grade >= 90.0 {
        'A'
    } else if grade >= 80.0 {
        'B'
    } else if grade >= 70.0 {
        'C'
    } else if grade >= PASSING_GRADE {
        'D'
    } else {
        'F'
    }
}
