const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

#[macro_use]
mod helper;

#[macro_use]
mod helper_e;

mod test_0_e {
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

mod test_0_x_options_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_x_option_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, x_help_msg!());
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

mod test_1_e {
    use exec_target::exec_target_with_env_in;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_red() {
        let oup =
            exec_target_with_env_in(TARGET_EXE_PATH, ["-r", "c"], env_1!(), b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<R>c<E>defg\n");
        assert!(oup.status.success());
    }
    #[test]
    fn test_green() {
        let oup =
            exec_target_with_env_in(TARGET_EXE_PATH, ["-g", "c"], env_1!(), b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<G>c<E>defg\n");
        assert!(oup.status.success());
    }
    #[test]
    fn test_blue() {
        let oup =
            exec_target_with_env_in(TARGET_EXE_PATH, ["-b", "c"], env_1!(), b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<B>c<E>defg\n");
        assert!(oup.status.success());
    }
    #[test]
    fn test_cyan() {
        let oup =
            exec_target_with_env_in(TARGET_EXE_PATH, ["-c", "c"], env_1!(), b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<C>c<E>defg\n");
        assert!(oup.status.success());
    }
    #[test]
    fn test_magenda() {
        let oup =
            exec_target_with_env_in(TARGET_EXE_PATH, ["-m", "c"], env_1!(), b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<M>c<E>defg\n");
        assert!(oup.status.success());
    }
    #[test]
    fn test_yellow() {
        let oup =
            exec_target_with_env_in(TARGET_EXE_PATH, ["-y", "c"], env_1!(), b"abcdefg" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<Y>c<E>defg\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_invalid_utf8() {
        let v = std::fs::read(fixture_invalid_utf8!()).unwrap();
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-r", "a"], env_1!(), &v);
        assert_eq!(
            oup.stderr,
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_1_more_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_unmark() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "a.c", "-u", "b"],
            env_1!(),
            b"abcde" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<R>a<E>b<R>c<E>de\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_colors_same_line() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "a", "-g", "b", "-b", "c"],
            env_1!(),
            b"abcde" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<R>a<E><G>b<E><B>c<E>de\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_pattern_not_found() {
        let oup =
            exec_target_with_env_in(TARGET_EXE_PATH, ["-r", "z"], env_1!(), b"abcde" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcde\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_empty_input() {
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-r", "a"], env_1!(), b"" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_pattern_at_beginning() {
        let oup =
            exec_target_with_env_in(TARGET_EXE_PATH, ["-r", "a"], env_1!(), b"abcde" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<R>a<E>bcde\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_pattern_at_end() {
        let oup =
            exec_target_with_env_in(TARGET_EXE_PATH, ["-r", "e"], env_1!(), b"abcde" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcd<R>e<E>\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_lines() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "a"],
            env_1!(),
            b"abc\ndefa" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<R>a<E>bc\ndef<R>a<E>\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_all_colors() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            [
                "-r", "a", "-g", "b", "-b", "c", "-c", "d", "-m", "e", "-y", "f", "-u", "g",
            ],
            env_1!(),
            b"abcdefg" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<R>a<E><G>b<E><B>c<E><C>d<E><M>e<E><Y>f<E>g\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_overlapping_matches() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "ab", "-g", "bc"],
            env_1!(),
            b"abcde" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<R>a<E><G>bc<E>de\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_complex_regex() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "a[bc]d"],
            env_1!(),
            b"abde abd acde" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<R>abd<E>e <R>abd<E> <R>acd<E>e\n");
        assert!(oup.status.success());
    }
}

mod test_2_e {
    use exec_target::exec_target_with_env_in;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_red_green() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "c", "-g", "d"],
            env_1!(),
            b"abcdefg" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<R>c<E><G>d<E>efg\n");
        assert!(oup.status.success());
    }
    #[test]
    fn test_red_green_red() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "c", "-g", "d", "-r", "e"],
            env_1!(),
            b"abcdefg" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab<R>c<E><G>d<E><R>e<E>fg\n");
        assert!(oup.status.success());
    }
}

mod test_2_more_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_long_options() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["--red", "a", "--green", "b"],
            env_1!(),
            b"abcde" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<R>a<E><G>b<E>cde\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_mix_short_long_options() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "a", "--green", "b"],
            env_1!(),
            b"abcde" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<R>a<E><G>b<E>cde\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_no_matches() {
        let oup =
            exec_target_with_env_in(TARGET_EXE_PATH, ["-r", "z"], env_1!(), b"abcde" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcde\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_utf8_chars() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "あ"],
            env_1!(),
            "あいうえお".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<R>あ<E>いうえお\n");
        assert!(oup.status.success());
    }
}

mod test_3_e {
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

mod test_3_more_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_single_capture_group() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "a(b)c"],
            env_1!(),
            b"abcde" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "a<R>b<E>cde\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_capture_groups() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "a(b)c(d)"],
            env_1!(),
            b"abcde" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "a<R>b<E>cde\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_no_capture_group() {
        let oup =
            exec_target_with_env_in(TARGET_EXE_PATH, ["-r", "abc"], env_1!(), b"abcde" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<R>abc<E>de\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_nested_capture_groups() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "a((b)c)d"],
            env_1!(),
            b"abcde" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "a<R>bc<E>de\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_empty_capture_group() {
        let oup =
            exec_target_with_env_in(TARGET_EXE_PATH, ["-r", "a()c"], env_1!(), b"acde" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "acde\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_optional_capture_group_match() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "a(b)?c"],
            env_1!(),
            b"abcde" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "a<R>b<E>cde\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_optional_capture_group_no_match() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "a(b)?c"],
            env_1!(),
            b"acde" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "acde\n");
        assert!(oup.status.success());
    }
}

mod test_4_more_e {
    use exec_target::{exec_target_with_env, exec_target_with_env_in};
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_invalid_regex() {
        let oup = exec_target_with_env(TARGET_EXE_PATH, ["-r", "*"], env_1!());
        assert!(oup.stderr.contains("regex parse error"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_large_input() {
        let mut large_input = String::new();
        for i in 0..1000 {
            large_input.push_str(&format!("line {} abc\n", i));
        }
        //
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "abc"],
            env_1!(),
            large_input.as_bytes(),
        );
        //
        let mut expected_output = String::new();
        for i in 0..1000 {
            expected_output.push_str(&format!("line {} <R>abc<E>\n", i));
        }
        //
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, expected_output);
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_performance() {
        let mut large_input = String::new();
        for i in 0..1000 {
            large_input.push_str(&format!("line {} abcdefghijklmnopqrstuvwxyz\n", i));
        }
        //
        let start = std::time::Instant::now();
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "[a-z]+"],
            env_1!(),
            large_input.as_bytes(),
        );
        let duration = start.elapsed();
        //
        assert_eq!(oup.stderr, "");
        assert!(duration < std::time::Duration::from_secs(1));
        assert!(oup.status.success());
    }
}

mod test_5_more_e {
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    use std::fs::File;
    use std::io::Read;
    //
    #[test]
    fn test_env_override() {
        let mut env = env_1!();
        env.push(("AKI_MCOLOR_COLOR_SEQ_RED_ST", "[RED]"));
        env.push(("AKI_MCOLOR_COLOR_SEQ_ED", "[/RED]"));
        //
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-r", "a"], env, b"abcde" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "[RED]a[/RED]bcde\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_crlf_line_endings() {
        let oup = exec_target_with_env_in(
            TARGET_EXE_PATH,
            ["-r", "a"],
            env_1!(),
            b"abc\r\ndefa" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "<R>a<E>bc\ndef<R>a<E>\n");
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_binary_input() {
        let mut f = File::open(fixture_invalid_utf8!()).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        //
        let oup = exec_target_with_env_in(TARGET_EXE_PATH, ["-r", "a"], env_1!(), &buffer);
        //
        assert!(oup.stderr.contains("stream did not contain valid UTF-8"));
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}
