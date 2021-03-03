use cucumber_rust::{async_trait, criteria::feature, futures::FutureExt, Context, Cucumber, World};
use cucumber_rust_example::{create_tables, drop_tables, init_db};
use sqlx::SqlitePool;
use std::convert::Infallible;

mod steps;

pub enum MyWorld {
    Nothing,
    SomeString(String),
    SuffixedString(String),
    TwoStrings(String, String),
}

#[async_trait(?Send)]
impl World for MyWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::Nothing)
    }
}

#[tokio::main]
async fn main() {
    let pool = init_db().await;

    Cucumber::<MyWorld>::new()
        // Specifies where our feature files exist
        .features(&["./features"])
        // Adds the implementation of our steps to the runner
        .steps(steps::example::steps())
        // Add some global context for all the tests, like databases.
        .context(Context::new().add(pool))
        // Add some lifecycle functions to manage our database nightmare
        .before(feature("Example feature"), |ctx| {
            let pool = ctx.get::<SqlitePool>().unwrap().clone();
            async move { create_tables(&pool).await }.boxed()
        })
        .after(feature("Example feature"), |ctx| {
            let pool = ctx.get::<SqlitePool>().unwrap().clone();
            async move { drop_tables(&pool).await }.boxed()
        })
        // Parses the command line arguments if passed
        .cli()
        // Runs the Cucumber tests and then exists
        .run_and_exit()
        .await
}
