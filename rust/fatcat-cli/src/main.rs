
use fatcat_openapi;
use serde_json;
use toml;
//use fatcat_openapi::Api;
use fatcat_openapi::ApiNoContext;
use structopt::StructOpt;
use swagger::{AuthData, ContextBuilder, EmptyContext, Has, Push, XSpanIdString};
use fatcat_openapi::ContextWrapperExt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case", about = "CLI interface to Fatcat API" )]
struct Opt {

    #[structopt(env = "FATCAT_API_HOST", default_value = "https://api.fatcat.wiki")]
    api_host: String,

    #[structopt(env = "FATCAT_API_AUTH_TOKEN", hide_env_values = true)]
    api_token: Option<String>,

    #[structopt(env = "FATCAT_SEARCH_HOST", default_value = "https://search.fatcat.wiki")]
    search_host: String,

    #[structopt(short)]
    verbose: bool,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Status,
    Get {
        #[structopt(long)]
        toml: bool,
    },
    //Update {},
    //Create {},
    //Delete {},
    //Edit
    //Editgroup
    //Download
    //Search
}

fn main() {
    let opt = Opt::from_args();

    let client = if opt.api_host.starts_with("https://") {
        // Using Simple HTTPS
        fatcat_openapi::client::Client::try_new_https(&opt.api_host).expect("Failed to create HTTPS client")
    } else if opt.api_host.starts_with("http://") {
        // Using HTTP
        fatcat_openapi::client::Client::try_new_http(&opt.api_host).expect("Failed to create HTTP client")
    } else {
        panic!("unsupported API Host prefix");
    };

    let context: swagger::make_context_ty!(
        ContextBuilder,
        EmptyContext,
        Option<AuthData>,
        XSpanIdString
    ) = swagger::make_context!(
        ContextBuilder,
        EmptyContext,
        None as Option<AuthData>,
        XSpanIdString::default()
    );

    let client = client.with_context(context);
    let mut rt = tokio::runtime::current_thread::Runtime::new().unwrap();

    match opt.cmd {
        Command::Get {toml} => {
            let result = rt.block_on(client.get_changelog_entry(789));
            match result {
                Ok(fatcat_openapi::GetChangelogEntryResponse::FoundChangelogEntry(ce)) => {
                    if toml {
                        let doc = toml::Value::try_from(&ce).unwrap();
                        println!("{}", doc)
                    } else {
                        println!("{}", serde_json::to_string(&ce).unwrap())
                    }
                },
                _ => println!("{:?}", result),
            }
        },
        Command::Status => {
            println!("All good!");
        },
    }
}
