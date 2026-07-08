pub fn normalize_tags(tags: &[String]) -> Vec<String> {
    let mut out: Vec<String> = tags
        .iter()
        .flat_map(|t| t.split(','))
        .map(|t| t.trim().to_lowercase().replace(' ', "-"))
        .filter(|t| !t.is_empty())
        .collect();
    out.sort();
    out.dedup();
    out
}

pub fn tags_match(task_tags: &[String], filter_tags: &[String]) -> bool {
    let task_lower: Vec<String> = task_tags.iter().map(|t| t.to_lowercase().replace(' ', "-")).collect();
    filter_tags.iter().any(|f| {
        let f_norm = f.to_lowercase().replace(' ', "-");
        task_lower.contains(&f_norm)
    })
}
