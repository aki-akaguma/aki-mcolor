#[macro_use]
mod helper;

macro_rules! do_execute {
    ($args:expr) => {
        do_execute!($args, "")
    };
    ($args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute(&sioe, &program, $args);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .pg_err()
                    .lock()
                    .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
    ($env:expr, $args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            #[allow(clippy::box_default)]
            Box::new(StringOut::default()),
            #[allow(clippy::box_default)]
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute_env(&sioe, &program, $args, $env);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .pg_err()
                    .lock()
                    .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
}

macro_rules! buff {
    ($sioe:expr, serr) => {
        $sioe.pg_err().lock().buffer_to_string()
    };
    ($sioe:expr, sout) => {
        $sioe.pg_out().lock().buffer_to_string()
    };
}

macro_rules! env_1 {
    () => {{
        let mut env = conf::EnvConf::new();
        env.color_seq_red_start = "<R>".to_string();
        env.color_seq_green_start = "<G>".to_string();
        env.color_seq_blue_start = "<B>".to_string();
        env.color_seq_cyan_start = "<C>".to_string();
        env.color_seq_magenda_start = "<M>".to_string();
        env.color_seq_yellow_start = "<Y>".to_string();
        env.color_seq_end = "<E>".to_string();
        env
    }};
}

mod test_0_s {
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

mod test_0_x_options_s {
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
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(buff!(sioe, sout).contains("-X rust-version-info"));
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

mod test_1_s {
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_red() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-r", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<R>c<E>defg\n");
        assert!(r.is_ok());
    }
    #[test]
    fn test_green() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-g", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<G>c<E>defg\n");
        assert!(r.is_ok());
    }
    #[test]
    fn test_blue() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-b", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<B>c<E>defg\n");
        assert!(r.is_ok());
    }
    #[test]
    fn test_cyan() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-c", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<C>c<E>defg\n");
        assert!(r.is_ok());
    }
    #[test]
    fn test_magenda() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-m", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<M>c<E>defg\n");
        assert!(r.is_ok());
    }
    #[test]
    fn test_yellow() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-y", "c"], "abcdefg");
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
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-r", "a"], &v);
        assert_eq!(
            buff!(sioe, serr),
            concat!(program_name!(), ": stream did not contain valid UTF-8\n",)
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    */
}

mod test_2_s {
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_red_green() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-r", "c", "-g", "d"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<R>c<E><G>d<E>efg\n");
        assert!(r.is_ok());
    }
    #[test]
    fn test_red_green_red() {
        let env = env_1!();
        let (r, sioe) = do_execute!(&env, ["-r", "c", "-g", "d", "-r", "e"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<R>c<E><G>d<E><R>e<E>fg\n");
        assert!(r.is_ok());
    }
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
