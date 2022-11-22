use std::alloc::{alloc, dealloc, Layout};

pub enum Alteration {
    //Replacement { position: usize, length: usize, value: String},
    Insersion { position: usize, value: String},
    Deltetion { position: usize, length: usize}
}

#[derive(Default)]
pub struct Codegen {
    changes: Vec<Alteration>,
    length_difference: i32
}


impl Codegen {
    pub fn append_change (&mut self, alteration: Alteration) {
        self.length_difference += match &alteration {
            //Alteration::Replacement { value, length, ..} => value.len() as i32 - *length as i32,
            Alteration::Insersion { value, ..} => value.as_bytes().len() as i32,
            Alteration::Deltetion { length, ..} => -(*length as i32)
        };
        self.changes.push(alteration);
    }

    pub fn apply (&mut self, input: &String) -> Result< (*mut u8, isize), &str> {
        let input = input.as_bytes();

        let buffer_length: isize = (input.len() as i32 + self.length_difference) as isize;
        if buffer_length < 0 { return Err("Tried to delete past the end of the input string"); }
        let layout = Layout::from_size_align(buffer_length as usize, 1).unwrap();
        unsafe {
            let buffer_ptr: *mut u8 = alloc(layout);
            let mut input_char_ptr: isize = 0;
            let mut offset_output_char_ptr: isize = 0;
            let mut last_editied_position: usize = 0;
            //let mut deleted_at 
            while self.changes.len() > 0 {
                match self.changes.pop().unwrap() {
                    Alteration::Insersion { position, value } => {
                        //if input_char_ptr > position as isize { input_char_ptr = position }
                        if last_editied_position > position { dealloc(buffer_ptr, layout); return Err("Alterations are out of order"); }
                        while input_char_ptr < position as isize {
                            *(buffer_ptr.offset(input_char_ptr + offset_output_char_ptr)) = input[input_char_ptr as usize];
                            input_char_ptr += 1;
                        }
                        let ins_bytes = value.as_bytes();
                        let ins_len = ins_bytes.len();
                        for i in 0..ins_len {
                            *(buffer_ptr.offset(i as isize + input_char_ptr + offset_output_char_ptr)) = ins_bytes[i as usize];
                        }
                        offset_output_char_ptr += ins_len as isize;
                        last_editied_position = position;
                    },
                    Alteration::Deltetion { position, length } => {
                        if last_editied_position > position { dealloc(buffer_ptr, layout); return Err("Alterations are out of order"); }
                        while input_char_ptr < position as isize {
                            *(buffer_ptr.offset(input_char_ptr + offset_output_char_ptr)) = input[input_char_ptr as usize];
                            input_char_ptr += 1;
                        }
                        input_char_ptr += length as isize;
                        offset_output_char_ptr -= length as isize ;
                        last_editied_position = position;   
                    }
                };
            }
            let input_len = input.len() as isize;
            while input_char_ptr < input_len {
                *(buffer_ptr.offset(input_char_ptr + offset_output_char_ptr)) = input[input_char_ptr as usize];
                input_char_ptr += 1;
            }

            for i in 0..buffer_length {
                let a = *(buffer_ptr.offset(i as isize));
                print!("{}", a as char);
            };
            print!("\n");
            Ok((buffer_ptr, buffer_length))
        }
    }
}