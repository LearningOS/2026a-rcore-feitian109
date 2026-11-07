use alloc::vec;
use alloc::vec::Vec;

/// Deadlock detector for single group of resources (mutex, semaphore)
pub struct DeadlockDetector {
    /// available resources (res_id)
    avail: Vec<usize>,
    /// task allocated resources (tid, res_id)
    alloc: Vec<Vec<usize>>,
    /// task need resources (tid, res_id)
    need: Vec<Vec<usize>>,
}

impl DeadlockDetector {
    /// new detector
    pub fn new() -> Self {
        Self {
            avail: Vec::new(),
            alloc: Vec::new(),
            need: Vec::new(),
        }
    }

    /// extend list length
    fn extend(&mut self, tid: usize, res_id: usize) {
        // extend avail
        if self.avail.len() <= res_id {
            self.avail.resize(res_id + 1, 0);
        }
        // extend alloc
        if self.alloc.len() <= tid {
            self.alloc.resize(tid + 1, Vec::new());
        }
        if self.alloc[tid].len() <= res_id {
            self.alloc[tid].resize(res_id + 1, 0);
        }
        // extend need
        if self.need.len() <= tid {
            self.need.resize(tid + 1, Vec::new());
        }
        if self.need[tid].len() <= res_id {
            self.need[tid].resize(res_id + 1, 0);
        }
    }

    /// set avail resources
    pub fn set_res(&mut self, res_id: usize, val: usize) {
        self.extend(0, res_id);
        self.avail[res_id] = val;
    }

    /// add task need resource
    pub fn add_need(&mut self, tid: usize, res_id: usize) {
        self.extend(tid, res_id);
        self.need[tid][res_id] += 1;
    }

    /// remove task need resource
    pub fn remove_need(&mut self, tid: usize, res_id: usize) {
        self.extend(tid, res_id);
        self.need[tid][res_id] -= 1;
    }

    /// lock resource
    pub fn lock(&mut self, tid: usize, res_id: usize) {
        self.extend(tid, res_id);
        self.need[tid][res_id] -= 1;
        self.alloc[tid][res_id] += 1;
        self.avail[res_id] -= 1;
    }

    /// unlock resource
    pub fn unlock(&mut self, tid: usize, res_id: usize) {
        self.extend(tid, res_id);
        self.avail[res_id] += 1;
        self.alloc[tid][res_id] -= 1;
    }

    /// check safety
    pub fn is_safe(&self) -> bool {
        // step 1
        let m = self.need.len();
        let mut work = self.avail.clone();
        let mut finish = vec![false; m];
        // step 2
        loop {
            if let Some(i) = (0..m).into_iter().find(|&i| {
                !finish[i]
                    && self.need[i]
                        .iter()
                        .zip(work.iter())
                        .all(|(need, work)| need <= work)
            }) {
                // step 3
                for (work, alloc) in work.iter_mut().zip(self.alloc[i].iter()) {
                    *work += alloc;
                }
                finish[i] = true;
            } else {
                // step 4
                return finish.into_iter().all(|f| f);
            }
        }
    }
}
