extern crate regex;
extern crate rusqlite;

use history::history::Features;
use rusqlite::Connection;
use network::Network;

pub fn add_db_functions(db: &Connection) {
    let network = Network::default();
    db.create_scalar_function("nn_rank", 10, true, move |ctx| {
        let age_factor = ctx.get::<f64>(0)?;
        let length_factor = ctx.get::<f64>(1)?;
        let exit_factor = ctx.get::<f64>(2)?;
        let recent_failure_factor = ctx.get::<f64>(3)?;
        let selected_dir_factor = ctx.get::<f64>(4)?;
        let dir_factor = ctx.get::<f64>(5)?;
        let overlap_factor = ctx.get::<f64>(6)?;
        let immediate_overlap_factor = ctx.get::<f64>(7)?;
        let selected_occurrences_factor = ctx.get::<f64>(8)?;
        let occurrences_factor = ctx.get::<f64>(9)?;

        let features = Features {
            age_factor,
            length_factor,
            exit_factor,
            recent_failure_factor,
            selected_dir_factor,
            dir_factor,
            overlap_factor,
            immediate_overlap_factor,
            selected_occurrences_factor,
            occurrences_factor,
        };

        Ok(network.forward(&features))
    }).expect("Successful create_scalar_function");
}