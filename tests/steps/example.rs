use cucumber_rust::{t, Steps};
use cucumber_rust_example::{
    default_string, find_by_id, insert_data, interesting_appendage, some_async_stringer,
};
use sqlx::SqlitePool;

use crate::MyWorld;

pub fn steps() -> Steps<crate::MyWorld> {
    let mut steps: Steps<crate::MyWorld> = Steps::new();

    steps.given("a string with some particular value", |_world, _ctx| {
        MyWorld::SomeString(default_string())
    });

    steps.when(
        "I append a known suffix to the value",
        |world, _ctx| match world {
            MyWorld::SomeString(x) => MyWorld::SuffixedString(interesting_appendage(&x)),
            _ => panic!("Invalid world state"),
        },
    );

    steps.then_regex(r#"^that string is now equal to "(.*)"$"#, |world, ctx| {
        match world {
            MyWorld::SuffixedString(x) => assert_eq!(x, ctx.matches[1]),
            _ => panic!("Invalid world state"),
        }
        MyWorld::Nothing
    });

    steps.given_async(
        "some awkward async process to get a string",
        t!(|_world, _ctx| {
            let s = some_async_stringer().await;
            MyWorld::SomeString(s)
        }),
    );

    steps.given_async(
        "some tables being filled with data",
        t!(|world, ctx| {
            let pool = ctx.get::<SqlitePool>().unwrap();
            insert_data(pool).await;
            world
        }),
    );

    steps.when_async(
        "we check a database we have prefilled with a string",
        t!(|world, ctx| {
            let a = match world {
                MyWorld::SomeString(x) => x,
                _ => panic!("Invalid world state"),
            };

            let pool = ctx.get::<SqlitePool>().unwrap();
            let b = find_by_id(pool, 1).await.unwrap();
            MyWorld::TwoStrings(a, b.value)
        }),
    );

    steps.then("we find we somehow had the same string", |world, _| {
        match world {
            MyWorld::TwoStrings(a, b) => assert_eq!(a, b),
            _ => panic!("Invalid world state"),
        }
        MyWorld::Nothing
    });

    steps
}
