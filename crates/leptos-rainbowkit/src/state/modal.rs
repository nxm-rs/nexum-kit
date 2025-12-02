use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModalType {
    None,
    Connect,
    Account,
    Chain,
}

#[derive(Debug, Clone, Copy)]
pub struct ModalState {
    current: RwSignal<ModalType>,
}

impl ModalState {
    pub fn new() -> Self {
        Self {
            current: RwSignal::new(ModalType::None),
        }
    }

    pub fn open_connect(&self) {
        self.current.set(ModalType::Connect);
    }

    pub fn open_account(&self) {
        self.current.set(ModalType::Account);
    }

    pub fn open_chain(&self) {
        self.current.set(ModalType::Chain);
    }

    pub fn close(&self) {
        self.current.set(ModalType::None);
    }

    pub fn is_open(self, modal_type: ModalType) -> Signal<bool> {
        Signal::derive(move || self.current.get() == modal_type)
    }
}

// Context provider
pub fn provide_modal_state() -> ModalState {
    let state = ModalState::new();
    provide_context(state);
    state
}

pub fn use_modal_state() -> ModalState {
    expect_context::<ModalState>()
}
