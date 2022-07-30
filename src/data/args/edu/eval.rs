use clap::{Args, Subcommand};

#[derive(Args)]
pub struct ListArgs {
  /// Semesters id
  #[clap(short)]
  pub seme: i32,
}

#[derive(Subcommand)]
pub enum EvalCommands {
  /// Get enable semesters
  Seme,

  /// Get the teaching evaluation list
  List(ListArgs),
}
