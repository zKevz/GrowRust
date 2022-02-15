use byteorder::WriteBytesExt;

use super::variant::Variant;

#[derive(Debug, Clone)]
pub struct VariantList<'a> {
    variants: Vec<Variant<'a>>,
}

impl<'a> VariantList<'a> {
    pub fn new(arr: &[Variant<'a>]) -> Self {
        Self {
            variants: arr.to_vec(),
        }
    }

    pub fn push<IntoVariant>(&mut self, variant: IntoVariant)
    where
        IntoVariant: Into<Variant<'a>>,
    {
        self.variants.push(variant.into());
    }

    pub fn serialize(&self) -> std::io::Result<Vec<u8>> {
        let mut vec = Vec::<u8>::with_capacity(512); // half a kb
        vec.write_u8(self.variants.len() as u8)?;

        for (i, variant) in self.variants.iter().enumerate() {
            vec.write_u8(i as u8)?;
            variant.serialize(&mut vec)?;
        }

        Ok(vec)
    }
}
