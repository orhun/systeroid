use {
    systeroid::args::Args, systeroid_core::error::Result,
    systeroid_core::sysctl::display::DisplayType,
};

#[cfg_attr(not(feature = "live-tests"), ignore)]
#[test]
fn test_systeroid() -> Result<()> {
    let args = Args::default();
    let mut output = Vec::new();
    systeroid::run(args, &mut output)?;
    assert!(String::from_utf8_lossy(&output).contains("kernel.watchdog ="));

    let args = Args {
        values: vec![String::from("vm.zone_reclaim_mode=0")],
        display_type: DisplayType::Binary,
        ..Args::default()
    };
    let mut output = Vec::new();
    systeroid::run(args, &mut output)?;
    assert_eq!("0", String::from_utf8_lossy(&output));

    let args = Args {
        preload_system_files: true,
        ..Args::default()
    };
    systeroid::run(args, &mut Vec::new())?;

    let args = Args {
        preload_files: true,
        values: vec![String::from("sysctl.conf")],
        ..Args::default()
    };
    systeroid::run(args, &mut Vec::new())?;

    Ok(())
}
