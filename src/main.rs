use std::env;
use fluent_fallback::Localization;
use fluent_resmgr::ResourceManager;
use unic_langid::LanguageIdentifier;

fn main() {
    let cur_dir = env::current_dir().expect("Unable to find the current directory.").display().to_string();
    //println!("{}", cur_dir);

    let res_mgr = ResourceManager::new(cur_dir);
    let langid: LanguageIdentifier = "en-US".parse().expect("Failed to parse.");
    let loc = Localization::with_env(
        vec![
            "messages.ftl".into(),
        ],
        true,
        vec![langid],
        res_mgr,
    );
    let bundles = loc.bundles();
    let mut errors = vec![];

    println!("we get this far");
    let value = bundles.format_value_sync("hello-world", None, &mut errors).expect("Failed to format a value");
    println!("but we panic before we can get this far");

    assert_eq!(value, Some("Hello World [en]".into()));
}
