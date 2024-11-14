use typst::symbols::emoji;
use typst::symbols::sym;

fn main() {
    let mut vec_result = vec!["return {\n".to_string()];
    let syms = sym();
    for (name, sym, _) in syms.scope().iter() {
        if let typst::foundations::Value::Symbol(s) = sym {
            for (ss, symym) in s.variants() {
                vec_result.push(format!(
                    "{{ word = \'\\\\{name}.{ss}\', label = \'\\\\{name}.{ss}  {}\', insertText = \'{}\', filterText = \'\\\\{name}.{ss}\' }},\n",
                    symym.char(),
                    symym.char()
                ));
                println!(
                    "{{ word = \'\\\\{name}.{ss}\', label = \'\\\\{name}.{ss}  {}\', insertText = \'{}\', filterText = \'\\\\{name}.{ss}\' }},\n",
                    symym.char(),
                    symym.char()
                );
            }
        }
    }
    vec_result.push("}".to_string());
    // write vec_result to file
    std::fs::write("items_typst.lua", vec_result.join("")).unwrap();

    let mut vec_result = vec!["return {\n".to_string()];
    let syms = emoji();
    for (name, sym, _) in syms.scope().iter() {
        if let typst::foundations::Value::Symbol(s) = sym {
            for (ss, symym) in s.variants() {
                vec_result.push(format!(
                    "{{ word = \'\\\\{name}.{ss}\', label = \'\\\\{name}.{ss}  {}\', insertText = \'{}\', filterText = \'\\\\{name}.{ss}\' }},\n",
                    symym.char(),
                    symym.char()
                ));
                println!(
                    "{{ word = \'\\\\{name}.{ss}\', label = \'\\\\{name}.{ss}  {}\', insertText = \'{}\', filterText = \'\\\\{name}.{ss}\' }},\n",
                    symym.char(),
                    symym.char()
                );
            }
        }
    }
    vec_result.push("}".to_string());
    // write vec_result to file
    std::fs::write("items_typst_emoji.lua", vec_result.join("")).unwrap();
}
