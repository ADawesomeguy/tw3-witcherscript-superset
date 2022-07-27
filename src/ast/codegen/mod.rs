use std::ops::Deref;

pub mod context;
mod emit_data;

pub use emit_data::EmitAdditionalData;

pub trait Codegen {
  fn emit(
    &self, _: &context::Context, output: &mut Vec<u8>, _: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    write!(output, "##default Codegen::emit impl##")?;

    Ok(())
  }

  fn emit_join(
    &self, _: &context::Context, _: &mut Vec<u8>, _: &'static str, _: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    unimplemented!("default emit_join impl called");
  }
}

impl<A> Codegen for Vec<A>
where
  A: Codegen,
{
  fn emit(
    &self, context: &context::Context, output: &mut Vec<u8>, data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    for child in self {
      child.emit(context, output, data)?;
    }

    Ok(())
  }

  fn emit_join(
    &self, context: &context::Context, output: &mut Vec<u8>, join_char: &'static str,
    data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    use std::io::Write as IoWrite;

    let mut children = self.iter().peekable();

    while let Some(child) = children.next() {
      child.emit(context, output, data)?;

      if children.peek().is_some() {
        write!(output, "{}", join_char)?;
      }
    }

    Ok(())
  }
}

impl<A> Codegen for std::boxed::Box<A>
where
  A: Codegen,
{
  fn emit(
    &self, context: &context::Context, output: &mut Vec<u8>, data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    self.deref().emit(context, output, data)
  }
}

impl<A> Codegen for std::rc::Rc<A>
where
  A: Codegen,
{
  fn emit(
    &self, context: &context::Context, output: &mut Vec<u8>, data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    self.deref().emit(context, output, data)
  }
}

impl<A> Codegen for Option<A>
where
  A: Codegen,
{
  fn emit(
    &self, context: &context::Context, output: &mut Vec<u8>, data: &Option<EmitAdditionalData>,
  ) -> Result<(), std::io::Error> {
    if let Some(inner) = self {
      inner.emit(context, output, data)?;
    }

    Ok(())
  }
}
