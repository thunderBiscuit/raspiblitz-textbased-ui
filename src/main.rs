// examples at https://github.com/gyscos/cursive/tree/master/examples

use cursive::views::TextView;
use cursive::Cursive;

fn main() {
    let mut siv = Cursive::default();

    siv.load_toml(include_str!("./style.toml")).unwrap();

    // We can quit by pressing `q`
    siv.add_global_callback('q', Cursive::quit);

    // Add a simple view
    siv.add_layer(TextView::new(
        "\n  New Bitcoin Block!
        \n  Block Number:           603,965
  Timestamp:              2019-Nov-15 22:36 UTC  
  # of Transactions:      1,725
  Transaction Volume:     6409 BTC
        "
    ));

    // Run the event loop
    siv.run();
}
