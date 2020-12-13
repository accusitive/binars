use binars::{ExpressionRef, Module, Op, Type};
fn math(m: &mut Module, op: Op, ty: Type) -> ExpressionRef
{
    {
        let body = {
            let children = {
                let l = m.get_local(0, ty);
                let r = m.get_local(1, ty);
                vec![m.binary(op, l, r)]
            };
            m.new_block("main-block", children, Type::auto())
        };
        body
    }
}
fn main()
{
    let mut m = Module::new();
    {
        let body = math(&mut m, Op::add_int32(), Type::int_32());
        m.add_function(
            "addi",
            Type::create(vec![Type::int_32(), Type::int_32()]),
            Type::int_32(),
            vec![Type::int_32(); 2],
            body,
        );
    }
    {
        let body = math(&mut m, Op::sub_float32(), Type::float_32());
        m.add_function(
            "subf",
            Type::create(vec![Type::float_32(), Type::float_32()]),
            Type::float_32(),
            vec![Type::float_32(); 2],
            body,
        );
    }
    m.print();
    m.validate();
}
