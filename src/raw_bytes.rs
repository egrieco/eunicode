use std::u8;
use termwiz::escape::{Action, CSI, ControlCode, parser::Parser};

/// RawBytes state: may contain invalid UTF-8
pub struct RawBytes(pub Vec<u8>);

impl RawBytes {
    pub fn default() -> RawBytes {
        RawBytes(vec![])
    }

    pub fn from_string(string: &str) -> RawBytes {
        RawBytes(string.as_bytes().to_vec())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn strip_ansi_escapes(&self, keep_colors: bool) -> String {
        let mut escape_parser = Parser::new();
        let mut output = String::default();

        for action in Parser::parse_as_vec(&mut escape_parser, &self.0) {
            match action {
                // collect actual chars
                Action::Print(char) => output.push(char),
                Action::PrintString(string) => output.push_str(&string),

                // collect linefeeds and such
                Action::Control(control_code) => match control_code {
                    // ControlCode::Null => todo!(),
                    // ControlCode::StartOfHeading => todo!(),
                    // ControlCode::StartOfText => todo!(),
                    // ControlCode::EndOfText => todo!(),
                    // ControlCode::EndOfTransmission => todo!(),
                    // ControlCode::Enquiry => todo!(),
                    // ControlCode::Acknowledge => todo!(),
                    // ControlCode::Bell => todo!(),
                    // ControlCode::Backspace => output.push(""),
                    ControlCode::HorizontalTab => output.push('\t'),
                    ControlCode::LineFeed => output.push('\n'),
                    // ControlCode::VerticalTab => todo!(),
                    // ControlCode::FormFeed => output.push(""),
                    ControlCode::CarriageReturn => output.push('\r'),
                    // ControlCode::ShiftOut => todo!(),
                    // ControlCode::ShiftIn => todo!(),
                    // ControlCode::DataLinkEscape => todo!(),
                    // ControlCode::DeviceControlOne => todo!(),
                    // ControlCode::DeviceControlTwo => todo!(),
                    // ControlCode::DeviceControlThree => todo!(),
                    // ControlCode::DeviceControlFour => todo!(),
                    // ControlCode::NegativeAcknowledge => todo!(),
                    // ControlCode::SynchronousIdle => todo!(),
                    // ControlCode::EndOfTransmissionBlock => todo!(),
                    // ControlCode::Cancel => todo!(),
                    // ControlCode::EndOfMedium => todo!(),
                    // ControlCode::Substitute => todo!(),
                    // ControlCode::Escape => todo!(),
                    // ControlCode::FileSeparator => todo!(),
                    // ControlCode::GroupSeparator => todo!(),
                    // ControlCode::RecordSeparator => todo!(),
                    // ControlCode::UnitSeparator => todo!(),
                    _ => {}
                },
                // Action::Esc(esc) => todo!(),

                // pass through CSI SGR codes to change how text is rendered
                Action::CSI(csi) => match csi {
                    CSI::Sgr(sgr) => {
                        if keep_colors {
                            output.push_str(&format!("\x1b[{}m", sgr.to_string()))
                        }
                    }
                    // CSI::Cursor(cursor) => todo!(),
                    // CSI::Edit(edit) => todo!(),
                    // CSI::Mode(mode) => todo!(),
                    // CSI::Device(device) => todo!(),
                    // CSI::Mouse(mouse_report) => todo!(),
                    // CSI::Window(window) => todo!(),
                    // CSI::Keyboard(keyboard) => todo!(),
                    // CSI::SelectCharacterPath(character_path, _) => todo!(),
                    // CSI::Unspecified(unspecified) => todo!(),
                    _ => {}
                },

                // skip device control codes and OSC
                // Action::DeviceControl(device_control_mode) => todo!(),
                // Action::OperatingSystemCommand(operating_system_command) => todo!(),
                // don't allow programs to get terminal info
                // Action::XtGetTcap(items) => todo!(),

                // TODO allow graphics rendering?
                // Action::Sixel(sixel) => todo!(),
                // Action::KittyImage(kitty_image) => todo!(),
                _ => {}
            }
        }

        output
    }
}
