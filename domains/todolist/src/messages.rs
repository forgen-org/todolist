pub enum Message {
    AddTask { description: String },
    CompleteTask,
    SkipTask,
    StartTask,
}
