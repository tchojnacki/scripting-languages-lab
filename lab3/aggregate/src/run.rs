use crate::{aggregation::Aggregation, args::AggregateArgs};
use std::io::{self, BufRead};

pub fn parse_data_labels(args: &mut AggregateArgs) -> Result<(), &str> {
    if let Some(column_label) = &args.column_label {
        let group_labels = &args.group_label;

        if let Some(labels) = io::stdin().lock().lines().flatten().next() {
            let labels = labels.split(&args.separator).collect::<Vec<_>>();

            let column_index = labels.iter().position(|l| l == column_label);
            let group_indices = group_labels
                .iter()
                .map(|group| labels.iter().position(|l| l == group))
                .collect::<Option<Vec<_>>>();

            if column_index.is_none() || group_indices.is_none() {
                return Err("Unrecognized label found!");
            }

            args.column_index = column_index.unwrap();
            args.column_label = None;

            args.group_index = group_indices.unwrap();
            args.group_label = Vec::new();
        } else {
            return Err("No line containing labels found!");
        }
    }

    Ok(())
}

pub fn process(args: &AggregateArgs) {
    let mut aggregation = <dyn Aggregation>::from_string(&args.aggregation);

    for line in io::stdin().lock().lines().flatten() {
        let parts = line.split(&args.separator).collect::<Vec<_>>();

        if let Some(element) = parts
            .get(args.column_index)
            .and_then(|string| string.replace(',', ".").parse::<f64>().ok())
        {
            aggregation.consume(element);
        }
    }

    println!(
        "{}",
        aggregation
            .results()
            .map(|r| r.to_string())
            .unwrap_or_default()
    );
}
