mod args;
mod create_distribution;

use std::fs;
use std::path::Path;
use clap::{Arg, command, Command, Parser};
use dialoguer::Select;
use uuid::Uuid;
use crate::args::EntityType::User;
use crate::args::{CreateCommand, CreateDistribution, CreateSubcommand, CreateUser, EntityType, UserCommand, UserSubcommand, WebprovisionsArgs};

// fn main() {
//
//    let match_result = command!()
//        .subcommand(
//            Command::new("create")
//                .arg(
//                    Arg::new("resource").short('r')
//                )
//        )
//        .arg(
//          Arg::new("firstname").short('f')
//       )
//        .arg(
//           Arg::new("lastname").short('l')
//        )
//        .get_matches();
//
//     //println!("{}", match_result.get_one::<String>("pet-name").unwrap_or(&"NO PET NAME".to_string()))
//
//     let create_args = match_result.subcommand_matches("create");
//     println!("Does resource exist? {}", create_args.unwrap().get_one::<String>("resource").unwrap());
//     // let items = vec!["foo", "bar", "baz"];
//     //
//     // let selection = Select::new()
//     //     .with_prompt("What do you choose?")
//     //     .items(&items)
//     //     .interact()
//     //     .unwrap();
//     //
//     //
//     // println!("You chose: {}", items[selection]);
// }

fn create_user_action(data: &CreateUser) {
    println!("Creating user with name: {}", data.name)
}



fn handle_user_command(cmd: &UserCommand) {
    let test = &cmd.command;

    match test {
        UserSubcommand::Create(val) => {
            create_user_action(val);
        }
        _ => {}
    };
}

fn handle_create_command(cmd: &CreateCommand) {
    match &cmd.command {
        CreateSubcommand::Distribution(val) => {
            create_distribution::create_distribution_action(val);
        }
    }
}

fn main() {
    let args = WebprovisionsArgs::parse();

    println!("{:?}", args);

    let entity_type = args.entity_type;

    match entity_type {
        EntityType::User(cmd) => {
            handle_user_command(&cmd);
        },
        EntityType::Create(cmd) => {
            handle_create_command(&cmd);
        },
        _ => println!("Unmatched command"),
    };

    // if let EntityType::User(_) = entity_type {
    //     println!("yes!")
    // }

    // args.entity_type match {
    //     User()
    // }
}