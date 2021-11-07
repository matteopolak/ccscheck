use hex;
use reqwest;
use serde_json::json;
use sha2::{Digest, Sha512};
use std::{
	path::{Path, PathBuf},
	process::Command,
};

fn play_audio(path: PathBuf) {
	#[cfg(target_os = "windows")]
	Command::new("powershell")
		.arg(format!(
			"(New-Object Media.SoundPlayer '{}').PlaySync()",
			path.to_str().unwrap()
		))
		.spawn()
		.ok();

	#[cfg(target_os = "linux")]
	Command::new("/usr/bin/play").arg(path).spawn().ok();
}

#[cfg(target_os = "linux")]
fn send_notification(message: &str) {
	// TODO: Windows notification

	let user = std::env::var_os("CCS_USER").unwrap();

	Command::new("/bin/bash")
		.arg("-c")
		.arg(&format!("/bin/sudo -u {} DISPLAY=:0 DBUS_SESSION_BUS_ADDRESS=\"unix:path=/run/user/$(/bin/id -u {})/bus\" /bin/notify-send \"{}\"", &user, &user, message))
		.spawn()
		.ok();
}

fn send_point_gain(base: &Path) {
	play_audio(base.join("gain.wav"));

	#[cfg(target_os = "linux")]
	send_notification("You gained points");
}

fn send_point_loss(base: &Path) {
	play_audio(base.join("loss.wav"));

	#[cfg(target_os = "linux")]
	send_notification("You lost points");
}

fn main() {
	let mut args = std::env::args();

	// retrieve the answer from the command-line
	// if there is none, exit with a non-zero code
	let answer = match args.nth(1) {
		Some(v) => v,
		None => std::process::exit(1),
	};

	let removed = match args.next() {
		Some(v) => v == "1",
		None => std::process::exit(1),
	};

	let mut hasher = Sha512::new();

	// hash the answer
	hasher.update(&answer);

	// get the hash
	let hash = hex::encode(hasher.finalize_reset());

	#[cfg(target_os = "windows")]
	let base = Path::new("C:\\CyberPatriot");

	#[cfg(target_os = "linux")]
	let base = Path::new("/opt/CyberPatriot");

	if !base.join("hashes").join(&hash).exists() {
		std::process::exit(2);
	}

	hasher.update(format!("{}{}", hash, answer));

	let flag = hex::encode(hasher.finalize());
	let team_id = std::env::var("TEAM_ID").unwrap_or("0123-0123-0123".to_string());

	let body = json!({
	  "id": team_id,
	  "flag": flag,
	  "removed": removed
	});

	let client = reqwest::blocking::Client::new();
	let response = client
		.put("http://192.168.1.129:3000/submit")
		.json(&body)
		.send()
		.unwrap();

	match response.status() {
		reqwest::StatusCode::NO_CONTENT => (),
		reqwest::StatusCode::ACCEPTED => send_point_gain(base),
		reqwest::StatusCode::RESET_CONTENT => send_point_loss(base),
		_ => (),
	}
}
