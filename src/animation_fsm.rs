use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum AnimationFSMState {
    Idle,
    CreateLayout,
    RunAnimation,
    UpdateData,
    DestroyLayout,
}

impl AnimationFSMState {
    pub fn next(&mut self) {
        *self = match self {
            AnimationFSMState::Idle => AnimationFSMState::CreateLayout,
            AnimationFSMState::CreateLayout => AnimationFSMState::RunAnimation,
            AnimationFSMState::RunAnimation => AnimationFSMState::UpdateData,
            AnimationFSMState::UpdateData => AnimationFSMState::DestroyLayout,
            AnimationFSMState::DestroyLayout => AnimationFSMState::Idle,
        }
    }
}

pub enum FSMCtrlMsg {
    ToIdle,
    Next,
}

pub struct AnimationFSM {
    state: AnimationFSMState,
    create_layout: Option<Box<dyn FnOnce(&mut Self) + Send + 'static>>,
    run_animation: Option<Box<dyn FnOnce(&mut Self) + Send + 'static>>,
    update_data: Option<Box<dyn FnOnce(&mut Self) + Send + 'static>>,
    destroy_layout: Option<Box<dyn FnOnce(&mut Self) + Send + 'static>>,
    pub sender: Sender<FSMCtrlMsg>,
    receiver: Receiver<FSMCtrlMsg>,
}

impl Default for AnimationFSM {
    fn default() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            state: AnimationFSMState::Idle,
            create_layout: None,
            run_animation: None,
            update_data: None,
            destroy_layout: None,
            sender,
            receiver,
        }
    }
}

impl AnimationFSM {
    pub fn set_create_layout<F>(&mut self, callback: F)
        where
            F: FnOnce(&mut Self) + Send + 'static,
    {
        self.create_layout = Some(Box::new(callback));
    }
    pub fn set_run_animation<F>(&mut self, callback: F)
        where
            F: FnOnce(&mut Self) + Send + 'static,
    {
        self.run_animation = Some(Box::new(callback));
    }
    pub fn set_update_data<F>(&mut self, callback: F)
        where
            F: FnOnce(&mut Self) + Send + 'static,
    {
        self.update_data = Some(Box::new(callback));
    }
    pub fn set_destroy_layout<F>(&mut self, callback: F)
        where
            F: FnOnce(&mut Self) + Send + 'static,
    {
        self.destroy_layout = Some(Box::new(callback));
    }
}

impl AnimationFSM {
    pub fn start(&mut self) {
        if self.state == AnimationFSMState::Idle {
            self.state.next();
        }
    }
    pub fn next(&mut self) {
        self.state.next();
    }
    pub fn run(&mut self) {
        match self.state {
            AnimationFSMState::Idle => {}
            AnimationFSMState::CreateLayout => {
                if let Some(f) = self.create_layout.take() {
                    f(self);
                }
            }
            AnimationFSMState::RunAnimation => {
                if let Some(f) = self.run_animation.take() {
                    f(self);
                }
            }
            AnimationFSMState::UpdateData => {
                if let Some(f) = self.update_data.take() {
                    f(self);
                }
            }
            AnimationFSMState::DestroyLayout => {
                if let Some(f) = self.destroy_layout.take() {
                    f(self);
                }
            }
        }
        match self.receiver.try_recv() {
            Ok(FSMCtrlMsg::ToIdle) => {
                self.state = AnimationFSMState::Idle;
                self.create_layout = None;
                self.run_animation = None;
                self.update_data = None;
                self.destroy_layout = None;
            }
            Ok(FSMCtrlMsg::Next) => {
                self.state.next();
            }
            Err(_) => {}
        }
    }
}
