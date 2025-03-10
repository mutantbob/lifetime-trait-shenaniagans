use std::ops::DerefMut;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct H266MergersPad {
    settings: Arc<Mutex<H266MergersPadSettings>>,
}

impl H266MergersPad {
    pub fn imp(&self) -> &Self {
        self
    }
}

//

pub struct H266MergersPadSettings {
    pub demo_guts: String,
}

pub trait IndexSettings<T: DerefMut<Target = H266MergersPadSettings>> {
    fn settings_at(&mut self, idx: usize) -> Option<T>;

    fn n_pads(&self) -> usize;
}

impl IndexSettings<&mut H266MergersPadSettings> for &mut [H266MergersPadSettings] {
    fn settings_at(&mut self, idx: usize) -> Option<&mut H266MergersPadSettings> {
        Some(&mut self[idx])
    }

    fn n_pads(&self) -> usize {
        self.len()
    }
}

impl IndexSettings<MutexGuard<H266MergersPadSettings>> for Vec<H266MergersPad> {
    fn settings_at(&mut self, idx: usize) -> Option<MutexGuard<H266MergersPadSettings>> {
        let pad: &H266MergersPad = &self[idx];
        if let Ok(settings) = pad.imp().settings.lock() {
            let s2: MutexGuard<'_, H266MergersPadSettings> = settings;
            return Some(s2);
        }
        None
    }

    fn n_pads(&self) -> usize {
        self.len()
    }
}
