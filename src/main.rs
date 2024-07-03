use std::{env, fs};
use fluent::{FluentBundle, FluentValue, FluentResource, FluentArgs};
//use fluent_fallback::Localization;
//use fluent_resmgr::ResourceManager;
use unic_langid::LanguageIdentifier;

fn main() {
    //let cur_dir = env::current_dir().expect("Unable to find the current directory.").display().to_string();
    //println!("{}", cur_dir);
/*
let ftl_string = String::from("
hello-world = Hello, world!
intro = Welcome, { $name }.
");
*/

    let ftl_string = fs::read_to_string("messages.ftl").expect("Failed to read file");
    let res_mgr = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");
    //let res_mgr = ResourceManager::new(cur_dir);
    let langid: LanguageIdentifier = "en-US".parse().expect("Failed to parse.");

    let mut bundle = FluentBundle::new(vec![langid]);

    bundle.add_resource(res_mgr).expect("Failed to add FTL resources to the bundle.");

    let msg = bundle.get_message("hello-world").expect("Message doesn't exist.");

/*
    let loc = Localization::with_env(
        vec![
            "messages.ftl".into(),
        ],
        true,
        vec![langid],
        res_mgr,
    );
    let bundles = loc.bundles();
*/
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(&pattern, None, &mut errors);
    assert_eq!(&value, "Hello, world!");
    let mut args = FluentArgs::new();
    args.set("name", FluentValue::from("Heather"));
    let msg = bundle.get_message("intro").expect("Message doesn't exist.");
    
    let mut errors = vec![];
    let pattern = msg.value().expect("Message has no value.");
    let value = bundle.format_pattern(&pattern, Some(&args), &mut errors);
    println!("{}", value);
    assert_eq!(value, "Welcome, \u{2068}Heather\u{2069}.");
/*
    println!("we get this far");
    let value = bundles.format_value_sync("hello-world", None, &mut errors).expect("Failed to format a value");
    println!("but we panic before we can get this far");

    assert_eq!(value, Some("Hello World [en]".into()));
*/
}
