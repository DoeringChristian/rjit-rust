use std::fmt::Debug;
use std::io::BufWriter;
use std::io::Write;
use std::process::Command;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum VarType {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
}

impl VarType {
    pub fn prim(self) -> &'static str {
        match self {
            VarType::Bool => "bool",
            VarType::I8 => "i8",
            VarType::U8 => "u8",
            VarType::I16 => "i16",
            VarType::U16 => "u16",
            VarType::I32 => "i32",
            VarType::U32 => "u32",
            VarType::I64 => "i64",
            VarType::U64 => "u64",
            VarType::F32 => "f32",
            VarType::F64 => "f64",
        }
    }
    pub fn ty(self) -> &'static str {
        match self {
            VarType::Bool => "Bool",
            VarType::I8 => "Int8",
            VarType::U8 => "UInt8",
            VarType::I16 => "Int16",
            VarType::U16 => "UInt16",
            VarType::I32 => "Int32",
            VarType::U32 => "UInt32",
            VarType::I64 => "Int64",
            VarType::U64 => "UInt64",
            VarType::F32 => "Float32",
            VarType::F64 => "Float64",
        }
    }
    pub fn vt(self) -> &'static str {
        match self {
            VarType::Bool => "Bool",
            VarType::I8 => "I8",
            VarType::U8 => "U8",
            VarType::I16 => "I16",
            VarType::U16 => "U16",
            VarType::I32 => "I32",
            VarType::U32 => "U32",
            VarType::I64 => "I64",
            VarType::U64 => "U64",
            VarType::F32 => "F32",
            VarType::F64 => "F64",
        }
    }
}

fn main() -> std::io::Result<()> {
    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("src/autogen.rs")?;
    let mut f = BufWriter::new(f);

    let types = [VarType::F32, VarType::F64];

    write!(
        f,
        r#"
        use once_cell::sync::Lazy;

        pub static TRACE: Lazy<rjit::Trace> = Lazy::new(|| rjit::Trace::default());

        "#
    )?;
    writeln!(f,)?;

    for ty in types {
        writeln!(f, "pub struct {ty}(pub rjit::VarRef);", ty = ty.ty())?;
        writeln!(f, "")?;

        write!(
            f,
            "
                impl From<{prim}> for {ty} {{
                   fn from(value: {prim}) -> Self{{
                        Self(
                        TRACE.literal(value).unwrap()
                        )
                   }}
                }}
                impl<'a> From<&'a [{prim}]> for {ty}{{
                    fn from(value: &'a [{prim}]) -> Self {{
                        Self(
                            TRACE
                                .array(value)
                                .unwrap(),
                        )
                    }}
                }}
               ",
            prim = ty.prim(),
            ty = ty.ty(),
        )?;

        for from in types.iter().filter(|_ty| **_ty != ty) {
            write!(
                f,
                r#"
                    impl From<{from}> for {to}{{
                        fn from(value: {from}) -> Self{{
                            Self(value.0.cast(&rjit::VarType::{to_vt}).unwrap())
                        }}
                    }}
                   "#,
                to = ty.ty(),
                to_vt = ty.vt(),
                from = from.ty(),
            )?;
            // write!(
            //     f,
            //     r#"
            //         impl From<{prim}> for {to}{{
            //             fn from(value: {prim}) -> Self{{
            //                 {from}::from(value).into()
            //             }}
            //         }}
            //         impl<'a> From<&'a [{prim}]> for {to}{{
            //             fn from(value: &'a [{prim}]) -> Self{{
            //                 {from}::from(value).into()
            //             }}
            //         }}
            //        "#,
            //     to = ty.ty(),
            //     from = from.ty(),
            //     prim = from.prim(),
            // )?;
        }
    }

    let ops = ["Add", "Mul"];

    // Arythmetic operations table:
    for Op in ops {
        for lhs in types {
            for rhs in types {
                let output = lhs.max(rhs);

                write!(
                    f,
                    "
                    impl std::ops::{Op}<{rhs}> for {lhs} {{
                        type Output = {output};

                        fn {op}(self, rhs: {rhs}) -> Self::Output {{
                            {output}(self.0.{op}(&rhs.0).unwrap())
                        }}
                    }}
                   ",
                    lhs = lhs.ty(),
                    rhs = rhs.ty(),
                    output = output.ty(),
                    op = Op.to_lowercase(),
                )?;
            }
        }
    }

    Command::new("cargo").arg("fmt").spawn().unwrap();

    Ok(())
}
