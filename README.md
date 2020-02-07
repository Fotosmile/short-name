# short-name

[![Build Status](https://travis-ci.org/Fotosmile/short-name.svg?branch=master)](https://travis-ci.org/Fotosmile/short-name)

A Rust library for generation a struct/enum variants short name.

## Usage

Consider you have an enum with some variants, 
and you have added `AsShortName` derive attribute for the enum: 

```rust
#[derive(Default)]
pub struct VariantImitation {
    _field1: String,
    _field2: u8,
    _field3: bool,
}

#[derive(AsShortName)]
pub enum MixedEnum {
    UnnamedVariant1(VariantImitation),
    AnotherOneNamedVariant {
        _field1: VariantImitation,
        _field2: VariantImitation,
    },
    TheLastOneVariantUnit,
}

fn main() {
    let mixed_variant1= MixedEnum::UnnamedVariant1(VariantImitation::default());
    let mixed_variant2 = MixedEnum::AnotherOneNamedVariant {
        _field1: VariantImitation::default(),
        _field2: VariantImitation::default(),
    };
    let mixed_variant3 = UnitsEnum::TheLastOneVariantUnit;
    
    assert_eq!("Unnamed Variant1", mixed_variant1.as_short_name());
    assert_eq!("Another One Named Variant", mixed_variant2.as_short_name());
    assert_eq!("The Last One Variant Unit", mixed_variant3.as_short_name());
}
```
Then you will be able to print just a variant name using `.as_short_name()` method, 
instead of a full debug info with enum name, fields name, 
fields values using `Debug` attribute.