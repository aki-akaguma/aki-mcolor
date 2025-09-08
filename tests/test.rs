const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

#[macro_use]
mod helper;

macro_rules! env_1 {
    () => {{
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert("AKI_MCOLOR_COLOR_SEQ_RED_ST".to_string(), "<R>".to_string());
        env.insert(
            "AKI_MCOLOR_COLOR_SEQ_GREEN_ST".to_string(),
            "<G>".to_string(),
        );
        env.insert(
            "AKI_MCOLOR_COLOR_SEQ_BLUE_ST".to_string(),
            "<B>".to_string(),
        );
        env.insert(
            "AKI_MCOLOR_COLOR_SEQ_CYAN_ST".to_string(),
            "<C>".to_string(),
        );
        env.insert(
            "AKI_MCOLOR_COLOR_SEQ_MAGENDA_ST".to_string(),
            "<M>".to_string(),
        );
        env.insert(
            "AKI_MCOLOR_COLOR_SEQ_YELLOW_ST".to_string(),
            "<Y>".to_string(),
        );
        env.insert("AKI_MCOLOR_COLOR_SEQ_ED".to_string(), "<E>".to_string());
        env
    }};
}

mod test_0 {
    use exec_target::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-H"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, ["-V"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--version"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_invalid_opt() {
        let oup = exec_target(TARGET_EXE_PATH, ["-z"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                "Missing option: r|g|b|c|m|y|u\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    #[test]
    fn test_non_option() {
        let oup = exec_target(TARGET_EXE_PATH, [""]);
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
        assert!(!oup.status.success());
    }
}

mod test_0_x_options {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_x_option_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("Options:"));
        assert!(oup.stdout.contains("-X rust-version-info"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        // The first one should be executed and the program should exit.
        assert!(oup.stdout.contains("Options:"));
        assert!(!oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
}

mod test_1 {
    use exec_target::exec_target_with_env_in;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    use std::collections::HashMap;
    //
    #[test]
    fn test_red() {
        let env = env_1!();
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-r", "c"], env, b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<R>c<E>defg\n");
        assert!(oup.status.success());
    }
    #[test]
    fn test_green() {
        let env = env_1!();
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-g", "c"], env, b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<G>c<E>defg\n");
        assert!(oup.status.success());
    }
    #[test]
    fn test_blue() {
        let env = env_1!();
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-b", "c"], env, b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<B>c<E>defg\n");
        assert!(oup.status.success());
    }
    #[test]
    fn test_cyan() {
        let env = env_1!();
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-c", "c"], env, b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<C>c<E>defg\n");
        assert!(oup.status.success());
    }
    #[test]
    fn test_magenda() {
        let env = env_1!();
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-m", "c"], env, b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<M>c<E>defg\n");
        assert!(oup.status.success());
    }
    #[test]
    fn test_yellow() {
        let env = env_1!();
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-y", "c"], env, b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<Y>c<E>defg\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8() {
        let v = {
            use std::io::Read;
            let mut f = std::fs::File::open(fixture_invalid_utf8!()).unwrap();
            let mut v = Vec::new();
            f.read_to_end(&mut v).unwrap();
            v
        };
        let env = env_1!();
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-r", "a"], env, &v);
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_2 {
    use exec_target::exec_target_with_env_in;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    use std::collections::HashMap;
    //
    #[test]
    fn test_red_green() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "c", "-g", "d"],
            env,
            b"abcdefg" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<R>c<E><G>d<E>efg\n");
        assert!(oup.status.success());
    }
    #[test]
    fn test_red_green_red() {
        let env = env_1!();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "c", "-g", "d", "-r", "e"],
            env,
            b"abcdefg" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<R>c<E><G>d<E><R>e<E>fg\n");
        assert!(oup.status.success());
    }
}

mod test_3 {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_output_broken_pipe() {
        let cmdstr = format!(
            "cat \"{}\" | \"{}\" -r \"A\" | head -n 2",
            fixture_text10k!(),
            TARGET_EXE_PATH
        );
        let oup = exec_target("sh", ["-c", &cmdstr]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "\u{1b}[1;31mA\u{1b}[0mBCDEFG\nHIJKLMN\n");
        assert!(oup.status.success());
    }
}
