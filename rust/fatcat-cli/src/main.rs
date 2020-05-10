
use fatcat_openapi;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case", about = "CLI interface to Fatcat API" )]
struct Opt {

    #[structopt(env = "FATCAT_API_HOST", default = "https://fatcat.wiki/v0"]
    api_host: String,

    #[structopt(env = "FATCAT_API_AUTH_TOKEN", hide_env_values = true)]
    api_token: Option<String>,

    #[structopt(env = "FATCAT_SEARCH_HOST", default = "https://search.fatcat.wiki"]
    search_host: String,

    #[structopt(short)]
    verbose: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Status {},
    Get {},
    //Update {},
    //Create {},
    //Delete {},
    //Edit
    //Editgroup
    //Download
    //Search
}

fn main() {
    println!("Hello, world!");
    let result = client.get_changelog_entry(789).wait();
    println!("{:?} (X-Span-ID: {:?})", result, client.context().x_span_id.clone().unwrap_or(String::from("<none>")));
}
