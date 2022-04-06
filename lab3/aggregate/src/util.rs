pub fn extract_group_contents(line: &str, separator: &str, group_indices: &[usize]) -> Vec<String> {
    line.split(separator)
        .enumerate()
        .filter_map(|(i, part)| {
            if group_indices.contains(&i) {
                Some(String::from(part))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

pub fn print_result(result: Option<f64>, group_labels: &[String], separator: &str) {
    let mut group_labels = group_labels.to_vec();
    group_labels.push(result.map(|r| r.to_string()).unwrap_or_default());

    println!("{}", group_labels.join(separator));
}
