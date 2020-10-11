use super::{
    bud_property::{BudProperty, EqualizerType},
    ids, simple, Msg,
};

#[derive(Debug)]
pub struct Simple {
    pub data: u8,
    msg_id: u8,
    response: bool,
}

/// New simple message
pub fn new(msg_id: u8, data: u8) -> Simple {
    Simple {
        msg_id,
        data,
        response: false,
    }
}

/// New simple response message
pub fn new_response(msg_id: u8, data: u8) -> Simple {
    Simple {
        msg_id,
        data,
        response: true,
    }
}

impl Msg for Simple {
    fn get_data(&self) -> Vec<u8> {
        vec![self.data]
    }

    fn get_id(&self) -> u8 {
        self.msg_id
    }

    fn is_response(&self) -> bool {
        self.response
    }
}

// 'simple' based messages used in the protocol

pub fn new_equalizer(d: EqualizerType) -> simple::Simple {
    simple::new(ids::EQUALIZER, d.encode())
}

pub fn new_adjust_sound_sync(adjust: bool) -> simple::Simple {
    simple::new(ids::ADJUST_SOUND_SYNC, adjust.into())
}