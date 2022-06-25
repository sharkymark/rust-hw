// rust program to calculate sales commissions

use std::env;
use std::io::Write;


fn main() {
    println!("\n\nHello Commissions World! (written in Rust, mtm 6-25-22)");

    let args: Vec<String> = env::args().collect();
    //println!("{:?}", args);

    if args.len() == 2 && &args[1] == "--help" {
        def_help();
        cl_help();
		return
    }

	if args.len() != 6 {
		println!("\n*OOPS* Houston, we have a problem - not enough input parameters. TYPE ./commissions --help for more info");
        cl_help();
		return
	}

    let pt = &args[1];
    let q = &args[2].parse::<f32>().unwrap();
    let v = &args[3].parse::<f32>().unwrap();
    let r = &args[4].parse::<f32>().unwrap();
    let a = &args[5].parse::<f32>().unwrap();
    let mut rt_input = 0.0;  

	if pt == "fixedrate" {
        let mut line = String::new();
		print!("\n\nEnter fixed rate(no % symbol needed): ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut line).expect("Error: Could not read rate input");
        rt_input = line.trim().to_string().parse::<f32>().unwrap();;
	}    

	println!("\n*INPUTS*");
	println!("Plan Type: {}",pt);
	if pt == "fixedrate" {
		println!("Fixed Rate: {}%",rt_input.to_string());
	}
	println!("Quota: ${}",q);
	println!("Previous Quota Attainment ${} | {:.1}%",a,a/q*100.0);
	println!("Variable Comp: ${}",v);
	println!("Revenue: ${}",r);

	println!("\n*CALCULATED*");

	if pt == "fixedrate" {
        // calc
		println!("Commission: ${}",r*rt_input/100.0);
	}

	if pt == "varrate" {
		// calc
		println!("Rate: (var comp/quota) {:.1}%",v/q*100.0);
		println!("Commission: ${}",r*v/q);
	}

	if pt == "leveraged" {
		// calc
        // 	attain := revenue / quota
	    // varcomp * attain, attain
		println!("Transactional Quota Attainment: {:.1}%",r/q*100.0);
		println!("Commission: ${}",r/q*v);
	}

    // attainment calc
    println!("Cummulative Quota Attainment: ${} and {:.1}%",a+r,(a+r)/q*100.0);
	println!("\n*RE-RUN for another commissions scenario*");
    cl_help();



}

fn cl_help() {
    println!("\nUsage: commissions <plan type> <quota> <variable comp> <revenue> <attainment to date>\nplan types: fixedrate, varrate, leveraged\n");
}

fn def_help() {
    println!("\nThis command line app can calculate 3 types of commission plans.");
	println!("\nFixed Rate (plan type: fixedrate) apply a specific percentage e.g., 10% to the revenue number to determine the commission amount.");
	println!("\nVariable Rate (plan type: varrate) apply a deterministic or variable percentage to the revenue number to determine the commission amount.  The rate is the annual variable compensation amount / annual quota.");
	println!("\nLeveraged (plan type: leveraged) takes the revenue / quota to determine a rate that is applied to the annual variable compensation amount to determine the commission amount.");
}