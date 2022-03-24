use msvc_demangler::{demangle, Name, Operator, Params, ParseResult, Type};
use pelite::pe32::{Pe, PeFile};
use std::collections::{HashMap, HashSet};

fn blacklist() -> HashSet<&'static str> {
  HashSet::from_iter(vec![])
}

pub fn dump(image: &[u8]) -> pelite::Result<HashMap<String, Vec<Parsed>>> {
  // Interpret the bytes as a PE32+ executable
  let file = PeFile::from_bytes(image)?;

  let mut parsed = vec![];
  let mut skipped = vec![];

  // For example: iterate over the named exports
  for result in file.exports()?.by()?.iter_names() {
    if let (Ok(name), Ok(_)) = result {
      if blacklist().contains(name.to_str().unwrap()) {
        continue;
      }

      let symbol = Symbol::new(name.to_str().unwrap());
      let res = symbol.to_rust();

      match res {
        Ok(symbol) => parsed.push(symbol),
        Err(err) => skipped.push((symbol, err)),
      }
    }
  }

  let mut by_class = HashMap::new();

  for parsed in parsed {
    by_class
      .entry(parsed.class.clone())
      .or_insert(Vec::new())
      .push(parsed);
  }

  Ok(by_class)
}

#[derive(Debug)]
pub enum Error {
  UnsupportedType(String),
  UnsupportedName(String),
}

pub struct Symbol {
  name: String,
  demangled: String,
}

impl Symbol {
  pub fn new(name: impl Into<String>) -> Self {
    let name: String = name.into();
    let demangled = demangle(&name, msvc_demangler::DemangleFlags::llvm()).unwrap();

    Self { name, demangled }
  }

  pub fn mangled(&self) -> &str {
    &self.name
  }

  pub fn demangled(&self) -> &str {
    &self.demangled
  }

  pub fn parsed(&self) -> ParseResult<'_> {
    msvc_demangler::parse(&self.name).unwrap()
  }

  pub fn to_rust(&self) -> Result<Parsed, Error> {
    let parsed = self.parsed();

    if let Type::MemberFunction(_, _, params, _, ret) = &parsed.symbol_type {
      let mut referenced_types = vec![];

      let name = parse_name(&parsed.symbol.name)?;

      let class = parse_names(parsed.symbol.scope.names.iter().rev())?;

      referenced_types.push(class.clone());

      if name == "defaultctorclosure" {
        return Err(Error::UnsupportedName(format!("defaultctorclosure")));
      }

      let params = parse_params(params)?;

      for param in &params {
        if !param.starts_with("libc")
          && !param.starts_with("NonNull")
          && !param.starts_with("*const")
        {
          referenced_types.push(param.clone());
        }
      }

      let ret = match **ret {
        Type::None => None,
        Type::Void(_) => None,
        _ => Some(parse_type(ret)?),
      };

      let params = params
        .iter()
        .enumerate()
        .map(|(i, name)| format!("p{i}: {}", name))
        .collect::<Vec<_>>()
        .join(", ");

      let demangled = self.demangled();
      let mangled = self.mangled();

      let ret = match ret {
        Some(ret) => format!(" -> {ret}"),
        None => format!(""),
      };

      let parsed = format!(
        r#"
#[link_name = "{mangled}"]
// {demangled}
fn {class}_{name}(this: NonNull<{class}>, {params}){ret};
"#
      );

      return Ok(Parsed { class, parsed });
    }

    Err(Error::UnsupportedType(format!("{:?}", parsed.symbol_type)))
  }
}

pub struct Parsed {
  class: String,
  pub parsed: String,
}

fn parse_name(name: &Name) -> Result<String, Error> {
  Ok(match name {
    Name::NonTemplate(bytes) => String::from_utf8_lossy(bytes).to_string(),
    Name::Operator(Operator::Ctor) => format!("constructor"),
    Name::Operator(Operator::Dtor) => format!("destructor"),
    Name::Operator(op) => format!("{:?}", op).to_ascii_lowercase(),
    _ => Err(Error::UnsupportedName(format!("{:?}", name)))?,
  })
}

fn parse_names<'a>(names: impl Iterator<Item = &'a Name<'a>>) -> Result<String, Error> {
  Ok(
    names
      .map(parse_name)
      .collect::<Result<Vec<_>, _>>()?
      .join("_"),
  )
}

fn parse_type(ty: &Type) -> Result<String, Error> {
  Ok(match ty {
    Type::Ref(ty, _) => format!("NonNull<{}>", parse_type(ty)?),
    Type::Ptr(ty, _) => format!("*const {}", parse_type(ty)?),

    Type::Struct(symbol, _) => format!("{}", parse_name(&symbol.name)?),
    Type::Class(symbol, _) => format!("{}", parse_name(&symbol.name)?),
    Type::Enum(symbol, _) => format!("libc::c_int /* {} */", parse_name(&symbol.name)?),

    Type::Int(_) => format!("libc::c_int"),
    Type::Ushort(_) => format!("libc::c_ushort"),
    Type::Float(_) => format!("libc::c_float"),

    Type::Bool(_) => format!("libc::c_bool"),
    Type::Char(_) => format!("libc::c_char"),

    Type::None => format!("<none>"),
    _ => Err(Error::UnsupportedType(format!("{:?}", ty)))?,
  })
}

fn parse_params(params: &Params) -> Result<Vec<String>, Error> {
  params
    .types
    .iter()
    .filter(|ty| if let Type::Void(_) = ty { false } else { true })
    .map(parse_type)
    .collect::<Result<Vec<_>, _>>()
}
