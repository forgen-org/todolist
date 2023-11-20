pub enum Message {
    AddTask { description: String },
    CompleteTask,
    DeleteTask,
    SkipTask,
    StartTask,
}
