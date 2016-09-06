macro_rules! set_value {
    ($self_:ident, $reg:ident, $set:ident, $value:ident) => {{
        let mut reg = $self_.config.$reg.clone();
        reg.$set($value);
        try!($self_.interface.write(reg.addr(), reg.reg()));
        $self_.config.$reg = reg;
        Ok(())
    }};
}

macro_rules! set_value16 {
    ($self_:ident, $reg:ident, $set:ident, $value:ident) => {{
        let mut reg = $self_.config.$reg.clone();
        reg.$set($value);
        try!($self_.interface.write16(reg.addr(), reg.reg()));
        $self_.config.$reg = reg;
        Ok(())
    }};
}
