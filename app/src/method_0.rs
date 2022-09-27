use beam_bvm_util::safe::app::doc;

#[no_mangle]
#[allow(non_snake_case)]
fn Method_0() {
    doc::doc_add_group("\0");
    doc::doc_add_group("roles\0");
    doc::doc_add_group("manager\0");

    doc::doc_add_group("create\0");
    doc::doc_close_group(); // create

    doc::doc_add_group("destroy\0");
    doc::doc_add_text("cid\0", "ContractID\0".as_ptr());
    doc::doc_close_group(); // destroy

    doc::doc_add_group("view\0");
    doc::doc_close_group(); // view

    doc::doc_close_group(); // manager
    doc::doc_close_group(); // roles
    doc::doc_close_group(); // \0
}
