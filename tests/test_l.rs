#[macro_use]
mod helper;

#[macro_use]
mod helper_l;

mod test_0_l {
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_invalid_opt() {
        let (r, sioe) = do_execute!(["-z"]);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                "Missing option: r|g|b|c|m|y|u\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!([""]);
        #[rustfmt::skip]
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(), ": ",
                "Missing option: r|g|b|c|m|y|u\n",
                "Unexpected argument: \n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
}

mod test_0_x_options_l {
    use libaki_mcolor::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    //
    #[test]
    fn test_x_rust_version_info() {
        let (r, sioe) = do_execute!(["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(!buff!(sioe, sout).is_empty());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_help() {
        let (r, sioe) = do_execute!(["-X", "help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), x_help_msg!());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let (r, sioe) = do_execute!(["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let (r, sioe) = do_execute!(["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        // The first one should be executed and the program should exit.
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(!buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
}

mod test_1_l {
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_red() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<R>c<E>defg\n");
        assert!(r.is_ok());
    }
    #[test]
    fn test_green() {
        let (r, sioe) = do_execute!(env_1!(), ["-g", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<G>c<E>defg\n");
        assert!(r.is_ok());
    }
    #[test]
    fn test_blue() {
        let (r, sioe) = do_execute!(env_1!(), ["-b", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<B>c<E>defg\n");
        assert!(r.is_ok());
    }
    #[test]
    fn test_cyan() {
        let (r, sioe) = do_execute!(env_1!(), ["-c", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<C>c<E>defg\n");
        assert!(r.is_ok());
    }
    #[test]
    fn test_magenda() {
        let (r, sioe) = do_execute!(env_1!(), ["-m", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<M>c<E>defg\n");
        assert!(r.is_ok());
    }
    #[test]
    fn test_yellow() {
        let (r, sioe) = do_execute!(env_1!(), ["-y", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<Y>c<E>defg\n");
        assert!(r.is_ok());
    }
    //
    /*
    #[test]
    fn test_invalid_utf8() {
        let v = {
            use std::io::Read;
            let mut f = std::fs::File::open(fixture_invalid_utf8!()).unwrap();
            let mut v = Vec::new();
            f.read_to_end(&mut v).unwrap();
            v
        };
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a"], &v);
        assert_eq!(
            buff!(sioe, serr),
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    */
}

mod test_1_more_l {
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_unmark() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a.c", "-u", "b"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<R>a<E>b<R>c<E>de\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_colors_same_line() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a", "-g", "b", "-b", "c"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<R>a<E><G>b<E><B>c<E>de\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_pattern_not_found() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "z"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcde\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_empty_input() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a"], "");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_pattern_at_beginning() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<R>a<E>bcde\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_pattern_at_end() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "e"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcd<R>e<E>\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_lines() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a"], "abc\ndefa");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<R>a<E>bc\ndef<R>a<E>\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_all_colors() {
        let (r, sioe) = do_execute!(
            env_1!(),
            ["-r", "a", "-g", "b", "-b", "c", "-c", "d", "-m", "e", "-y", "f", "-u", "g"],
            "abcdefg"
        );
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(
            buff!(sioe, sout),
            "<R>a<E><G>b<E><B>c<E><C>d<E><M>e<E><Y>f<E>g\n"
        );
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_overlapping_matches() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "ab", "-g", "bc"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<R>a<E><G>bc<E>de\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_complex_regex() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a[bc]d"], "abde abd acde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<R>abd<E>e <R>abd<E> <R>acd<E>e\n");
        assert!(r.is_ok());
    }
}

mod test_2_l {
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_red_green() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "c", "-g", "d"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<R>c<E><G>d<E>efg\n");
        assert!(r.is_ok());
    }
    #[test]
    fn test_red_green_red() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "c", "-g", "d", "-r", "e"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<R>c<E><G>d<E><R>e<E>fg\n");
        assert!(r.is_ok());
    }
}

mod test_2_more_l {
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_long_options() {
        let (r, sioe) = do_execute!(env_1!(), ["--red", "a", "--green", "b"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<R>a<E><G>b<E>cde\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_mix_short_long_options() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a", "--green", "b"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<R>a<E><G>b<E>cde\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_no_matches() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "z"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "abcde\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_utf8_chars() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "あ"], "あいうえお");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<R>あ<E>いうえお\n");
        assert!(r.is_ok());
    }
}

mod test_3_l {
    /*
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use std::io::Write;
    //
     * can NOT test
    #[test]
    fn test_output_broken_pipe() {
    }
    */
}

mod test_3_more_l {
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_single_capture_group() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a(b)c"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "a<R>b<E>cde\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_capture_groups() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a(b)c(d)"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "a<R>b<E>cde\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_no_capture_group() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "abc"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<R>abc<E>de\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_nested_capture_groups() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a((b)c)d"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "a<R>bc<E>de\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_empty_capture_group() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a()c"], "acde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "acde\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_optional_capture_group_match() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a(b)?c"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "a<R>b<E>cde\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_optional_capture_group_no_match() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a(b)?c"], "acde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "acde\n");
        assert!(r.is_ok());
    }
}

mod test_4_more_l {
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_invalid_regex() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "*"], "");
        assert!(buff!(sioe, serr).contains("regex parse error"));
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_large_input() {
        let mut large_input = String::new();
        for i in 0..1000 {
            large_input.push_str(&format!("line {} abc\n", i));
        }
        //
        let (r, sioe) = do_execute!(env_1!(), ["-r", "abc"], &large_input);
        //
        let mut expected_output = String::new();
        for i in 0..1000 {
            expected_output.push_str(&format!("line {} <R>abc<E>\n", i));
        }
        //
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), expected_output);
        assert!(r.is_ok());
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
        let (r, sioe) = do_execute!(env_1!(), ["-r", "[a-z]+"], &large_input);
        let duration = start.elapsed();
        //
        assert_eq!(buff!(sioe, serr), "");
        assert!(duration < std::time::Duration::from_secs(1));
        assert!(r.is_ok());
    }
}

mod test_5_more_l {
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_env_override() {
        let mut env = env_1!();
        env.push(("AKI_MCOLOR_COLOR_SEQ_RED_ST", "[RED]"));
        env.push(("AKI_MCOLOR_COLOR_SEQ_ED", "[/RED]"));
        //
        let (r, sioe) = do_execute!(env, ["-r", "a"], "abcde");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "[RED]a[/RED]bcde\n");
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_crlf_line_endings() {
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a"], "abc\r\ndefa");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "<R>a<E>bc\ndef<R>a<E>\n");
        assert!(r.is_ok());
    }
    //
    /*
    #[test]
    fn test_binary_input() {
        use std::fs::File;
        let mut f = File::open(fixture_invalid_utf8!()).unwrap();
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).unwrap();
        //
        let env = env_1!();
        let (r, sioe) = do_execute!(env_1!(), ["-r", "a"], &buffer);
        //
        assert!(buff!(sioe, sout).contains("stream did not contain valid UTF-8"));
        assert_eq!(buff!(sioe, serr), "");
        assert!(r.is_err());
    }
    */
}
