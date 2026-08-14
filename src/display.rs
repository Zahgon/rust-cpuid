use std::fmt::Display;

use crate::{
    Associativity, CacheType, CpuIdResult, DatType, ExtendedRegisterStateLocation, SgxSectionInfo,
    SoCVendorBrand, TopologyType,
};

use termimad::{minimad::TextTemplate, minimad::TextTemplateExpander, MadSkin};

pub fn raw<R: crate::CpuIdReader>(cpuid: R) { panic!("STUB: not implemented") }

fn table2(skin: &MadSkin, attrs: &[(&str, String)]) { panic!("STUB: not implemented") }

fn table3(skin: &MadSkin, attrs: &[(&'static str, &'static str, String)]) { panic!("STUB: not implemented") }

fn print_title_line(skin: &MadSkin, title: &str, attr: Option<&str>) { panic!("STUB: not implemented") }

fn print_title_attr(skin: &MadSkin, title: &str, attr: &str) { panic!("STUB: not implemented") }

fn print_title(skin: &MadSkin, title: &str) { panic!("STUB: not implemented") }

fn print_subtitle(skin: &MadSkin, title: &str) { panic!("STUB: not implemented") }

fn print_attr<T: Display, A: Display>(skin: &MadSkin, name: T, attr: A) { panic!("STUB: not implemented") }

fn print_cpuid_result<T: Display>(skin: &MadSkin, name: T, attr: CpuIdResult) { panic!("STUB: not implemented") }

fn bool_repr(x: bool) -> String { panic!("STUB: not implemented") }

trait RowGen {
    fn fmt(attr: &Self) -> String;

    fn tuple<'a>(t: &'a str, attr: Self) -> (&'a str, String)
    where
        Self: Sized,
    { panic!("STUB: not implemented") }

    fn triple<'a>(c: &'a str, t: &'a str, attr: Self) -> (&'a str, &'a str, String)
    where
        Self: Sized,
    { panic!("STUB: not implemented") }
}

impl RowGen for bool {
    fn fmt(attr: &Self) -> String { panic!("STUB: not implemented") }
}

impl RowGen for u64 {
    fn fmt(attr: &Self) -> String { panic!("STUB: not implemented") }
}

impl RowGen for usize {
    fn fmt(attr: &Self) -> String { panic!("STUB: not implemented") }
}

impl RowGen for u32 {
    fn fmt(attr: &Self) -> String { panic!("STUB: not implemented") }
}

impl RowGen for u16 {
    fn fmt(attr: &Self) -> String { panic!("STUB: not implemented") }
}

impl RowGen for u8 {
    fn fmt(attr: &Self) -> String { panic!("STUB: not implemented") }
}

impl RowGen for String {
    fn fmt(attr: &Self) -> String { panic!("STUB: not implemented") }
}

impl RowGen for Associativity {
    fn fmt(attr: &Self) -> String { panic!("STUB: not implemented") }
}

impl RowGen for CacheType {
    fn fmt(attr: &Self) -> String { panic!("STUB: not implemented") }
}

impl RowGen for TopologyType {
    fn fmt(attr: &Self) -> String { panic!("STUB: not implemented") }
}

impl RowGen for ExtendedRegisterStateLocation {
    fn fmt(attr: &Self) -> String { panic!("STUB: not implemented") }
}

impl RowGen for DatType {
    fn fmt(attr: &Self) -> String { panic!("STUB: not implemented") }
}

impl RowGen for Option<SoCVendorBrand> {
    fn fmt(attr: &Self) -> String { panic!("STUB: not implemented") }
}

pub fn markdown<R: crate::CpuIdReader>(cpuid: crate::CpuId<R>) { panic!("STUB: not implemented") }
