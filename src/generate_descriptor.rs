// This will be used by Daniela to generate the descriptor
// You don't need to use this, you're not cool enough, sorry not sorry <3
use bdk::bitcoin::hashes::hex::FromHex;
use bdk::bitcoin::PublicKey;
use bdk::descriptor;
use bdk::descriptor::Descriptor;
use bdk::miniscript::descriptor::TapTree;
use bdk::miniscript::policy::Concrete;
use bdk::miniscript::DummyKey;
use std::str::FromStr;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let recovery_private_key = "cQjbLyRnFNXCaSwvCQaiqBoMDa24CeCiQ1miqp5NPpMWPFLzZCTB";

    // let keys = [
    //     // TODO: Paste keys here
    // ];

    // let keys_joined: String = keys
    //     .into_iter()
    //     .map(|k| format!("pk({})", k))
    //     .collect::<Vec<_>>()
    //     .join(",");
    // let first_policy_str = format!("thresh({},{})", keys.len() - 1, keys_joined);
    // let first_policy = Concrete::<String>::from_str(&first_policy_str)?.compile()?;
    // let first_tap_leaf = TapTree::Leaf(Arc::new(first_policy));

    // let second_policy_str = format!("and(older(100),pk({}))", recovery_private_key);
    // let second_policy = Concrete::<String>::from_str(&second_policy_str)?.compile()?;
    // let second_tap_leaf = TapTree::Leaf(Arc::new(second_policy));

    // let dummy_internal =
    //     "020202020202020202020202020202020202020202020202020202020202020202".to_string();
    // let tap_tree = TapTree::Tree(Arc::new(first_tap_leaf), Arc::new(second_tap_leaf));
    // let descriptor = Descriptor::new_tr(dummy_internal, Some(tap_tree))?;
    // println!("{}", descriptor);

    Ok(())
}
