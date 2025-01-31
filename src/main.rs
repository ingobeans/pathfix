use clipboard_win::{formats, get_clipboard, set_clipboard};
fn main() {
    let result: String = get_clipboard(formats::Unicode).expect("Couldn't read clipboard");
    let frontslash = result.contains("\\");
    let replace_with = if frontslash { "/" } else { "\\" };
    let new: String;
    if result.contains("\\\\") && frontslash {
        new = result.replace("\\\\", replace_with);
    } else if result.contains("\\") && frontslash {
        new = result.replace("\\", replace_with);
    } else if result.contains("/") && !frontslash {
        new = result.replace("/", replace_with)
    } else {
        new = result;
    }
    set_clipboard(formats::Unicode, new).expect("Couldn't write clipboard");
    println!("fixed the path :>")
}
