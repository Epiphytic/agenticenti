pub mod composer;
pub mod generated {
    pub mod prompts {
        include!(concat!(env!("OUT_DIR"), "/prompts.rs"));
    }
}
