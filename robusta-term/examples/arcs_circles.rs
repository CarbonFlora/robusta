use robusta_term::init::using_dxf_init;

fn main() {
    let d = "robusta-dxf/tests/resources/arcs_circles-2018.dxf".to_string();
    using_dxf_init(d);
}
