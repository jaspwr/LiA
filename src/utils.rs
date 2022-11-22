pub fn byte_by_byte_token_identifier (token: String) -> impl FnMut(u8) -> Option<usize> {
    // TODO: ignore text node and escape chars
    let _token = token.as_bytes();
    let len = _token.len();
    let mut in_token_count: usize = 0;
    move |byte: u8| -> Option<usize> {
        let token = token.as_bytes();
        return if byte == token[in_token_count] {
            in_token_count += 1;
            if in_token_count >= len {
                in_token_count = 0;
                Some(len)
            } else {
                None
            }
        } else {
            in_token_count = 0;
            None
        }
    }
}

fn char_is_token_name (char: u8) -> bool {
    char >= '!' as u8 
    && char != ',' as u8
    && char != ';' as u8
}

pub fn extract_next_token (bytes: &[u8], start_index: usize) -> Option<(String, usize)> {
    // TODO: ignore text node and escape chars
    let mut ret = Vec::<u8>::new();
    let mut char_ptr = start_index;
    let bytes_len = bytes.len();
    const MAX_LEN: usize = 5000;
    while !char_is_token_name(bytes[char_ptr]) { 
        char_ptr += 1;
        if char_ptr > bytes_len || char_ptr > MAX_LEN { return None; }
    }
    while char_is_token_name(bytes[char_ptr]) {
        ret.push(bytes[char_ptr]);
        char_ptr += 1;
        if char_ptr > bytes_len || char_ptr > MAX_LEN { return None; }
    }
    Some((String::from_utf8(ret).unwrap(), char_ptr))
}