//! CLI command parser and dispatcher
//!
//! Handles parsing terminal commands and executing actions.

use heapless::Vec;

/// Maximum number of command arguments
pub const MAX_ARGS: usize = 4;

/// Parsed command with arguments
#[derive(Debug, Clone)]
pub struct Command<'a> {
    pub name: &'a str,
    pub args: Vec<&'a str, MAX_ARGS>,
}

/// Parse a command line into command name and arguments
pub fn parse_command(line: &str) -> Option<Command<'_>> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    let mut parts = line.split_whitespace();
    let name = parts.next()?;

    let mut args = Vec::new();
    for arg in parts {
        if args.push(arg).is_err() {
            // Too many arguments
            break;
        }
    }

    Some(Command { name, args })
}

/// Command response/result
pub enum CommandResult {
    Ok,
    OkWithMessage(&'static str),
    Error(&'static str),
    Unknown,
}

/// Available commands
pub enum CliCommand {
    Help,
    Status,
    Reset,
    ImuRead,
    ImuStream,
    ImuStreamStop,
    ImuRange,
    ImuFilter,
    ImuStatus,
    LedOn,
    LedOff,
    LedColor,
    Unknown,
}

/// Identify command from string
pub fn identify_command(name: &str) -> CliCommand {
    match name {
        "help" => CliCommand::Help,
        "status" => CliCommand::Status,
        "reset" => CliCommand::Reset,
        "imu_read" => CliCommand::ImuRead,
        "imu_stream" => CliCommand::ImuStream,
        "imu_stop" => CliCommand::ImuStreamStop,
        "imu_range" => CliCommand::ImuRange,
        "imu_filter" => CliCommand::ImuFilter,
        "imu_status" => CliCommand::ImuStatus,
        "led_on" => CliCommand::LedOn,
        "led_off" => CliCommand::LedOff,
        "led_color" => CliCommand::LedColor,
        _ => CliCommand::Unknown,
    }
}

/// Help text for all commands
pub const HELP_TEXT: &str = "\
Available Commands:
  help                    - Show this help
  status                  - Show system status
  reset                   - Reset system

  IMU Commands:
  imu_read                - Read accelerometer once
  imu_stream <hz>         - Stream IMU data (10, 50, 100 Hz)
  imu_stop                - Stop IMU streaming
  imu_range <g>           - Set accel range (2, 4, 8, 16)
  imu_filter <hz>         - Set filter bandwidth
  imu_status              - Show IMU configuration

  LED Commands:
  led_on                  - Turn on LED (blue)
  led_off                 - Turn off LED
  led_color <r> <g> <b>   - Set LED color (0-255)
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        assert!(parse_command("").is_none());
        assert!(parse_command("   ").is_none());
    }

    #[test]
    fn test_parse_simple() {
        let cmd = parse_command("help").unwrap();
        assert_eq!(cmd.name, "help");
        assert_eq!(cmd.args.len(), 0);
    }

    #[test]
    fn test_parse_with_args() {
        let cmd = parse_command("led_color 255 0 128").unwrap();
        assert_eq!(cmd.name, "led_color");
        assert_eq!(cmd.args.len(), 3);
        assert_eq!(cmd.args[0], "255");
        assert_eq!(cmd.args[1], "0");
        assert_eq!(cmd.args[2], "128");
    }

    #[test]
    fn test_parse_extra_whitespace() {
        let cmd = parse_command("  imu_stream   50  ").unwrap();
        assert_eq!(cmd.name, "imu_stream");
        assert_eq!(cmd.args.len(), 1);
        assert_eq!(cmd.args[0], "50");
    }

    #[test]
    fn test_identify_commands() {
        assert!(matches!(identify_command("help"), CliCommand::Help));
        assert!(matches!(identify_command("imu_read"), CliCommand::ImuRead));
        assert!(matches!(identify_command("led_on"), CliCommand::LedOn));
        assert!(matches!(identify_command("invalid"), CliCommand::Unknown));
    }
}
