use std::io;

pub fn cgpa_calc() {

    println!("How many courses do you have?");
    
    let mut num_of_courses = String::new();
    io::stdin()
        .read_line(&mut num_of_courses)
        .expect("Enter a valid student number");
    
    let num_of_courses: i32 = num_of_courses.trim()
                                            .parse()
                                            .expect("Please enter a valid number");
    println!("This is the number of courses: {}", num_of_courses);

    // let count: &i32 = &num_of_courses;

    let mut count = 0;
    let mut total_credit_load = 0;
    let mut total_grade_points  = 0;
    let remaining = 22;

    while count < num_of_courses {
        println!("Credit load left = {}", remaining-total_credit_load);

        // Course code 
        println!("Enter Course Code: ");
        let mut course_code = String::new();
        io::stdin() 
            .read_line(& mut course_code)
            .expect("Failed to read input");
        let _course_code = course_code.trim();
                                          

        // Course credit
        println!("Enter Course Credit: ");
        let mut course_credit = String::new();
        io::stdin() 
            .read_line(& mut course_credit)
            .expect("Failed to read input");
        let course_credit: i32 = course_credit.trim()
                                              .parse()
                                              .expect("Make enter a valid number");

        // Grade input
        println!("Enter Grade (A-F): ");
        let mut grade_score = String::new();
        io::stdin() 
            .read_line(& mut grade_score)
            .expect("Failed to read input");
        // .chars() turns a string slice into an iterator over its characters, .next() gets the first character from that iterator
        let grade_score = grade_score.trim()
                                     .chars()
                                     .next()
                                     .expect("No grade entered");

        let grade_point =  grade_calculation(grade_score);
        let grade_point: i32 = grade_point.trim()
                                     .parse()
                                     .expect("Invalid grade point");

        total_grade_points += grade_point * course_credit;
        total_credit_load += course_credit;
        count += 1;
                             
        if total_credit_load > remaining {
            panic!("Exceeded maximum credit load");
        }


        let cgpa = total_grade_points as f64 / total_credit_load as f64;
        let formatted_cgpa = format!("{:.2}", cgpa);
        println!("The overall cgpa of the {} number of courses is {}", num_of_courses, formatted_cgpa)
    }
}

fn grade_calculation(param: char) -> String {
    match param.to_ascii_uppercase() {
        'A' => String::from("5"),
        'B' => String::from("4"),
        'C' => String::from("3"),
        'D' => String::from("2"),
        'E' => String::from("1"),
        'F' => String::from("0"),
        _ => panic!("Invalid day")
    }
}