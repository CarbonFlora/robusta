use robusta_term::init::using_dxf_init;

fn main() {
    let d = "robusta-dxf/tests/resources/text_mtext.dxf".to_string();
    using_dxf_init(d);
}
