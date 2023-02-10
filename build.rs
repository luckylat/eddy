fn main(){
    glib_build_tools::compile_resources(
        "resources",
        "resources/eddy.gresource.xml",
        "eddy.gresource",
    );
}
