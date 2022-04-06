use crate::{aggregation::Aggregation, args::AggregateArgs, util};
use std::{
    io::{self, BufRead},
    mem,
};

pub fn parse_data_header(args: &mut AggregateArgs) -> Result<(), &str> {
    if let Some(column_label) = mem::take(&mut args.column_label) {
        let group_labels = mem::take(&mut args.group_labels);

        if let Some(labels) = io::stdin().lock().lines().flatten().next() {
            let labels = labels.split(&args.separator).collect::<Vec<_>>();

            let column_index = labels.iter().position(|l| l == &column_label);
            let group_indices = group_labels
                .iter()
                .map(|group| labels.iter().position(|l| l == group))
                .collect::<Option<Vec<_>>>();

            if column_index.is_none() || group_indices.is_none() {
                return Err("Unrecognized label found!");
            }

            args.column_index = column_index.unwrap();
            args.group_indices = group_indices.unwrap();
        } else {
            return Err("No line containing labels found!");
        }
    }

    Ok(())
}

pub fn process(args: &AggregateArgs) {
    let mut aggregation = <dyn Aggregation>::from_string(&args.aggregation);

    let mut prev_groups: Option<Vec<String>> = None;

    for line in io::stdin().lock().lines().flatten() {
        if let Some(element) = line
            .split(&args.separator)
            .nth(args.column_index)
            .and_then(|string| string.replace(',', ".").parse::<f64>().ok())
        {
            let cur_groups =
                util::extract_group_contents(&line, &args.separator, &args.group_indices);

            if let Some(prev_groups) = prev_groups {
                if prev_groups != cur_groups {
                    util::print_result(aggregation.result(), &prev_groups, &args.separator);

                    aggregation = <dyn Aggregation>::from_string(&args.aggregation);
                }
            }

            aggregation.consume(element);

            prev_groups = Some(cur_groups);
        }
    }

    util::print_result(
        aggregation.result(),
        &prev_groups.unwrap_or_default(),
        &args.separator,
    )
}
