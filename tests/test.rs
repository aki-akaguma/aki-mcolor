const TARGET_EXE_PATH: &'static str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            indoc::indoc!(
                r#"
            Usage:
              aki-mcolor [options]

            mark up text with color

            Options:
              -r, --red <exp>       write it in red
              -g, --green <exp>     write it in green
              -b, --blue <exp>      write it in blue
              -c, --cyan <exp>      write it in cyan
              -m, --magenda <exp>   write it in magenda
              -y, --yellow <exp>    write it in yellow
              -u, --unmark <exp>    write it in non-color

              -H, --help        display this help and exit
              -V, --version     display version information and exit
              -X <x-options>    x options. try -X help

            Option Parameters:
              <exp>     regular expression, color the entire match. 

            Environments:
              AKI_MCOLOR_COLOR_SEQ_RED_ST       red start sequence specified by ansi
              AKI_MCOLOR_COLOR_SEQ_GREEN_ST     greep start sequence specified by ansi
              AKI_MCOLOR_COLOR_SEQ_BLUE_ST      blue start sequence specified by ansi
              AKI_MCOLOR_COLOR_SEQ_CYAN_ST      cyan start sequence specified by ansi
              AKI_MCOLOR_COLOR_SEQ_MAGENDA_ST   magenda start sequence specified by ansi
              AKI_MCOLOR_COLOR_SEQ_YELLOW_ST    yellow start sequence specified by ansi
              AKI_MCOLOR_COLOR_SEQ_ED           color end sequence specified by ansi
            "#
            ),
            "\n"
        )
    };
}

macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

macro_rules! program_name {
    () => {
        "aki-mcolor"
    };
}

macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}

//mod helper;

mod test_0 {
    use exec_target::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, &["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, &["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, &["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, &["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_non_option() {
        let oup = exec_target(TARGET_EXE_PATH, &[""]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option: r|g|b|c|m|y|u\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
}

mod test_1 {
    use exec_target::exec_target_with_env_in;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    use std::collections::HashMap;
    //
    #[test]
    fn test_red() {
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert("AKI_MCOLOR_COLOR_SEQ_RED_ST".to_string(), "<S>".to_string());
        env.insert("AKI_MCOLOR_COLOR_SEQ_ED".to_string(), "<E>".to_string());
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-r", "c"], env, b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<S>c<E>defg\n");
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_green() {
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert(
            "AKI_MCOLOR_COLOR_SEQ_GREEN_ST".to_string(),
            "<S>".to_string(),
        );
        env.insert("AKI_MCOLOR_COLOR_SEQ_ED".to_string(), "<E>".to_string());
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-g", "c"], env, b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<S>c<E>defg\n");
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_blue() {
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert(
            "AKI_MCOLOR_COLOR_SEQ_BLUE_ST".to_string(),
            "<S>".to_string(),
        );
        env.insert("AKI_MCOLOR_COLOR_SEQ_ED".to_string(), "<E>".to_string());
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-b", "c"], env, b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<S>c<E>defg\n");
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_cyan() {
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert(
            "AKI_MCOLOR_COLOR_SEQ_CYAN_ST".to_string(),
            "<S>".to_string(),
        );
        env.insert("AKI_MCOLOR_COLOR_SEQ_ED".to_string(), "<E>".to_string());
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-c", "c"], env, b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<S>c<E>defg\n");
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_magenda() {
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert(
            "AKI_MCOLOR_COLOR_SEQ_MAGENDA_ST".to_string(),
            "<S>".to_string(),
        );
        env.insert("AKI_MCOLOR_COLOR_SEQ_ED".to_string(), "<E>".to_string());
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-m", "c"], env, b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<S>c<E>defg\n");
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_yellow() {
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert(
            "AKI_MCOLOR_COLOR_SEQ_YELLOW_ST".to_string(),
            "<S>".to_string(),
        );
        env.insert("AKI_MCOLOR_COLOR_SEQ_ED".to_string(), "<E>".to_string());
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-y", "c"], env, b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<S>c<E>defg\n");
        assert_eq!(oup.status.success(), true);
    }
}

mod test_3 {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_output_broken_pipe() {
        let cmdstr = format!(
            "cat \"{}\" | \"{}\" -r \"A\" | head -n 2",
            fixture_text10k!(),
            TARGET_EXE_PATH
        );
        let oup = exec_target("sh", &["-c", &cmdstr]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "\u{1b}[1;31mA\u{1b}[0mBCDEFG\nHIJKLMN\n");
        assert_eq!(oup.status.success(), true);
    }
}
