//{{{ OptUcXParam
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum OptUcXParam {
    #[default]
    Void,
    Help,
    RustVersionInfo,
    BaseDir(String),
}

impl ::std::str::FromStr for OptUcXParam {
    type Err = OptUcXParamParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let oc = match s {
            "void" => OptUcXParam::Void,
            "help" => OptUcXParam::Help,
            "rust-version-info" => OptUcXParam::RustVersionInfo,
            _ => {
                let bs = "base_dir=";
                if let Some(stripped) = s.strip_prefix(bs) {
                    OptUcXParam::BaseDir(stripped.to_string())
                } else {
                    let s = format!("can not parse '{s}'");
                    return Err(OptUcXParamParseError::new(s));
                }
            }
        };
        Ok(oc)
    }
}

impl ::std::fmt::Display for OptUcXParam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let s = match *self {
            OptUcXParam::Void => "void",
            OptUcXParam::Help => "help",
            OptUcXParam::RustVersionInfo => "rust-version-info",
            OptUcXParam::BaseDir(_) => "base_dir=",
        };
        write!(f, "{s}")
    }
}
//}}} OptUcXParam

//{{{ OptUcXParamParseError
#[derive(Debug)]
pub struct OptUcXParamParseError {
    desc: String,
}

impl OptUcXParamParseError {
    fn new(s: String) -> OptUcXParamParseError {
        OptUcXParamParseError { desc: s }
    }
}

impl ::std::fmt::Display for OptUcXParamParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.desc.fmt(f)
    }
}

impl ::std::error::Error for OptUcXParamParseError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
}
//}}} OptUcXParamParseError

