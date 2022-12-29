pub use crate::util::{ 
	Analyzer, 
	DefaultAnalyzer,
};

#[allow(unused)]
pub struct AnalyzerFactory {
}

impl AnalyzerFactory {
    #[allow(unused)]
    pub fn new() -> AnalyzerFactory {
        return AnalyzerFactory { };
    }

    pub fn create(&self) -> Box<dyn Analyzer> {
        return Box::new(DefaultAnalyzer::new());
    }
}
