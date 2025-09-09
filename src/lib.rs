use core::fmt;
pub use static_automata_macros::{grammar, Validate};

#[derive(Debug)]
pub struct Utf8Error;

impl fmt::Display for Utf8Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Invalid UTF-8")
	}
}

impl core::error::Error for Utf8Error {}

const fn read_extended(bytes: &[u8], i: usize) -> Result<u8, Utf8Error> {
	let b = bytes[i];

	if b & 0xc0 != 0x80 {
		return Err(Utf8Error);
	}

	Ok(b & 0x3f)
}

pub const fn decode_utf8_char(bytes: &[u8], i: usize) -> Result<(char, usize), Utf8Error> {
	if bytes.len() <= i {
		return Err(Utf8Error);
	}

	let a = bytes[i];
	let (code_point, len) = if a & 0x80 == 0 {
		// 1 byte
		(a as u32 & 0x7f, 1)
	} else if a & 0xe0 == 0xc0 {
		// 2 bytes.
		if bytes.len() <= i + 1 {
			return Err(Utf8Error);
		}

		let Ok(b) = read_extended(bytes, i + 1) else {
			return Err(Utf8Error);
		};

		let code_point = (a as u32 & 0x1f) << 6 | b as u32;

		(code_point, 2)
	} else if a & 0xf0 == 0xe0 {
		// 3 bytes.
		if bytes.len() <= i + 2 {
			return Err(Utf8Error);
		}

		let Ok(b) = read_extended(bytes, i + 1) else {
			return Err(Utf8Error);
		};

		let Ok(c) = read_extended(bytes, i + 2) else {
			return Err(Utf8Error);
		};

		let code_point = (a as u32 & 0x0f) << 12 | (b as u32) << 6 | c as u32;

		(code_point, 3)
	} else if a & 0xf8 == 0xf0 {
		// 4 bytes.
		if bytes.len() <= i + 3 {
			return Err(Utf8Error);
		}

		let Ok(b) = read_extended(bytes, i + 1) else {
			return Err(Utf8Error);
		};

		let Ok(c) = read_extended(bytes, i + 2) else {
			return Err(Utf8Error);
		};

		let Ok(d) = read_extended(bytes, i + 3) else {
			return Err(Utf8Error);
		};

		let code_point = (a as u32 & 0x07) << 18 | (b as u32) << 12 | (c as u32) << 6 | d as u32;

		(code_point, 4)
	} else {
		return Err(Utf8Error);
	};

	// Surrogate code point.
	if code_point >= 0xD800 && code_point <= 0xDFFF {
		return Err(Utf8Error);
	}

	// Overflow.
	if code_point > 0x10FFFF {
		return Err(Utf8Error);
	}

	Ok((unsafe { char::from_u32_unchecked(code_point) }, len))
}

#[cfg(test)]
mod tests {
	use super::*;

	fn decode_utf8(bytes: &[u8]) -> String {
		let mut i = 0;

		let mut result = String::new();
		while i < bytes.len() {
			let (c, len) = decode_utf8_char(bytes, i).unwrap();
			i += len;
			result.push(c);
		}

		result
	}

	const VALID: [&str; 13] = [
		// Basic Latin.
		"Hello, World!",
		// Greek and Coptic.
		"Γειά σου Κόσμε",
		// Cyrillic.
		"Привет, мир!",
		// Arabic.
		"مرحبا بالعالم",
		// Devanagari.
		"नमस्ते दुनिया",
		// Hebrew.
		"שלום עולם",
		// Thai.
		"สวัสดีโลก",
		// Hiragana (Japanese).
		"こんにちは",
		// Katakana (Japanese).
		"コンニチハ",
		// CJK Unified Ideographs (Chinese).
		"你好，世界",
		// Mathematical Operators.
		"∀x ∈ ℝ: ∃y ≥ 0",
		// Emoji.
		"😀🚀✨",
		// Boundaries.
		"\0\u{10FFFF}",
	];

	#[test]
	fn valid() {
		for string in VALID {
			let decoded = decode_utf8(string.as_bytes());
			assert_eq!(string, decoded);
		}
	}

	const INVALID: [&[u8]; 3] = [
		&[0b1111_0100, 0b10_010000, 0b10_000000, 0b10_000000], // 0x110000
		&[0b1110_1101, 0b10_100000, 0b10_000000],              // 0xD800
		&[0b1110_1101, 0b10_111111, 0b10_111111],              // 0xDFFF
	];

	#[test]
	fn invalid() {
		for string in INVALID {
			assert!(decode_utf8_char(string, 0).is_err());
		}
	}
}
