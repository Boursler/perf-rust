pub mod main {
    use super::task::TaskMessage;
    //use crate::gui::events::perf::PerfEvent;
    use crate::events::perf::PerfEvent;
    use crate::state::*;
    /// Messages to be sent to the parent widget from
    /// other child widgets, and consumed on update
    #[derive(Debug, Clone)]
    pub enum Message {
        Loaded(Result<save_load::SavedState, save_load::LoadError>),
        Saved(Result<(), save_load::SaveError>),
        InputChanged(String),
        NewAppPressed,
        CommandSelected(PerfEvent),
        CyclesToggled(bool),
        InstructionsToggled(bool),
        ClockToggled(bool),
        CSToggled(bool),
        L1DCacheReadsToggled(bool),
        L1DCacheWritesToggled(bool),
        L1DCacheReadMissesToggled(bool),
        L1ICacheReadMissesToggled(bool),
        JsonToggled(bool),
        ListToggled(bool),
        VerboseToggled(bool),
        LaunchCommand,
        RecieveTask(usize, TaskMessage),
    }
}

pub mod task {
    #[derive(Debug, Clone)]
    pub enum TaskMessage {
        Run,
    }
}
