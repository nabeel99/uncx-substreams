// use super::pb::uncx::;
use super::UncxProgramEvent;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
pub fn parse_uncx_logs(log_messages: &Vec<String>) -> Result<Option<UncxProgramEvent>,String> {
    let mut result: Option<UncxProgramEvent> = None;

    for log_message in log_messages {
        if log_message.starts_with("Program data: ") {
            let b64_str = log_message.replace("Program data: ", "").trim().to_string();
            let bytes_stream = STANDARD.decode(b64_str);
                let bytes_stream_unwraped = bytes_stream.map_err(|_| "error while decoding program data")?;
              result = UncxProgramEvent::unpack_event(&bytes_stream_unwraped).ok();
            
        }
    }

    return Ok(result);
}