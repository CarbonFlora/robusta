use robusta_term::init::using_dxf_init;

fn main() {
    let d = "robusta-term/tests/resources/text_only.dxf".to_string();
    using_dxf_init(d);
}
