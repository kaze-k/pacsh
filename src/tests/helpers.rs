use crate::modules::Module;

pub(crate) fn render_to_string<M: Module>(module: &M, gap: usize) -> String {
    let mut output = Vec::new();
    module
        .render(&mut output, gap)
        .expect("render should succeed");

    String::from_utf8(output).expect("render output should be UTF-8")
}
