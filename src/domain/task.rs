struct Task {
    title: String,
    description: String,
    hours_create: u32,
    date_create: String,
    category: String,
    tags: Vec<String>,
    date_limit: String,
    progress: u32,
    sub_tasks: Vec<String>,
}
