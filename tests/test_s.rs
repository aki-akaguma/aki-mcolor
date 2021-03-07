macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            "Usage:\n",
            "  aki-mcolor [options]\n",
            "\n",
            "mark up text with color\n",
            "\n",
            "Options:\n",
            "  -r, --red <exp>       regular expression, mark color is red\n",
            "  -g, --green <exp>     regular expression, mark color is green\n",
            "  -b, --blue <exp>      regular expression, mark color is blue\n",
            "  -c, --cyan <exp>      regular expression, mark color is cyan\n",
            "  -m, --magenda <exp>   regular expression, mark color is magenda\n",
            "  -y, --yellow <exp>    regular expression, mark color is yellow\n",
            "  -u, --unmark <exp>    regular expression, unmark color\n",
            "\n",
            "  -H, --help        display this help and exit\n",
            "  -V, --version     display version information and exit\n",
            "\n",
            "Env:\n",
            "  AKI_MCOLOR_RED_ST         red start sequence\n",
            "  AKI_MCOLOR_GREEN_ST       greep start sequence\n",
            "  AKI_MCOLOR_BLUE_ST        blue start sequence\n",
            "  AKI_MCOLOR_CYAN_ST        cyan start sequence\n",
            "  AKI_MCOLOR_MAGENDA_ST     magenda start sequence\n",
            "  AKI_MCOLOR_YELLOW_ST      yellow start sequence\n",
            "  AKI_MCOLOR_ED             color end sequence\n",
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

/*
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}
*/

macro_rules! do_execute {
    ($args:expr) => {
        do_execute!($args, "")
    };
    ($args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            Box::new(StringOut::default()),
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute(&sioe, &program, $args);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                #[rustfmt::skip]
                                        let _ = sioe.perr().lock()
                                            .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.perr().lock().buffer_str()
    };
    ($sioe:expr, sout) => {
        $sioe.pout().lock().buffer_str()
    };
}

mod test_0 {
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::*;
    use std::io::Write;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(&["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(&["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(&["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(&["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_non_option() {
        let (r, sioe) = do_execute!(&[""]);
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
        assert_eq!(r.is_ok(), false);
    }
}

mod test_1 {
    /*
        use libaki_mcolor::*;
        use runnel::medium::stringio::{StringErr, StringIn, StringOut};
        use libaki_mcolor::*;
        use std::io::Write;
        use std::collections::HashMap;
        //
        #[test]
        fn test_red() {
            let mut env: HashMap<String, String> = HashMap::new();
            env.insert("RUST_MCOLOR_RED_ST".to_string(), "<S>".to_string());
            env.insert("RUST_MCOLOR_ED".to_string(), "<E>".to_string());
            let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-r", "c"], env, b"abcdefg" as &[u8]);
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, "ab<S>c<E>defg\n");
            assert_eq!(oup.status.success(), true);
        }
        #[test]
        fn test_green() {
            let mut env: HashMap<String, String> = HashMap::new();
            env.insert("RUST_MCOLOR_GREEN_ST".to_string(), "<S>".to_string());
            env.insert("RUST_MCOLOR_ED".to_string(), "<E>".to_string());
            let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-g", "c"], env, b"abcdefg" as &[u8]);
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, "ab<S>c<E>defg\n");
            assert_eq!(oup.status.success(), true);
        }
        #[test]
        fn test_blue() {
            let mut env: HashMap<String, String> = HashMap::new();
            env.insert("RUST_MCOLOR_BLUE_ST".to_string(), "<S>".to_string());
            env.insert("RUST_MCOLOR_ED".to_string(), "<E>".to_string());
            let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-b", "c"], env, b"abcdefg" as &[u8]);
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, "ab<S>c<E>defg\n");
            assert_eq!(oup.status.success(), true);
        }
        #[test]
        fn test_cyan() {
            let mut env: HashMap<String, String> = HashMap::new();
            env.insert("RUST_MCOLOR_CYAN_ST".to_string(), "<S>".to_string());
            env.insert("RUST_MCOLOR_ED".to_string(), "<E>".to_string());
            let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-c", "c"], env, b"abcdefg" as &[u8]);
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, "ab<S>c<E>defg\n");
            assert_eq!(oup.status.success(), true);
        }
        #[test]
        fn test_magenda() {
            let mut env: HashMap<String, String> = HashMap::new();
            env.insert("RUST_MCOLOR_MAGENDA_ST".to_string(), "<S>".to_string());
            env.insert("RUST_MCOLOR_ED".to_string(), "<E>".to_string());
            let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-m", "c"], env, b"abcdefg" as &[u8]);
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, "ab<S>c<E>defg\n");
            assert_eq!(oup.status.success(), true);
        }
        #[test]
        fn test_yellow() {
            let mut env: HashMap<String, String> = HashMap::new();
            env.insert("RUST_MCOLOR_YELLOW_ST".to_string(), "<S>".to_string());
            env.insert("RUST_MCOLOR_ED".to_string(), "<E>".to_string());
            let oup = exec_target_with_env_in(TARGET_EXE_PATH, &["-y", "c"], env, b"abcdefg" as &[u8]);
            assert_eq!(oup.stderr, "");
            assert_eq!(oup.stdout, "ab<S>c<E>defg\n");
            assert_eq!(oup.status.success(), true);
        }
    */
}

mod test_3 {
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
