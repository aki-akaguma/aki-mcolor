#[allow(unused_macros)]
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

#[allow(unused_macros)]
macro_rules! x_help_msg {
    () => {
        concat!(
            indoc::indoc!(
                r#"
            Options:
              -X rust-version-info     display rust version info and exit
            "#
            ),
            "\n",
        )
    };
}

#[allow(unused_macros)]
macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

#[allow(unused_macros)]
macro_rules! program_name {
    () => {
        "aki-mcolor"
    };
}

#[allow(unused_macros)]
macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

#[allow(unused_macros)]
macro_rules! fixture_invalid_utf8 {
    () => {
        "fixtures/invalid_utf8.txt"
    };
}

#[allow(unused_macros)]
macro_rules! fixture_text10k {
    () => {
        "fixtures/text10k.txt"
    };
}

#[allow(unused_macros)]
macro_rules! env_1 {
    () => {{
        vec![
            ("AKI_MCOLOR_COLOR_SEQ_RED_ST", "<R>"),
            ("AKI_MCOLOR_COLOR_SEQ_GREEN_ST", "<G>"),
            ("AKI_MCOLOR_COLOR_SEQ_BLUE_ST", "<B>"),
            ("AKI_MCOLOR_COLOR_SEQ_CYAN_ST", "<C>"),
            ("AKI_MCOLOR_COLOR_SEQ_MAGENDA_ST", "<M>"),
            ("AKI_MCOLOR_COLOR_SEQ_YELLOW_ST", "<Y>"),
            ("AKI_MCOLOR_COLOR_SEQ_ED", "<E>"),
        ]
    }};
}
