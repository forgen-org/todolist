pub enum Message {
    AddTask { description: String },
    CompleteTask,
    DeleteTask,
    PauseTask,
    SkipTask,
    StartTask,
}
