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

pub trait IndexSettings {
    type Target<'a>: DerefMut<Target = H266MergersPadSettings>
    where
        Self: 'a;
    fn settings_at(&mut self, idx: usize) -> Option<Self::Target<'_>>;

    fn n_pads(&self) -> usize;
}

impl IndexSettings for &mut [H266MergersPadSettings] {
    type Target<'a>
        = &'a mut H266MergersPadSettings
    where
        Self: 'a;
    fn settings_at(&mut self, idx: usize) -> Option<Self::Target<'_>> {
        Some(&mut self[idx])
    }

    fn n_pads(&self) -> usize {
        self.len()
    }
}

impl IndexSettings for Vec<H266MergersPad> {
    type Target<'a>
        = MutexGuard<'a, H266MergersPadSettings>
    where
        Self: 'a;
    fn settings_at(&mut self, idx: usize) -> Option<Self::Target<'_>> {
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
