use std::sync::Arc;

use ash::vk;
use smallvec::SmallVec;

use crate::sync::Semaphore;

pub struct Submission {
    pub(crate) wait_semaphores: Vec<Arc<Semaphore>>,
    pub(crate) command_buffers: Vec<vk::CommandBuffer>,
    pub(crate) signal_semaphores: Vec<Arc<Semaphore>>,
}

impl Submission {
    pub fn new() -> Self {
        Self { wait_semaphores: vec![], command_buffers: vec![], signal_semaphores: vec![] }
    }
    pub fn add_command_buffer(&mut self, command_buffer: vk::CommandBuffer) {
        self.command_buffers.push(command_buffer);
    }
    pub fn add_wait_semaphore(&mut self, wait: Arc<Semaphore>) {
        self.wait_semaphores.push(wait.clone());
    }
    pub fn add_signal_semaphore(&mut self, signal: Arc<Semaphore>) {
        self.signal_semaphores.push(signal.clone());
    }
    pub fn cached(submit: &[&Submission]) -> SubmissionCache {
        let mut submits: SmallVec<[vk::SubmitInfo; 4]> = SmallVec::with_capacity(submit.len());
        let mut waits: SmallVec<[SmallVec<[vk::Semaphore; 4]>; 4]> = SmallVec::new();
        let mut signals: SmallVec<[SmallVec<[vk::Semaphore; 4]>; 4]> = SmallVec::new();
        for submission in submit {
            waits.push(
                submission.wait_semaphores.iter().map(|value|{ value.semaphore }).collect::<SmallVec<_>>()
            );
            signals.push(
                submission.signal_semaphores.iter().map(|value|{ value.semaphore }).collect::<SmallVec<_>>()
            );
            submits.push(
                {
                    let wait = waits.last().unwrap();
                    let signal = signals.last().unwrap();
                    vk::SubmitInfo {
                        command_buffer_count: submission.command_buffers.len() as u32,
                        p_command_buffers: submission.command_buffers.as_ptr(),
                        wait_semaphore_count: wait.len() as u32,
                        p_wait_semaphores: wait.as_ptr(),
                        signal_semaphore_count: signal.len() as u32,
                        p_signal_semaphores: signal.as_ptr(),
                        ..Default::default()
                    }
                }
            )
        }
        SubmissionCache {
            submits,
            waits,
            signals
        }
    }
}
#[derive(Clone, Default)]
pub struct SubmissionCache {
    pub(crate) submits: SmallVec<[vk::SubmitInfo; 4]>, 
    pub(crate) waits: SmallVec<[SmallVec<[vk::Semaphore; 4]>; 4]>, 
    pub(crate) signals: SmallVec<[SmallVec<[vk::Semaphore; 4]>; 4]>
}
