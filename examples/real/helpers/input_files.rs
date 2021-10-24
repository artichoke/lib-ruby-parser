use bumpalo::{
    collections::{String, Vec},
    Bump,
};

#[derive(Clone)]
pub(crate) struct InputFile<'a> {
    pub bump: &'a Bump,
    pub filepath: String<'a>,
    pub code: Vec<'a, u8>,
}

impl<'a> InputFile<'a> {
    fn eval(bump: &'a Bump, code: Vec<'a, u8>) -> Self {
        Self {
            filepath: String::from_str_in("(eval)", bump),
            code,
            bump,
        }
    }
}

pub(crate) struct InputFiles<'a> {
    pub files: std::vec::Vec<InputFile<'a>>,
}

impl<'a> InputFiles<'a> {
    pub(crate) fn new_eval(bump: &'a Bump, code: Vec<'a, u8>) -> Self {
        Self {
            files: vec![InputFile::eval(bump, code)],
        }
    }

    pub(crate) fn new_pattern(bump: &'a Bump, pattern: &str) -> Self {
        let files = glob::glob(&pattern)
            .expect("invalid glob pattern")
            .map(|f| f.unwrap().to_str().unwrap().to_string())
            .map(|filepath| InputFile {
                code: Vec::from_iter_in(std::fs::read(&filepath).unwrap().into_iter(), bump),
                filepath: String::from_str_in(&filepath, bump),
                bump,
            })
            .collect::<std::vec::Vec<_>>();

        Self { files }
    }

    pub(crate) fn len(&self) -> usize {
        self.files.len()
    }

    pub(crate) fn into_iter(self) -> std::vec::IntoIter<InputFile<'a>> {
        self.files.into_iter()
    }

    pub(crate) fn repeat(&mut self, n: usize) -> Self {
        let desired_len = self.len() * n;
        let files = self
            .files
            .clone()
            .into_iter()
            .cycle()
            .take(desired_len)
            .collect();
        Self { files }
    }

    pub(crate) fn new(
        bump: &'a Bump,
        code_to_eval: &Option<std::string::String>,
        pattern: &Option<std::string::String>,
        repeat: &Option<usize>,
    ) -> Self {
        let repeat = repeat.to_owned().unwrap_or(1);

        if let Some(code_to_eval) = code_to_eval {
            let code_to_eval = Vec::from_iter_in(code_to_eval.as_bytes().iter().cloned(), bump);
            Self::new_eval(bump, code_to_eval)
        } else if let Some(pattern) = pattern {
            Self::new_pattern(bump, pattern)
        } else {
            eprintln!("Either code to eval or pattern must be provided");
            std::process::exit(1)
        }
        .repeat(repeat)
    }
}
