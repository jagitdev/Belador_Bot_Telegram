struct Task {
    title: String,                  //titulo de la tarea
    description: String,            //descripción de la tarea
    hours_create: u32,              //horas de creación
    date_create: String,            //fecha de creación
    category: String,               //categoría de la tarea
    tags: Vec<String>,              //etiquetas de la tarea
    date_limit: String,             //fecha límite de la tarea
    progress: u32,                  //progreso de la tarea
    sub_tasks: Option<Vec<String>>, //subtareas de la tarea
}
