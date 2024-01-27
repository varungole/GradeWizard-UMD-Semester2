use std::io;

fn main()
{
    println!("Welcome to your grade calculator");

    println!("Please select the subject for which you want to check your grade!");
    println!("1.ENPM 612 || 2.ENPM 614 || 3.ENPM 809K");
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("You did not enter a choice!");
    
    let answer : i32 = choice.trim().parse().unwrap();

    if answer == 1
    {
        println!("ENPM 612");

        println!("Please enter your midterm marks");
        let mut midt = String::new();
        io::stdin().read_line(&mut midt).expect("Failed");
        let mid_term: f32 = midt.trim().parse().unwrap();

        println!("Please enter your final exam marks");
        let mut final_exam = String::new();
        io::stdin().read_line(&mut final_exam).expect("Failed");
        let final_exam : f32 = final_exam.trim().parse().unwrap();

        println!("Please enter your assignment1 marks");
        let mut ass_1 = String::new();
        io::stdin().read_line(&mut ass_1).expect("Failed");
        let assignment_1 : f32 = ass_1.trim().parse().unwrap();

        println!("Please enter your assignment2 marks");
        let mut ass_2 = String::new();
        io::stdin().read_line(&mut ass_2).expect("Failed");
        let assignment_2 : f32 = ass_2.trim().parse().unwrap();

        println!("Please enter your assignment3 marks");
        let mut ass_3 = String::new();
        io::stdin().read_line(&mut ass_3).expect("Failed");
        let assignment_3: f32 = ass_3.trim().parse().unwrap();

        println!("Please enter your research and project marks");
        let mut rp_pres = String::new();
        io::stdin().read_line(&mut rp_pres).expect("Failed");
        let rp_presentation : f32 = rp_pres.trim().parse().unwrap();
        

        let weight_midterm = 0.2;
        let weight_final_exam = 0.4;
        let weight_ass_1 = 0.1;
        let weight_ass_2 = 0.1;
        let weight_ass_3 = 0.1;
        let weight_rp_fp = 0.1;

        let mut sum = 0.0;

        sum = (weight_midterm* ((mid_term/30.0)*100.0)) + (weight_final_exam* ((final_exam/50.0)*100.0))+ (weight_ass_1* ((assignment_1/10.0)*100.0)) + 
        (weight_ass_2* ((assignment_2/10.0)*100.0)) + (weight_ass_3* ((assignment_3/10.0)*100.0)) + (weight_rp_fp* ((rp_presentation/10.0)*100.0));
        

        if sum >= 95.0
        {
            println!("You got an A+ : {}" , sum);
        }

        else if sum >= 90.0 && sum < 95.0
        {
            println!("You got an A : {}" , sum);
        }


        else if sum >= 85.0 && sum < 90.0
        {
            println!("You got a B+ : {}" , sum);
        }


        else if sum >= 80.0 && sum < 85.0
        {
            println!("You got a B : {}%" , sum);
        }

        else if sum >=75.0 && sum < 80.0
        {
            println!("You got a C+ : {}%" , sum);
        }

        else if sum >=70.0 && sum < 75.0
        {
            println!("You got a C : {}%" , sum);
        }

        else if sum >=65.0 && sum < 70.0
        {
            println!("You got a D+ : {}%" , sum);
        }

        else if sum >=60.0 && sum < 65.0
        {
            println!("You got a D : {}%" , sum);
        }

        else
        {
            println!("You failed king lmao");
        }
    }
    else if answer == 2
    {
        println!("ENPM 614");

        println!("Please enter your midterm marks");
        let mut midt = String::new();
        io::stdin().read_line(&mut midt).expect("Failed");
        let mid_term: f32 = midt.trim().parse().unwrap();

        println!("Please enter your final exam marks");
        let mut final_exam = String::new();
        io::stdin().read_line(&mut final_exam).expect("Failed");
        let final_exam : f32 = final_exam.trim().parse().unwrap();

        println!("Please enter your assignment1 marks");
        let mut ass_1 = String::new();
        io::stdin().read_line(&mut ass_1).expect("Failed");
        let assignment_1 : f32 = ass_1.trim().parse().unwrap();

        println!("Please enter your assignment2 marks");
        let mut ass_2 = String::new();
        io::stdin().read_line(&mut ass_2).expect("Failed");
        let assignment_2 : f32 = ass_2.trim().parse().unwrap();

        println!("Please enter your assignment3 marks");
        let mut ass_3 = String::new();
        io::stdin().read_line(&mut ass_3).expect("Failed");
        let assignment_3: f32 = ass_3.trim().parse().unwrap();

        println!("Please enter your research and project marks");
        let mut rp_pres = String::new();
        io::stdin().read_line(&mut rp_pres).expect("Failed");
        let rp_presentation : f32 = rp_pres.trim().parse().unwrap();
        

        let weight_midterm = 0.2;
        let weight_final_exam = 0.4;
        let weight_ass_1 = 0.1;
        let weight_ass_2 = 0.1;
        let weight_ass_3 = 0.1;
        let weight_rp_fp = 0.1;

        let mut sum = 0.0;

        sum = (weight_midterm* ((mid_term/30.0)*100.0)) + (weight_final_exam* ((final_exam/50.0)*100.0))+ (weight_ass_1* ((assignment_1/10.0)*100.0)) + 
        (weight_ass_2* ((assignment_2/10.0)*100.0)) + (weight_ass_3* ((assignment_3/10.0)*100.0)) + (weight_rp_fp* ((rp_presentation/10.0)*100.0));
        

        if sum >= 95.0
        {
            println!("You got an A+ : {}" , sum);
        }

        else if sum >= 90.0 && sum < 95.0
        {
            println!("You got an A : {}" , sum);
        }


        else if sum >= 85.0 && sum < 90.0
        {
            println!("You got a B+ : {}" , sum);
        }


        else if sum >= 80.0 && sum < 85.0
        {
            println!("You got a B : {}%" , sum);
        }

        else if sum >=75.0 && sum < 80.0
        {
            println!("You got a C+ : {}%" , sum);
        }

        else if sum >=70.0 && sum < 75.0
        {
            println!("You got a C : {}%" , sum);
        }

        else if sum >=65.0 && sum < 70.0
        {
            println!("You got a D+ : {}%" , sum);
        }

        else if sum >=60.0 && sum < 65.0
        {
            println!("You got a D : {}%" , sum);
        }

        else
        {
            println!("You failed king lmao");
        }
    }
    else if answer == 3
    {
        println!("ENPM 809K");


        println!("Please enter your final exam marks");
        let mut final_exam = String::new();
        io::stdin().read_line(&mut final_exam).expect("Failed");
        let final_exam : f32 = final_exam.trim().parse().unwrap();

        println!("Please enter your assignment 1 marks");
        let mut ass_1 = String::new();
        io::stdin().read_line(&mut ass_1).expect("Failed");
        let assignment_1 : f32 = ass_1.trim().parse().unwrap();

        println!("Please enter your assignment 2 marks");
        let mut ass_2 = String::new();
        io::stdin().read_line(&mut ass_2).expect("Failed");
        let assignment_2 : f32 = ass_2.trim().parse().unwrap();

        println!("Please enter your assignment 3 marks");
        let mut ass_3 = String::new();
        io::stdin().read_line(&mut ass_3).expect("Failed");
        let assignment_3: f32 = ass_3.trim().parse().unwrap();

        println!("Please enter your assignment 4 marks");
        let mut ass_4 = String::new();
        io::stdin().read_line(&mut ass_4).expect("Failed");
        let assignment_4: f32 = ass_4.trim().parse().unwrap();

        println!("Please enter your assignment 5 marks");
        let mut ass_5 = String::new();
        io::stdin().read_line(&mut ass_5).expect("Failed");
        let assignment_5: f32 = ass_5.trim().parse().unwrap();

        println!("Please enter your project 1 marks");
        let mut proj_1 = String::new();
        io::stdin().read_line(&mut proj_1).expect("Failed");
        let project_1 : f32 = proj_1.trim().parse().unwrap();

        println!("Please enter your project 2 marks");
        let mut proj_2 = String::new();
        io::stdin().read_line(&mut proj_2).expect("Failed");
        let project_2 : f32 = proj_2.trim().parse().unwrap();
        
        let weight_final_exam = 0.3;
        let weight_ass_1 = 0.06;
        let weight_ass_2 = 0.06;
        let weight_ass_3 = 0.06;
        let weight_ass_4 = 0.06;
        let weight_ass_5 = 0.06;
        let weight_project_1 = 0.2;
        let weight_project_2 = 0.2;


        let mut sum = 0.0;

        sum = (weight_final_exam* ((final_exam/50.0)*100.0))+ (weight_ass_1* ((assignment_1/10.0)*100.0)) + 
        (weight_ass_2* ((assignment_2/10.0)*100.0)) + (weight_ass_3* ((assignment_3/10.0)*100.0)) + (weight_ass_4* ((assignment_4/10.0)*100.0)) + (weight_ass_5 * ((assignment_5/10.0)*100.0))+   (weight_project_1* ((project_1/10.0)*100.0)) + (weight_project_2* ((project_2/10.0)*100.0));
        

        if sum >= 95.0
        {
            println!("You got an A+ : {}" , sum);
        }

        else if sum >= 90.0 && sum < 95.0
        {
            println!("You got an A : {}" , sum);
        }


        else if sum >= 85.0 && sum < 90.0
        {
            println!("You got a B+ : {}" , sum);
        }


        else if sum >= 80.0 && sum < 85.0
        {
            println!("You got a B : {}%" , sum);
        }

        else if sum >=75.0 && sum < 80.0
        {
            println!("You got a C+ : {}%" , sum);
        }

        else if sum >=70.0 && sum < 75.0
        {
            println!("You got a C : {}%" , sum);
        }

        else if sum >=65.0 && sum < 70.0
        {
            println!("You got a D+ : {}%" , sum);
        }

        else if sum >=60.0 && sum < 65.0
        {
            println!("You got a D : {}%" , sum);
        }

        else
        {
            println!("You failed king lmao");
        }
    }
    else
    {
        println!("Please select correct option!");
    }


}