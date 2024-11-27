
use cli_test::config;

#[test]
fn test_loggin_config() {
    let config = config::LogginConfig::new(config::Loglevel::Info, config::LogOut::Stdout);
    assert_eq!(config.level, config::Loglevel::Info);
    assert_eq!(config.output, config::LogOut::Stdout);
}

#[test]
fn test_read_stdin() {
    assert!(true);
}