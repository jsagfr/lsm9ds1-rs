// macro_rules! vec {
//     ( $( $x:expr ),* ) => {
//         {
//             let mut temp_vec = Vec::new();
//             $(
//                 temp_vec.push($x);
//             )*
//             temp_vec
//         }
//     };
// }
macro_rules! config {
    ( $( $id:ident : $t:ty = $e:expr),* ) => {
        #[derive(Clone, Copy, Debug)]
        struct ConfigBuilder {
            $(
                $id: $t,
            )*
        }

        #[derive(Clone, Copy, Debug)]
        struct Config {
            $(
                $id: $t,
            )*
        }

        impl ConfigBuilder {
            fn new() -> ConfigBuilder {
                ConfigBuilder {
                    $(
                        $id: $e,
                    )*
                }
            }

            fn finalize(&self) -> Config {
                Config {
                    $(
                        $id: self.$id,
                    )*
                }
            }


            $(
                fn $id<'a>(&'a mut self, value: $t) -> &'a mut ConfigBuilder {
                    self.$id = value;
                    self
                }
            )*
        }
    }
}

config! {
    sleep_on : Enable = Enable::Off,
    act_ths  : u8 = 0,
    act_dur  : u8 = 0,
    aoi_xl   : Enable = Enable::Off,
    inter_6d : Enable = Enable::Off,
    zhie_xl  : Enable = Enable::Off,
    zlie_xl  : Enable = Enable::Off,
    yhie_xl  : Enable = Enable::Off,
    ylie_xl  : Enable = Enable::Off,
    xhie_xl  : Enable = Enable::Off,
    xlie_xl  : Enable = Enable::Off
}


#[derive(Clone, Copy, Debug)]
pub enum Enable {
    On,
    Off,
}

#[cfg(test)]
mod tests {
    use super::{ConfigBuilder, Enable};
    
    #[test]
    fn it_works() {
        let b = ConfigBuilder::new()
            .act_ths(12)
            .sleep_on(Enable::On)
            .finalize();
    }
}
