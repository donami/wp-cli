use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct WebprovisionsArgs {
    // /// The first argument!
    // pub first_arg: String,
    // /// The second argument!
    // pub second_arg: String,
    // /// The third argument!
    // pub third_arg: String,

    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]
pub enum EntityType {
    User(UserCommand),
    Create(CreateCommand)
    // Video(VideoCommand),
    //
    // View(ViewCommand),
}

#[derive(Debug, Args)]
pub struct CreateCommand {
    #[clap(subcommand)]
    pub command: CreateSubcommand,
}

#[derive(Debug, Args)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: UserSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UserSubcommand {
    /// Creates a user
    Create(CreateUser),
    Test,
}

#[derive(Debug, Subcommand)]
pub enum CreateSubcommand {
    Distribution(CreateDistribution)
}

#[derive(Debug, Args)]
pub struct CreateUser {
    /// The name of the user
    pub name: String,
}

#[derive(Debug, Args)]
pub struct CreateDistribution {
    /// Name of the distribution
    pub name: String,
}
