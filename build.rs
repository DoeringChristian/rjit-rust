use enum_iterator::{all, Sequence};
use std::fmt::Debug;
use std::io::BufWriter;
use std::io::Write;
use std::process::Command;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Sequence)]
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

    let types = all::<VarType>();

    write!(
        f,
        r#"
        use once_cell::sync::Lazy;
        use std::marker::PhantomData;

        pub static TRACE: Lazy<rjit::Trace> = Lazy::new(|| rjit::Trace::default());
        
        pub struct Var<T>(rjit::VarRef, PhantomData<T>);

        "#
    )?;
    writeln!(f,)?;

    for ty in types.clone() {
        writeln!(f, "")?;

        write!(
            f,
            "
            impl From<{prim}> for Var<{prim}> {{
                fn from(value: {prim}) -> Self {{
                    Self(TRACE.literal(value).unwrap(), PhantomData)
                }}
            }}
            impl<'a> From<&'a [{prim}]> for Var<{prim}> {{
                fn from(value: &'a [{prim}]) -> Self {{
                    Self(TRACE.array(value).unwrap(), PhantomData)
                }}
            }}
               ",
            prim = ty.prim(),
        )?;

        for from in types.clone().filter(|_ty| *_ty != ty) {
            write!(
                f,
                r#"
                        impl From<Var<{from}>> for Var<{to}>{{
                            fn from(value: Var<{from}>) -> Self{{
                                Self(value.0.cast(&rjit::VarType::{to_vt}).unwrap(), PhantomData)
                            }}
                        }}
                        
                        impl From<{from}> for Var<{to}>{{
                            fn from(value: {from}) -> Self{{
                                Var::<{to}>::from(Var::<{from}>::from(value))
                            }}
                        }}
                        impl<'a> From<&'a [{from}]> for Var<{to}>{{
                            fn from(value: &'a [{from}]) -> Self{{
                                Var::<{to}>::from(Var::<{from}>::from(value))
                            }}
                        }}
                       "#,
                to_vt = ty.vt(),
                to = ty.prim(),
                from = from.prim(),
            )?;
        }
    }

    let ops = ["Add", "Sub", "Mul", "Div"];

    // Arythmetic operations table:
    for Op in ops {
        for lhs in types.clone() {
            for rhs in types.clone() {
                let output = lhs.max(rhs);

                write!(
                    f,
                    "
                        impl std::ops::{Op}<Var<{rhs}>> for Var<{lhs}> {{
                            type Output = Var<{output}>;

                            fn {op}(self, rhs: Var<{rhs}>) -> Self::Output {{
                                let lhs = self.0.cast(&rjit::VarType::{Output}).unwrap();
                                let rhs = rhs.0.cast(&rjit::VarType::{Output}).unwrap();
                                Var(lhs.{op}(&rhs).unwrap(), PhantomData)
                            }}
                        }}
                        
                        impl std::ops::{Op}<{rhs}> for Var<{lhs}> {{
                            type Output = Var<{output}>;

                            fn {op}(self, rhs: {rhs}) -> Self::Output {{
                                let lhs = self.0.cast(&rjit::VarType::{Output}).unwrap();
                                let rhs = Var::<{output}>::from(rhs);
                                Var(lhs.{op}(&rhs.0).unwrap(), PhantomData)
                            }}
                        }}
                       ",
                    lhs = lhs.prim(),
                    rhs = rhs.prim(),
                    output = output.prim(),
                    Output = output.vt(),
                    op = Op.to_lowercase(),
                )?;
            }
        }
    }

    Command::new("cargo").arg("fmt").spawn().unwrap();

    Ok(())
}
