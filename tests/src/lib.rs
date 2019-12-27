use short_name_derive::AsShortName;
use short_name_traits::ShortName;

#[derive(Default, AsShortName)]
pub struct NoFieldsStruct;

#[derive(Default, AsShortName)]
pub struct UnnamedFieldsStruct(String);

#[derive(Default, AsShortName)]
pub struct NamedFieldsStruct {
    _field: String,
}

#[derive(AsShortName)]
pub enum UnnamedVariantsEnum {
    UnnamedVariant1(VariantImitation),
    AnotherOneUnnamedVariant(VariantImitation),
    TheLastOneUnnamedVariant(VariantImitation),
}

#[derive(AsShortName)]
pub enum NamedVariantsEnum {
    NamedVariant1 {
        _field: VariantImitation,
    },
    AnotherOneNamedVariant {
        _field1: VariantImitation,
        _field2: VariantImitation,
    },
    TheLastOneNamedVariant {
        _field1: VariantImitation,
        _field2: VariantImitation,
        _field3: VariantImitation,
    },
}

#[derive(AsShortName)]
pub enum UnitsEnum {
    VariantUnit1,
    AnotherOneVariantUnit,
    TheLastOneVariantUnit,
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

#[derive(Default)]
pub struct VariantImitation {
    _field1: String,
    _field2: u8,
    _field3: bool,
}

#[test]
fn as_short_name_for_enum_with_unnamed_variants() {
    let unnamed_variant1 = UnnamedVariantsEnum::UnnamedVariant1(VariantImitation::default());
    let unnamed_variant2 =
        UnnamedVariantsEnum::AnotherOneUnnamedVariant(VariantImitation::default());
    let unnamed_variant3 =
        UnnamedVariantsEnum::TheLastOneUnnamedVariant(VariantImitation::default());

    assert_eq!("Unnamed Variant1", unnamed_variant1.as_short_name());
    assert_eq!(
        "Another One Unnamed Variant",
        unnamed_variant2.as_short_name()
    );
    assert_eq!(
        "The Last One Unnamed Variant",
        unnamed_variant3.as_short_name()
    );
}

#[test]
fn as_short_name_for_enum_with_named_variants() {
    let named_variant1 = NamedVariantsEnum::NamedVariant1 {
        _field: VariantImitation::default(),
    };
    let named_variant2 = NamedVariantsEnum::AnotherOneNamedVariant {
        _field1: VariantImitation::default(),
        _field2: VariantImitation::default(),
    };
    let named_variant3 = NamedVariantsEnum::TheLastOneNamedVariant {
        _field1: VariantImitation::default(),
        _field2: VariantImitation::default(),
        _field3: VariantImitation::default(),
    };

    assert_eq!("Named Variant1", named_variant1.as_short_name());
    assert_eq!("Another One Named Variant", named_variant2.as_short_name());
    assert_eq!("The Last One Named Variant", named_variant3.as_short_name());
}

#[test]
fn as_short_name_for_enum_with_units() {
    let unit_variant1 = UnitsEnum::VariantUnit1;
    let unit_variant2 = UnitsEnum::AnotherOneVariantUnit;
    let unit_variant3 = UnitsEnum::TheLastOneVariantUnit;

    assert_eq!("Variant Unit1", unit_variant1.as_short_name());
    assert_eq!("Another One Variant Unit", unit_variant2.as_short_name());
    assert_eq!("The Last One Variant Unit", unit_variant3.as_short_name());
}

#[test]
fn as_short_name_for_enum_with_mixed_variants() {
    let mixed_variant1 = MixedEnum::UnnamedVariant1(VariantImitation::default());
    let mixed_variant2 = MixedEnum::AnotherOneNamedVariant {
        _field1: VariantImitation::default(),
        _field2: VariantImitation::default(),
    };
    let mixed_variant3 = UnitsEnum::TheLastOneVariantUnit;

    assert_eq!("Unnamed Variant1", mixed_variant1.as_short_name());
    assert_eq!("Another One Named Variant", mixed_variant2.as_short_name());
    assert_eq!("The Last One Variant Unit", mixed_variant3.as_short_name());
}

#[test]
fn as_short_name_for_struct_without_fields() {
    let no_fields_struct = NoFieldsStruct::default();

    assert_eq!("No Fields Struct", no_fields_struct.as_short_name());
}

#[test]
fn as_short_name_for_struct_with_unnamed_field() {
    let unnamed_fields_struct = UnnamedFieldsStruct::default();

    assert_eq!(
        "Unnamed Fields Struct",
        unnamed_fields_struct.as_short_name()
    );
}

#[test]
fn as_short_name_for_struct_with_named_field() {
    let named_fields_struct = NamedFieldsStruct::default();

    assert_eq!("Named Fields Struct", named_fields_struct.as_short_name());
}
