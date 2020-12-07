use binars::{Op, Type};

fn main()
{
    let mut m = binars::Module::new();
    {
        let body = {
            let children = {
                let l = m.get_local(0, Type::int_32());
                let r = m.get_local(1, Type::int_32());
                let op = Op::add_int32();
                vec![m.binary(op, l, r)]
            };
            m.new_block("main-block", children, Type::auto())
        };
        m.add_function(
            "add",
            Type::create(vec![Type::int_32(), Type::int_32()]),
            Type::int_32(),
            vec![Type::int_32(); 2],
            body,
        );
    }
    m.print();
}
