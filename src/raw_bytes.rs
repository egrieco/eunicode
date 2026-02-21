use std::u8;
use termwiz::escape::{Action, CSI, ControlCode, parser::Parser};

/// RawBytes state: may contain invalid UTF-8
pub struct RawBytes(pub Vec<u8>);

impl RawBytes {
    pub fn default() -> RawBytes {
        RawBytes(vec![])
    }

    pub fn from_bytes(bytes: Vec<u8>) -> RawBytes {
        RawBytes(bytes)
    }

    pub fn from_string(string: &str) -> RawBytes {
        RawBytes(string.as_bytes().to_vec())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn strip_ansi_escapes(&self, keep_colors: bool) -> String {
        let mut escape_parser = Parser::new();
        let mut output: Vec<Action> = Default::default();

        for action in Parser::parse_as_vec(&mut escape_parser, &self.0) {
            let append_action = match action {
                // collect actual chars
                Action::Print(_char) => true,
                Action::PrintString(ref _string) => true,

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
                    ControlCode::Backspace => true,
                    ControlCode::HorizontalTab => true,
                    ControlCode::LineFeed => true,
                    ControlCode::VerticalTab => true,
                    ControlCode::FormFeed => true,
                    ControlCode::CarriageReturn => true,
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
                    _ => false,
                },
                // Action::Esc(esc) => todo!(),

                // pass through CSI SGR codes to change how text is rendered
                Action::CSI(ref csi) => match csi {
                    CSI::Sgr(_sgr) => keep_colors,
                    // CSI::Cursor(cursor) => todo!(),
                    // CSI::Edit(edit) => true,
                    // CSI::Mode(mode) => todo!(),
                    // CSI::Device(device) => todo!(),
                    // CSI::Mouse(mouse_report) => todo!(),
                    // CSI::Window(window) => todo!(),
                    // CSI::Keyboard(keyboard) => todo!(),
                    // CSI::SelectCharacterPath(character_path, _) => todo!(),
                    // CSI::Unspecified(unspecified) => todo!(),
                    _ => false,
                },

                // skip device control codes and OSC
                // Action::DeviceControl(device_control_mode) => todo!(),
                // Action::OperatingSystemCommand(operating_system_command) => todo!(),
                // don't allow programs to get terminal info
                // Action::XtGetTcap(ref _items) => true,

                // TODO allow graphics rendering?
                // Action::Sixel(sixel) => todo!(),
                // Action::KittyImage(kitty_image) => todo!(),
                _ => false,
            };

            if append_action {
                action.append_to(&mut output)
            }
        }

        output.iter().map(|a| a.to_string()).collect()
    }
}
