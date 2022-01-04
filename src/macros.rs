macro_rules! unwrap_or_continue {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(_) => {
                continue;
            }
        }
    };
}

macro_rules! unwrap_or_return {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(_) => {
                return;
            }
        }
    };
}

macro_rules! unwrap_or_dbg {
    ($res:expr) => {
        match $res {
            Ok(val) => val,
            Err(e) => {
                dbg!(e);
                return;
            }
        }
    };
}

pub(crate) use unwrap_or_continue;
pub(crate) use unwrap_or_dbg;
pub(crate) use unwrap_or_return;
