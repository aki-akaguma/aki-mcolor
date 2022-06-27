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
                let _ = sioe
                    .perr()
                    .lock()
                    .write_fmt(format_args!("{}: {}\n", program, err));
            }
        };
        (r, sioe)
    }};
    ($env:expr, $args:expr, $sin:expr) => {{
        let sioe = RunnelIoe::new(
            Box::new(StringIn::with_str($sin)),
            Box::new(StringOut::default()),
            Box::new(StringErr::default()),
        );
        let program = env!("CARGO_PKG_NAME");
        let r = execute_env(&sioe, &program, $args, $env);
        match r {
            Ok(_) => {}
            Err(ref err) => {
                let _ = sioe
                    .perr()
                    .lock()
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

mod test_0_s {
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

mod test_1_s {
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_red() {
        let mut env = conf::EnvConf::new();
        env.color_seq_red_start = "<S>".to_string();
        env.color_seq_end = "<E>".to_string();
        let (r, sioe) = do_execute!(&env, &["-r", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<S>c<E>defg\n");
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_green() {
        let mut env = conf::EnvConf::new();
        env.color_seq_green_start = "<S>".to_string();
        env.color_seq_end = "<E>".to_string();
        let (r, sioe) = do_execute!(&env, &["-g", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<S>c<E>defg\n");
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_blue() {
        let mut env = conf::EnvConf::new();
        env.color_seq_blue_start = "<S>".to_string();
        env.color_seq_end = "<E>".to_string();
        let (r, sioe) = do_execute!(&env, &["-b", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<S>c<E>defg\n");
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_cyan() {
        let mut env = conf::EnvConf::new();
        env.color_seq_cyan_start = "<S>".to_string();
        env.color_seq_end = "<E>".to_string();
        let (r, sioe) = do_execute!(&env, &["-c", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<S>c<E>defg\n");
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_magenda() {
        let mut env = conf::EnvConf::new();
        env.color_seq_magenda_start = "<S>".to_string();
        env.color_seq_end = "<E>".to_string();
        let (r, sioe) = do_execute!(&env, &["-m", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<S>c<E>defg\n");
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_yellow() {
        let mut env = conf::EnvConf::new();
        env.color_seq_yellow_start = "<S>".to_string();
        env.color_seq_end = "<E>".to_string();
        let (r, sioe) = do_execute!(&env, &["-y", "c"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<S>c<E>defg\n");
        assert_eq!(r.is_ok(), true);
    }
}
mod test_2_s {
    use libaki_mcolor::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    use std::io::Write;
    //
    #[test]
    fn test_red_green() {
        let mut env = conf::EnvConf::new();
        env.color_seq_red_start = "<R>".to_string();
        env.color_seq_green_start = "<G>".to_string();
        env.color_seq_end = "<E>".to_string();
        let (r, sioe) = do_execute!(&env, &["-r", "c", "-g", "d"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<R>c<E><G>d<E>efg\n");
        assert_eq!(r.is_ok(), true);
    }
    #[test]
    fn test_red_green_red() {
        let mut env = conf::EnvConf::new();
        env.color_seq_red_start = "<R>".to_string();
        env.color_seq_green_start = "<G>".to_string();
        env.color_seq_end = "<E>".to_string();
        let (r, sioe) = do_execute!(&env, &["-r", "c", "-g", "d", "-r", "e"], "abcdefg");
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), "ab<R>c<E><G>d<E><R>e<E>fg\n");
        assert_eq!(r.is_ok(), true);
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
