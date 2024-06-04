use clap::{command, Arg, ArgGroup, Command};

fn main() {
    let match_result = command!()
        .about("This application registers people with doctor's office")
        // .group(
        //     ArgGroup::new("person-register")
        //         .arg("first_name")
        //         .arg("last_name")
        // )
        // .group(
        //     ArgGroup::new("dog-register")
        //         .arg("pet_name")
        // )
        .subcommand(
            Command::new("register_person")
                .arg(Arg::new("first_name")
                         .short('f')
                         .long("first_name")
                         .aliases(["fname", "firstname"])
                         .required(true)
                         .help("The person's first name")
                     // .conflicts_with("last_name")
                )
                .arg(Arg::new("last_name")
                    .short('l')
                    .long("last_name")
                    .aliases(["lname", "lastname"])
                    .required(true)
                    .help("The person's last name")
                )
        )
        .subcommand(Command::new("register_pet")
            .arg(Arg::new("pet_name")
                .long("pet_name")
                .short('p')
                .help("Pet's name")
                .required(true)
            )
        )

        .arg(Arg::new("fluffy")
            .long("fluffy")
            .help("Is the person wearing a fluffy coat or not?")
        )
        .get_matches();

    println!("{}", match_result.get_one::<String>("fluffy").unwrap());
    if let Some(pet_args) = match_result.subcommand_matches("register_pet"){
        println!("Pet name = {}", pet_args.get_one::<String>("pet_name").unwrap());
    }

}
