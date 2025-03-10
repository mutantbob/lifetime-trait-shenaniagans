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

pub trait IndexSettings<'a, T: DerefMut<Target = H266MergersPadSettings>> {
    fn settings_at(&'a mut self, idx: usize) -> Option<T>;

    fn n_pads(&self) -> usize;
}

impl<'a> IndexSettings<'a, &'a mut H266MergersPadSettings> for &'a mut [H266MergersPadSettings] {
    fn settings_at(&'a mut self, idx: usize) -> Option<&'a mut H266MergersPadSettings> {
        Some(&mut self[idx])
    }

    fn n_pads(&self) -> usize {
        self.len()
    }
}

impl<'a> IndexSettings<'a, MutexGuard<'a, H266MergersPadSettings>> for Vec<H266MergersPad> {
    fn settings_at(&'a mut self, idx: usize) -> Option<MutexGuard<'a, H266MergersPadSettings>> {
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

pub fn foo<'a, T: DerefMut<Target = H266MergersPadSettings>>(
    mut pads: impl IndexSettings<'a, T> + 'a,
) {
    for i in 0..pads.n_pads() {
        let settings = pads.settings_at(i).unwrap();
        println!("{}", (*settings).demo_guts);
    }

    let _all: Vec<_> = (0..pads.n_pads())
        .map(|i| {
            let settings = pads.settings_at(i).unwrap();
            (*settings).demo_guts.clone()
        })
        .collect();
}
