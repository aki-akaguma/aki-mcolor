use flood_tide_gen::{FixupType, MetaType, Pasc};

pub fn do_gen_src() -> anyhow::Result<()> {
    flood_tide_gen::do_gen_src(
        Pasc::Void,
        "xtask/src/aki-mcolor-cmd.txt",
        Some("src/conf/cmd.help.rs.txt"),
        Some("src/conf/cmd.match.rs.txt"),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "red" => (false, true, MetaType::String),
                "green" => (false, true, MetaType::String),
                "blue" => (false, true, MetaType::String),
                "cyan" => (false, true, MetaType::String),
                "magenda" => (false, true, MetaType::String),
                "yellow" => (false, true, MetaType::String),
                "unmark" => (false, true, MetaType::String),
                //
                "X" => (false, true, MetaType::Other("opt_uc_x_param".into())),
                _ => return None,
            };
            Some(FixupType::from_tuple(tup))
        },
    )
}
