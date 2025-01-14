use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]
pub struct GraphArgs {
    /// please provide the path to the graph file
    pub graph: String,
}
