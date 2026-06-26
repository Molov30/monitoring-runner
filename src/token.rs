use std::process::Command;

pub fn get_token(ticket_dir: &std::path::Path) -> Result<String, String> {
    let ngpsh = if cfg!(windows) { "ngpsh.exe" } else { "ngpsh" };

    let out = Command::new(ngpsh)
        .args(["platformInfo", "system-token"])
        .env("NGP_SECRETS_DIR", ticket_dir)
        .output()
        .map_err(|e| format!("failed to run {}: {}", ngpsh, e))?;

    if !out.status.success() {
        return Err(format!("{} exited with {}", ngpsh, out.status));
    }

    let json: serde_json::Value = serde_json::from_slice(&out.stdout)
        .map_err(|e| format!("failed to parse {} output: {}", ngpsh, e))?;

    json["TokenValue"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("TokenValue field not found in {} output", ngpsh))
}
