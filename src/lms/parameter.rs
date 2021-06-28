use crate::{
    constants::MAX_N,
    hasher::{sha256::Sha256Hasher, Hasher},
};

pub trait LmsParameter: Hasher {
    const H: u8;
    const M: u8;
    const TYPE: u32;

    fn is_type_correct(_type: u32) -> bool {
        Self::TYPE == _type
    }

    fn number_of_lm_ots_keys() -> usize {
        2usize.pow(Self::H as u32)
    }
}

macro_rules! generate_parameter_type {
    ($name:ident, $h:literal, $m:literal, $type:literal, $hasher:ident) => {
        pub struct $name {
            hasher: $hasher,
        }

        impl LmsParameter for $name {
            const H: u8 = $h;
            const M: u8 = $m;
            const TYPE: u32 = $type;
        }

        impl Hasher for $name {
            fn get_hasher() -> Self {
                $name {
                    hasher: $hasher::new(),
                }
            }
            fn update(&mut self, data: &[u8]) {
                self.hasher.update(data)
            }

            fn finalize(self) -> crate::util::dynamic_array::DynamicArray<u8, MAX_N> {
                self.hasher.finalize()
            }

            fn finalize_reset(&mut self) -> crate::util::dynamic_array::DynamicArray<u8, MAX_N> {
                self.hasher.finalize_reset()
            }
        }
    };
}

generate_parameter_type!(LmsSha256M32H5, 5, 32, 5, Sha256Hasher);
generate_parameter_type!(LmsSha256M32H10, 10, 32, 6, Sha256Hasher);
generate_parameter_type!(LmsSha256M32H15, 15, 32, 7, Sha256Hasher);
generate_parameter_type!(LmsSha256M32H20, 20, 32, 8, Sha256Hasher);
generate_parameter_type!(LmsSha256M32H25, 25, 32, 9, Sha256Hasher);
