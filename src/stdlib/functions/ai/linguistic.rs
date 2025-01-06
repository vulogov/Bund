extern crate log;
use lingua::{LanguageDetector, LanguageDetectorBuilder};
use crate::stdlib::functions::ai::{NN, NNEntry, NNType, NNVal};
use rust_multistackvm::multistackvm::{VM};
use rust_dynamic::value::Value;
use lazy_static::lazy_static;
use std::sync::Mutex;
use easy_error::{Error, bail};

lazy_static! {
    static ref LANG_DETECTOR: Mutex<NRLanguage> = {
        let e: Mutex<NRLanguage> = Mutex::new(NRLanguage::init());
        e
    };
}

pub struct NRLanguage {
    d: LanguageDetector,
}

impl NRLanguage {
    fn new() -> Self {
        Self {
            d: LanguageDetectorBuilder::from_all_languages().with_preloaded_language_models().build(),
        }
    }
    pub fn init() -> NRLanguage {
        let res = NRLanguage::new();
        res
    }
}

pub fn languages_preload() {
    log::debug!("Pre-loading languages");
    let e = LANG_DETECTOR.lock().unwrap();
    drop(e);
}

fn detect_language(d: String) -> Result<String, Error> {
    let detector = LANG_DETECTOR.lock().unwrap();
    let detected_language = detector.d.detect_language_of(d);
    drop(detector);
    match detected_language {
        Some(lang) => return Result::Ok(format!("{}", lang).to_string()),
        _ => bail!("Can not detect language"),
    }
}

impl NNEntry {
    pub fn new_lang_classifier() -> Self {
        Self {
            id:     NNType::LangClassifier,
            nn:     NNVal::Null,
        }
    }
}


pub fn create_linguistic_classifier(vm: &mut VM, name: String, _conf: Value) -> Result<&mut VM, Error> {
    log::debug!("Create Language classifier named: {}", &name);

    let mut ai = NN.lock().unwrap();
    ai.insert(name.to_string(), NNEntry::new_lang_classifier());
    drop(ai);
    languages_preload();
    Ok(vm)
}

pub fn classify_linguistic_classifier(vm: &mut VM, name: String, input: Value) -> Result<&mut VM, Error> {
    let input_text = match input.cast_string() {
        Ok(input_text) => input_text,
        Err(err) => bail!("LINGUISTIC classify returns error on input: {}", err),
    };
    let ai = NN.lock().unwrap();
    if ai.contains_key(&name) {
        match detect_language(input_text) {
            Ok(lang) => {
                vm.stack.push(Value::from_string(lang));
            }
            Err(err) => {
                drop(ai);
                bail!("CLASSIFIERS.CLASSIFY returns: {}", err);
            }
        }
    } else {
        drop(ai);
        bail!("CLASSIFIERS.CLASSIFY not found classifier: {}", &name);
    }
    drop(ai);
    Ok(vm)
}
